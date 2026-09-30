use crate::features::concept_model::RelationKind;
use crate::features::information_model::{
    InformationClass, InformationDataType, PrimitiveType,
};
use crate::features::model::ModelProject;
use std::collections::BTreeMap;
use uuid::Uuid;

fn escape_turtle(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn sanitize_identifier(s: &str) -> String {
    let clean: String = s
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if clean.is_empty() {
        "item".to_string()
    } else {
        clean
    }
}

/// Eksporterer hele projektets informationsmodel til valid W3C SHACL Shapes og OWL Ontology (.shacl.ttl)
/// for maskinel validering af datakontrakter jf. FDA og EU SEMIC standarderne.
pub fn export_to_shacl_turtle(project: &ModelProject) -> String {
    let mut out = String::with_capacity(16 * 1024);
    let meta = project.metadata();

    let base_uri = if meta.uri().trim().is_empty() {
        "https://data.gov.dk/model/core/unspecified"
    } else {
        meta.uri().trim()
    };
    let clean_base = base_uri.trim_end_matches(['/', '#']);
    let ontology_uri = format!("<{}>", clean_base);

    // Standard Præfikser
    out.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    out.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    out.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    out.push_str("@prefix sh: <http://www.w3.org/ns/shacl#> .\n");
    out.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n");
    out.push_str("@prefix dct: <http://purl.org/dc/terms/> .\n\n");

    // Ontologi Hoved
    out.push_str(&format!("{} a owl:Ontology ;\n", ontology_uri));
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
    if !meta.version().trim().is_empty() {
        out.push_str(&format!(
            "    owl:versionInfo \"{}\" ;\n",
            escape_turtle(meta.version())
        ));
    }
    out.push_str("    rdfs:comment \"Genereret af Kant - FDA Begrebs- og Informationsmodellering.\"@da .\n\n");

    let info_model = project.information_model();

    // Enumerations i ontologien
    let mut sorted_enums: Vec<_> = info_model.enumerations().iter().collect();
    sorted_enums.sort_by(|a, b| a.name().cmp(b.name()));
    for en in sorted_enums {
        let enum_slug = sanitize_identifier(en.name());
        let enum_uri = format!("<{}/enums/{}>", clean_base, enum_slug);

        out.push_str(&format!("{} a owl:Class ;\n", enum_uri));
        out.push_str(&format!("    rdfs:label \"{}\"@da ;\n", escape_turtle(en.name())));
        if let Some(def) = en.definition() {
            if !def.trim().is_empty() {
                out.push_str(&format!(
                    "    rdfs:comment \"{}\"@da ;\n",
                    escape_turtle(def)
                ));
            }
        }
        if !en.values().is_empty() {
            let escaped_vals: Vec<String> = en
                .values()
                .iter()
                .map(|v| format!("\"{}\"", escape_turtle(v)))
                .collect();
            out.push_str(&format!("    owl:oneOf ( {} ) ;\n", escaped_vals.join(" ")));
        }
        out.push_str(&format!("    rdfs:isDefinedBy {} .\n\n", ontology_uri));
    }

    // NodeId til InformationClass mapping
    let mut node_to_class: BTreeMap<Uuid, &InformationClass> = BTreeMap::new();
    for node in project.information_graph().nodes() {
        if let Some(cls) = info_model.get_class(node.class_id()) {
            node_to_class.insert(node.id(), cls);
        }
    }

    // Generaliseringer (subClassOf)
    let mut class_parents: BTreeMap<Uuid, Vec<String>> = BTreeMap::new();
    // Associationer/relationer pr. klasse: (rel_slug, target_uri, min_count, max_count)
    type ClassRelationEntry = (String, String, Option<u32>, Option<u32>);
    let mut class_relations: BTreeMap<Uuid, Vec<ClassRelationEntry>> = BTreeMap::new();

    for edge in project.information_graph().edges() {
        let src_cls = node_to_class.get(&edge.from());
        let tgt_cls = node_to_class.get(&edge.to());

        if let (Some(&src), Some(&tgt)) = (src_cls, tgt_cls) {
            match edge.kind() {
                RelationKind::Generalization => {
                    let parent_slug = sanitize_identifier(tgt.name());
                    let parent_uri = format!("<{}/classes/{}>", clean_base, parent_slug);
                    class_parents.entry(src.id()).or_default().push(parent_uri);
                }
                RelationKind::Association | RelationKind::Composition => {
                    let rel_name = edge.label().unwrap_or(tgt.name());
                    let rel_slug = sanitize_identifier(rel_name);
                    let target_slug = sanitize_identifier(tgt.name());
                    let target_uri = format!("<{}/classes/{}>", clean_base, target_slug);
                    let min_count = edge.target_multiplicity().map(|m| m.lower());
                    let max_count = edge.target_multiplicity().and_then(|m| m.upper());
                    class_relations.entry(src.id()).or_default().push((
                        rel_slug,
                        target_uri,
                        min_count,
                        max_count,
                    ));
                }
                _ => {}
            }
        }
    }

    // Klasser (owl:Class og sh:NodeShape)
    let mut sorted_classes: Vec<_> = info_model.classes().iter().collect();
    sorted_classes.sort_by(|a, b| a.name().cmp(b.name()));

    for cls in sorted_classes {
        let class_slug = sanitize_identifier(cls.name());
        let class_uri = format!("<{}/classes/{}>", clean_base, class_slug);

        out.push_str(&format!("{} a owl:Class, sh:NodeShape ;\n", class_uri));
        out.push_str(&format!("    sh:targetClass {} ;\n", class_uri));
        out.push_str(&format!("    rdfs:label \"{}\"@da ;\n", escape_turtle(cls.name())));

        if let Some(desc) = cls.description() {
            if !desc.trim().is_empty() {
                out.push_str(&format!(
                    "    rdfs:comment \"{}\"@da ;\n",
                    escape_turtle(desc)
                ));
            }
        }

        // SubClassOf generalisering
        if let Some(parents) = class_parents.get(&cls.id()) {
            for p in parents {
                out.push_str(&format!("    rdfs:subClassOf {} ;\n", p));
            }
        }

        // Egenskabsbegrænsninger (sh:property)
        for attr in cls.attributes() {
            let attr_slug = sanitize_identifier(attr.name());
            let prop_path = format!("<{}/properties/{}>", clean_base, attr_slug);
            let mult = attr.multiplicity();

            out.push_str("    sh:property [\n");
            out.push_str(&format!("        sh:path {} ;\n", prop_path));
            out.push_str(&format!(
                "        sh:name \"{}\"@da ;\n",
                escape_turtle(attr.name())
            ));

            match attr.data_type() {
                InformationDataType::Primitive(p) => {
                    let xsd_type = match p {
                        PrimitiveType::CharacterString => "xsd:string",
                        PrimitiveType::Integer => "xsd:integer",
                        PrimitiveType::Decimal => "xsd:decimal",
                        PrimitiveType::Boolean => "xsd:boolean",
                        PrimitiveType::Date => "xsd:date",
                        PrimitiveType::DateTime => "xsd:dateTime",
                        PrimitiveType::Time => "xsd:time",
                        PrimitiveType::Uri => "xsd:anyURI",
                    };
                    out.push_str(&format!("        sh:datatype {} ;\n", xsd_type));
                }
                InformationDataType::Enumeration { enumeration_id } => {
                    if let Some(en) = info_model.get_enumeration(*enumeration_id) {
                        let val_strs: Vec<String> = en
                            .values()
                            .iter()
                            .map(|v| format!("\"{}\"", escape_turtle(v)))
                            .collect();
                        out.push_str(&format!("        sh:in ( {} ) ;\n", val_strs.join(" ")));
                    }
                }
                InformationDataType::Structured { structured_id } => {
                    if let Some(st) = info_model.get_structured_type(*structured_id) {
                        let st_slug = sanitize_identifier(st.name());
                        out.push_str(&format!(
                            "        sh:class <{}/types/{}> ;\n",
                            clean_base, st_slug
                        ));
                    }
                }
            }

            if mult.lower() > 0 {
                out.push_str(&format!("        sh:minCount {} ;\n", mult.lower()));
            }
            if let Some(u) = mult.upper() {
                out.push_str(&format!("        sh:maxCount {} ;\n", u));
            }
            out.push_str("    ] ;\n");
        }

        // Relationer som sh:property med sh:class
        if let Some(rels) = class_relations.get(&cls.id()) {
            for (rel_slug, target_class_uri, min_count, max_count) in rels {
                out.push_str("    sh:property [\n");
                out.push_str(&format!(
                    "        sh:path <{}/relations/{}> ;\n",
                    clean_base, rel_slug
                ));
                out.push_str(&format!("        sh:class {} ;\n", target_class_uri));
                if let Some(min) = min_count {
                    if *min > 0 {
                        out.push_str(&format!("        sh:minCount {} ;\n", min));
                    }
                }
                if let Some(max) = max_count {
                    out.push_str(&format!("        sh:maxCount {} ;\n", max));
                }
                out.push_str("    ] ;\n");
            }
        }

        out.push_str(&format!("    rdfs:isDefinedBy {} .\n\n", ontology_uri));
    }

    out
}
