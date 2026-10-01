use crate::features::concept_model::RelationKind;
use crate::features::concepts::Concept;
use crate::features::model::ModelProject;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

fn escape_turtle(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn resolve_concept_uri(concept: &Concept, base_uri: &str) -> String {
    if let Some(id_uri) = concept.identifier() {
        let trimmed = id_uri.trim();
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return format!("<{}>", trimmed);
        }
    }
    let base = base_uri.trim_end_matches(['/', '#']);
    format!("<{}/concepts/{}>", base, concept.id())
}

/// Eksporterer projektets begrebsliste og begrebsgraf til valid W3C SKOS RDF/Turtle (.ttl)
/// struktureret som et skos:ConceptScheme for integration i det fællesoffentlige Modelkatalog.
pub fn export_to_skos_turtle(project: &ModelProject) -> String {
    let mut out = String::with_capacity(16 * 1024);
    let meta = project.metadata();

    let base_uri = if meta.uri().trim().is_empty() {
        "https://data.gov.dk/model/core/unspecified"
    } else {
        meta.uri().trim()
    };
    let scheme_uri = format!("<{}>", base_uri.trim_end_matches(['/', '#']));

    // Standard Præfikser
    out.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    out.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    out.push_str("@prefix skos: <http://www.w3.org/2004/02/skos/core#> .\n");
    out.push_str("@prefix dct: <http://purl.org/dc/terms/> .\n");
    out.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    out.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n");

    // Mapping mellem DiagramNode Id og Concept Id
    let mut node_to_concept: BTreeMap<Uuid, Uuid> = BTreeMap::new();
    for node in project.concept_graph().nodes() {
        node_to_concept.insert(node.id(), node.concept_id());
    }

    // Beregn relationer (broader, narrower, related) for hvert begreb
    let mut broader_map: BTreeMap<Uuid, BTreeSet<Uuid>> = BTreeMap::new();
    let mut narrower_map: BTreeMap<Uuid, BTreeSet<Uuid>> = BTreeMap::new();
    let mut related_map: BTreeMap<Uuid, BTreeSet<Uuid>> = BTreeMap::new();

    for edge in project.concept_graph().edges() {
        let from_concept = node_to_concept.get(&edge.from());
        let to_concept = node_to_concept.get(&edge.to());

        if let (Some(&child_id), Some(&parent_id)) = (from_concept, to_concept) {
            match edge.kind() {
                RelationKind::Generalization => {
                    // Generalisering: from (barn) er mere specifikt end to (forælder)
                    broader_map.entry(child_id).or_default().insert(parent_id);
                    narrower_map.entry(parent_id).or_default().insert(child_id);
                }
                RelationKind::Association => {
                    related_map.entry(child_id).or_default().insert(parent_id);
                    related_map.entry(parent_id).or_default().insert(child_id);
                }
                _ => {}
            }
        }
    }

    // Find top-level begreber (begreber der ikke har nogen overordnede 'broader' begreber)
    let mut top_concepts: Vec<&Concept> = Vec::new();
    for c in project.concepts() {
        if broader_map.get(&c.id()).is_none_or(|set| set.is_empty()) {
            top_concepts.push(c);
        }
    }

    // skos:ConceptScheme blok
    out.push_str(&format!("{} a skos:ConceptScheme ;\n", scheme_uri));
    out.push_str(&format!(
        "    dct:title \"{}\"@da ;\n",
        escape_turtle(meta.name())
    ));

    if !meta.description().trim().is_empty() {
        out.push_str(&format!(
            "    dct:description \"{}\"@da ;\n",
            escape_turtle(meta.description())
        ));
    }
    if !meta.responsible_org().trim().is_empty() {
        out.push_str(&format!(
            "    dct:publisher \"{}\" ;\n",
            escape_turtle(meta.responsible_org())
        ));
    }
    if !meta.date_modified().trim().is_empty() {
        out.push_str(&format!(
            "    dct:modified \"{}\"^^xsd:date ;\n",
            escape_turtle(meta.date_modified())
        ));
    }
    if !meta.version().trim().is_empty() {
        out.push_str(&format!(
            "    owl:versionInfo \"{}\" ;\n",
            escape_turtle(meta.version())
        ));
    }

    if !top_concepts.is_empty() {
        let top_uris: Vec<String> = top_concepts
            .iter()
            .map(|tc| resolve_concept_uri(tc, base_uri))
            .collect();
        out.push_str(&format!(
            "    skos:hasTopConcept {} ;\n",
            top_uris.join(", ")
        ));
    }
    out.push_str("    rdfs:isDefinedBy ");
    out.push_str(&scheme_uri);
    out.push_str(" .\n\n");

    // Sorter begreber deterministisk efter foretrukken term
    let mut sorted_concepts: Vec<_> = project.concepts().iter().collect();
    sorted_concepts.sort_by(|a, b| a.preferred_term().cmp(b.preferred_term()));

    // Map over alle begrebers URI for nem opslag
    let concept_by_id: BTreeMap<Uuid, &Concept> =
        project.concepts().iter().map(|c| (c.id(), c)).collect();

    // Hvert skos:Concept
    for c in sorted_concepts {
        let c_uri = resolve_concept_uri(c, base_uri);
        let is_top = broader_map.get(&c.id()).is_none_or(|set| set.is_empty());

        out.push_str(&format!("{} a skos:Concept ;\n", c_uri));
        out.push_str(&format!("    skos:inScheme {} ;\n", scheme_uri));
        if is_top {
            out.push_str(&format!("    skos:topConceptOf {} ;\n", scheme_uri));
        }

        out.push_str(&format!(
            "    skos:prefLabel \"{}\"@da ;\n",
            escape_turtle(c.preferred_term())
        ));

        if let Some(alt) = c.accepted_term() {
            if !alt.trim().is_empty() {
                out.push_str(&format!(
                    "    skos:altLabel \"{}\"@da ;\n",
                    escape_turtle(alt)
                ));
            }
        }

        if let Some(hidden) = c.deprecated_term() {
            if !hidden.trim().is_empty() {
                out.push_str(&format!(
                    "    skos:hiddenLabel \"{}\"@da ;\n",
                    escape_turtle(hidden)
                ));
            }
        }

        if !c.definition().trim().is_empty() {
            out.push_str(&format!(
                "    skos:definition \"{}\"@da ;\n",
                escape_turtle(c.definition())
            ));
        }

        if let Some(ex) = c.example() {
            if !ex.trim().is_empty() {
                out.push_str(&format!(
                    "    skos:example \"{}\"@da ;\n",
                    escape_turtle(ex)
                ));
            }
        }

        if let Some(note) = c.application_note() {
            if !note.trim().is_empty() {
                out.push_str(&format!(
                    "    skos:scopeNote \"{}\"@da ;\n",
                    escape_turtle(note)
                ));
            }
        }

        if let Some(comm) = c.comment() {
            if !comm.trim().is_empty() {
                out.push_str(&format!(
                    "    rdfs:comment \"{}\"@da ;\n",
                    escape_turtle(comm)
                ));
            }
        }

        if let Some(src) = c.source() {
            if !src.trim().is_empty() {
                out.push_str(&format!("    dct:source \"{}\" ;\n", escape_turtle(src)));
            }
        } else if let Some(lsrc) = c.legal_source() {
            if !lsrc.trim().is_empty() {
                out.push_str(&format!("    dct:source \"{}\" ;\n", escape_turtle(lsrc)));
            }
        }

        // Broader
        if let Some(broaders) = broader_map.get(&c.id()) {
            for b_id in broaders {
                if let Some(target) = concept_by_id.get(b_id) {
                    out.push_str(&format!(
                        "    skos:broader {} ;\n",
                        resolve_concept_uri(target, base_uri)
                    ));
                }
            }
        }

        // Narrower
        if let Some(narrowers) = narrower_map.get(&c.id()) {
            for n_id in narrowers {
                if let Some(target) = concept_by_id.get(n_id) {
                    out.push_str(&format!(
                        "    skos:narrower {} ;\n",
                        resolve_concept_uri(target, base_uri)
                    ));
                }
            }
        }

        // Related
        if let Some(relateds) = related_map.get(&c.id()) {
            for r_id in relateds {
                if let Some(target) = concept_by_id.get(r_id) {
                    out.push_str(&format!(
                        "    skos:related {} ;\n",
                        resolve_concept_uri(target, base_uri)
                    ));
                }
            }
        }

        out.push_str(&format!("    rdfs:isDefinedBy {} .\n\n", scheme_uri));
    }

    out
}
