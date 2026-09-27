use crate::features::concept_model::{DiagramEdge, DiagramNode};
use crate::features::concepts::Concept;
use crate::features::information_model::{
    Attribute, ClassDiagramEdge, ClassDiagramNode, InformationClass,
};
use crate::features::model::{ModelMetadata, ModelProject};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictEntityKind {
    Concept(Uuid),
    Class(Uuid),
    Relation(Uuid),
    Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelConflict {
    pub entity_kind: ConflictEntityKind,
    pub entity_name: String,
    pub field_name: String,
    pub base_value: String,
    pub our_value: String,
    pub their_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MergeChangeSummary {
    pub concepts_added: Vec<String>,
    pub concepts_updated: Vec<String>,
    pub concepts_deleted: Vec<String>,
    pub classes_added: Vec<String>,
    pub classes_updated: Vec<String>,
    pub classes_deleted: Vec<String>,
    pub relations_added: Vec<String>,
    pub relations_updated: Vec<String>,
    pub relations_deleted: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeOutcome {
    pub merged_project: ModelProject,
    pub conflicts: Vec<ModelConflict>,
    pub summary: MergeChangeSummary,
    pub warnings: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
fn merge_field<T: PartialEq + Clone>(
    base: &T,
    ours: &T,
    theirs: &T,
    entity_kind: ConflictEntityKind,
    entity_name: &str,
    field_name: &str,
    format_fn: impl Fn(&T) -> String,
    conflicts: &mut Vec<ModelConflict>,
) -> T {
    if ours == theirs {
        ours.clone()
    } else if ours == base {
        theirs.clone()
    } else if theirs == base {
        ours.clone()
    } else {
        conflicts.push(ModelConflict {
            entity_kind,
            entity_name: entity_name.to_string(),
            field_name: field_name.to_string(),
            base_value: format_fn(base),
            our_value: format_fn(ours),
            their_value: format_fn(theirs),
        });
        ours.clone()
    }
}

fn opt_str_fmt(s: &Option<String>) -> String {
    s.as_deref().unwrap_or("<tom>").to_string()
}

fn str_fmt(s: &str) -> String {
    s.to_string()
}

/// Udfører en semantisk 3-vejs merge af to divergerende modelversioner (ours og theirs)
/// med udgangspunkt i en fælles forfader (base).
pub fn merge_models(
    base: &ModelProject,
    ours: &ModelProject,
    theirs: &ModelProject,
) -> MergeOutcome {
    let mut conflicts = Vec::new();
    let mut warnings = Vec::new();
    let mut summary = MergeChangeSummary::default();

    // 1. Merge Metadata
    let base_meta = base.metadata();
    let our_meta = ours.metadata();
    let their_meta = theirs.metadata();

    let meta_kind = ConflictEntityKind::Metadata;
    let meta_name = "Model Metadata";

    let name = merge_field(
        &base_meta.name().to_string(),
        &our_meta.name().to_string(),
        &their_meta.name().to_string(),
        meta_kind.clone(),
        meta_name,
        "name",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let description = merge_field(
        &base_meta.description().to_string(),
        &our_meta.description().to_string(),
        &their_meta.description().to_string(),
        meta_kind.clone(),
        meta_name,
        "description",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let uri = merge_field(
        &base_meta.uri().to_string(),
        &our_meta.uri().to_string(),
        &their_meta.uri().to_string(),
        meta_kind.clone(),
        meta_name,
        "uri",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let responsible_org = merge_field(
        &base_meta.responsible_org().to_string(),
        &our_meta.responsible_org().to_string(),
        &their_meta.responsible_org().to_string(),
        meta_kind.clone(),
        meta_name,
        "responsible_org",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let domain_area = merge_field(
        &base_meta.domain_area().to_string(),
        &our_meta.domain_area().to_string(),
        &their_meta.domain_area().to_string(),
        meta_kind.clone(),
        meta_name,
        "domain_area",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let version = merge_field(
        &base_meta.version().to_string(),
        &our_meta.version().to_string(),
        &their_meta.version().to_string(),
        meta_kind.clone(),
        meta_name,
        "version",
        |s| str_fmt(s),
        &mut conflicts,
    );

    let status = merge_field(
        &base_meta.status(),
        &our_meta.status(),
        &their_meta.status(),
        meta_kind.clone(),
        meta_name,
        "status",
        |s| format!("{}", s),
        &mut conflicts,
    );

    let legal_source_base = base_meta.legal_source().map(|s| s.to_string());
    let legal_source_ours = our_meta.legal_source().map(|s| s.to_string());
    let legal_source_theirs = their_meta.legal_source().map(|s| s.to_string());

    let legal_source = merge_field(
        &legal_source_base,
        &legal_source_ours,
        &legal_source_theirs,
        meta_kind,
        meta_name,
        "legal_source",
        opt_str_fmt,
        &mut conflicts,
    );

    let mut merged_meta = ModelMetadata::new(
        name,
        description,
        uri,
        responsible_org,
        domain_area,
        version,
        status,
    );
    merged_meta.set_legal_source(legal_source);

    let mut merged_project = ModelProject::new(merged_meta);

    // 2. Merge Concepts
    let base_concepts: HashMap<Uuid, &Concept> =
        base.concepts().iter().map(|c| (c.id(), c)).collect();
    let our_concepts: HashMap<Uuid, &Concept> =
        ours.concepts().iter().map(|c| (c.id(), c)).collect();
    let their_concepts: HashMap<Uuid, &Concept> =
        theirs.concepts().iter().map(|c| (c.id(), c)).collect();

    let all_concept_ids: HashSet<Uuid> = base_concepts
        .keys()
        .chain(our_concepts.keys())
        .chain(their_concepts.keys())
        .copied()
        .collect();

    for id in all_concept_ids {
        let b = base_concepts.get(&id).copied();
        let o = our_concepts.get(&id).copied();
        let t = their_concepts.get(&id).copied();

        match (b, o, t) {
            // Kun i ours (tilføjet lokalt)
            (None, Some(c_our), None) => {
                summary
                    .concepts_added
                    .push(c_our.preferred_term().to_string());
                merged_project.concepts_mut().push(c_our.clone());
            }
            // Kun i theirs (tilføjet på serveren)
            (None, None, Some(c_their)) => {
                summary
                    .concepts_added
                    .push(c_their.preferred_term().to_string());
                merged_project.concepts_mut().push(c_their.clone());
            }
            // Begge har oprettet samme ID uafhængigt
            (None, Some(c_our), Some(c_their)) => {
                let merged_c = merge_concept_instances(c_our, c_our, c_their, &mut conflicts);
                merged_project.concepts_mut().push(merged_c);
            }
            // Slettet i ours
            (Some(c_base), None, t_opt) => {
                match t_opt {
                    None => {
                        // Slettet af begge
                        summary
                            .concepts_deleted
                            .push(c_base.preferred_term().to_string());
                    }
                    Some(c_their) => {
                        if c_their == c_base {
                            // Serveren rørte det ikke, vi slettede det
                            summary
                                .concepts_deleted
                                .push(c_base.preferred_term().to_string());
                        } else {
                            // Slettet lokalt, men ændret på server -> bevar serverversion med advarsel
                            warnings.push(format!(
                                "Begrebet '{}' blev slettet lokalt, men ændret på serveren. Begrebet er bevaret.",
                                c_their.preferred_term()
                            ));
                            merged_project.concepts_mut().push(c_their.clone());
                        }
                    }
                }
            }
            // Slettet i theirs
            (Some(c_base), Some(c_our), None) => {
                if c_our == c_base {
                    // Vi rørte det ikke, serveren slettede det
                    summary
                        .concepts_deleted
                        .push(c_base.preferred_term().to_string());
                } else {
                    // Ændret lokalt, slettet på server -> bevar lokalversion med advarsel
                    warnings.push(format!(
                        "Begrebet '{}' blev slettet på serveren, men ændret lokalt. Lokale ændringer er bevaret.",
                        c_our.preferred_term()
                    ));
                    merged_project.concepts_mut().push(c_our.clone());
                }
            }
            // Eksisterer i alle tre
            (Some(c_base), Some(c_our), Some(c_their)) => {
                if c_our == c_their {
                    merged_project.concepts_mut().push(c_our.clone());
                } else {
                    let merged_c = merge_concept_instances(c_base, c_our, c_their, &mut conflicts);
                    summary
                        .concepts_updated
                        .push(merged_c.preferred_term().to_string());
                    merged_project.concepts_mut().push(merged_c);
                }
            }
            (None, None, None) => {}
        }
    }
    merged_project.sort_concepts_alphabetically();

    // 3. Merge Classes
    let base_classes: HashMap<Uuid, &InformationClass> = base
        .information_model()
        .classes()
        .iter()
        .map(|c| (c.id(), c))
        .collect();
    let our_classes: HashMap<Uuid, &InformationClass> = ours
        .information_model()
        .classes()
        .iter()
        .map(|c| (c.id(), c))
        .collect();
    let their_classes: HashMap<Uuid, &InformationClass> = theirs
        .information_model()
        .classes()
        .iter()
        .map(|c| (c.id(), c))
        .collect();

    let all_class_ids: HashSet<Uuid> = base_classes
        .keys()
        .chain(our_classes.keys())
        .chain(their_classes.keys())
        .copied()
        .collect();

    for id in all_class_ids {
        let b = base_classes.get(&id).copied();
        let o = our_classes.get(&id).copied();
        let t = their_classes.get(&id).copied();

        match (b, o, t) {
            (None, Some(cls_our), None) => {
                summary.classes_added.push(cls_our.name().to_string());
                merged_project
                    .information_model_mut()
                    .add_class(cls_our.clone());
            }
            (None, None, Some(cls_their)) => {
                summary.classes_added.push(cls_their.name().to_string());
                merged_project
                    .information_model_mut()
                    .add_class(cls_their.clone());
            }
            (None, Some(cls_our), Some(cls_their)) => {
                let merged_cls = merge_class_instances(cls_our, cls_our, cls_their, &mut conflicts);
                merged_project.information_model_mut().add_class(merged_cls);
            }
            (Some(cls_base), None, t_opt) => match t_opt {
                None => {
                    summary.classes_deleted.push(cls_base.name().to_string());
                }
                Some(cls_their) => {
                    if cls_their == cls_base {
                        summary.classes_deleted.push(cls_base.name().to_string());
                    } else {
                        warnings.push(format!(
                                "Klassen '{}' blev slettet lokalt, men ændret på serveren. Klassen er bevaret.",
                                cls_their.name()
                            ));
                        merged_project
                            .information_model_mut()
                            .add_class(cls_their.clone());
                    }
                }
            },
            (Some(cls_base), Some(cls_our), None) => {
                if cls_our == cls_base {
                    summary.classes_deleted.push(cls_base.name().to_string());
                } else {
                    warnings.push(format!(
                        "Klassen '{}' blev slettet på serveren, men ændret lokalt. Klassen er bevaret.",
                        cls_our.name()
                    ));
                    merged_project
                        .information_model_mut()
                        .add_class(cls_our.clone());
                }
            }
            (Some(cls_base), Some(cls_our), Some(cls_their)) => {
                if cls_our == cls_their {
                    merged_project
                        .information_model_mut()
                        .add_class(cls_our.clone());
                } else {
                    let merged_cls =
                        merge_class_instances(cls_base, cls_our, cls_their, &mut conflicts);
                    summary.classes_updated.push(merged_cls.name().to_string());
                    merged_project.information_model_mut().add_class(merged_cls);
                }
            }
            (None, None, None) => {}
        }
    }

    // 4. Merge Concept Relations (Edges)
    merge_concept_edges(base, ours, theirs, &mut merged_project);

    // 5. Merge Class Relations (Edges)
    merge_class_edges(base, ours, theirs, &mut merged_project);

    // 6. Merge Diagram Node Positions
    merge_diagram_nodes(base, ours, theirs, &mut merged_project);

    // 7. Synkroniser grafer
    merged_project.sync_concept_graph();
    merged_project.sync_information_graph();

    MergeOutcome {
        merged_project,
        conflicts,
        summary,
        warnings,
    }
}

fn merge_concept_instances(
    base: &Concept,
    ours: &Concept,
    theirs: &Concept,
    conflicts: &mut Vec<ModelConflict>,
) -> Concept {
    let kind = ConflictEntityKind::Concept(ours.id());
    let entity_name = ours.preferred_term();

    let term = merge_field(
        &base.preferred_term().to_string(),
        &ours.preferred_term().to_string(),
        &theirs.preferred_term().to_string(),
        kind.clone(),
        entity_name,
        "preferred_term",
        |s| str_fmt(s),
        conflicts,
    );

    let def = merge_field(
        &base.definition().to_string(),
        &ours.definition().to_string(),
        &theirs.definition().to_string(),
        kind.clone(),
        entity_name,
        "definition",
        |s| str_fmt(s),
        conflicts,
    );

    let belongs = merge_field(
        &base.belongs_to_domain(),
        &ours.belongs_to_domain(),
        &theirs.belongs_to_domain(),
        kind.clone(),
        entity_name,
        "belongs_to_domain",
        |b| format!("{:?}", b),
        conflicts,
    );

    let mut merged = Concept::new_with_id(ours.id(), term, def, belongs.clone());

    let opt_fields = [
        (
            "accepted_term",
            base.accepted_term().map(str::to_string),
            ours.accepted_term().map(str::to_string),
            theirs.accepted_term().map(str::to_string),
        ),
        (
            "deprecated_term",
            base.deprecated_term().map(str::to_string),
            ours.deprecated_term().map(str::to_string),
            theirs.deprecated_term().map(str::to_string),
        ),
        (
            "example",
            base.example().map(str::to_string),
            ours.example().map(str::to_string),
            theirs.example().map(str::to_string),
        ),
        (
            "comment",
            base.comment().map(str::to_string),
            ours.comment().map(str::to_string),
            theirs.comment().map(str::to_string),
        ),
        (
            "application_note",
            base.application_note().map(str::to_string),
            ours.application_note().map(str::to_string),
            theirs.application_note().map(str::to_string),
        ),
        (
            "legal_source",
            base.legal_source().map(str::to_string),
            ours.legal_source().map(str::to_string),
            theirs.legal_source().map(str::to_string),
        ),
        (
            "source",
            base.source().map(str::to_string),
            ours.source().map(str::to_string),
            theirs.source().map(str::to_string),
        ),
        (
            "identifier",
            base.identifier().map(str::to_string),
            ours.identifier().map(str::to_string),
            theirs.identifier().map(str::to_string),
        ),
        (
            "derived_from",
            base.derived_from().map(str::to_string),
            ours.derived_from().map(str::to_string),
            theirs.derived_from().map(str::to_string),
        ),
    ];

    for (field_name, b_val, o_val, t_val) in opt_fields {
        let res = merge_field(
            &b_val,
            &o_val,
            &t_val,
            kind.clone(),
            entity_name,
            field_name,
            opt_str_fmt,
            conflicts,
        );
        match field_name {
            "accepted_term" => merged.set_accepted_term(res),
            "deprecated_term" => merged.set_deprecated_term(res),
            "example" => merged.set_example(res),
            "comment" => merged.set_comment(res),
            "application_note" => merged.set_application_note(res),
            "legal_source" => merged.set_legal_source(res),
            "source" => merged.set_source(res),
            "identifier" => merged.set_identifier(res),
            "derived_from" => merged.set_derived_from(res),
            _ => {}
        }
    }

    merged
}

fn merge_class_instances(
    base: &InformationClass,
    ours: &InformationClass,
    theirs: &InformationClass,
    conflicts: &mut Vec<ModelConflict>,
) -> InformationClass {
    let kind = ConflictEntityKind::Class(ours.id());
    let entity_name = ours.name();

    let name = merge_field(
        &base.name().to_string(),
        &ours.name().to_string(),
        &theirs.name().to_string(),
        kind.clone(),
        entity_name,
        "name",
        |s| str_fmt(s),
        conflicts,
    );

    let desc = merge_field(
        &base.description().map(str::to_string),
        &ours.description().map(str::to_string),
        &theirs.description().map(str::to_string),
        kind.clone(),
        entity_name,
        "description",
        opt_str_fmt,
        conflicts,
    );

    let is_abstract = merge_field(
        &base.is_abstract(),
        &ours.is_abstract(),
        &theirs.is_abstract(),
        kind.clone(),
        entity_name,
        "is_abstract",
        |b| format!("{}", b),
        conflicts,
    );

    let is_local = merge_field(
        &base.is_local(),
        &ours.is_local(),
        &theirs.is_local(),
        kind.clone(),
        entity_name,
        "is_local",
        |b| format!("{}", b),
        conflicts,
    );

    let origin = merge_field(
        &base.origin_model().map(str::to_string),
        &ours.origin_model().map(str::to_string),
        &theirs.origin_model().map(str::to_string),
        kind.clone(),
        entity_name,
        "origin_model",
        opt_str_fmt,
        conflicts,
    );

    let mut merged = InformationClass::new_with_id(ours.id(), name);
    merged.set_description(desc);
    merged.set_abstract(is_abstract);
    merged.set_local(is_local);
    merged.set_origin_model(origin);

    // Concept IDs (Union)
    let mut c_ids = HashSet::new();
    c_ids.extend(ours.concept_ids());
    c_ids.extend(theirs.concept_ids());
    for cid in c_ids {
        merged.add_concept_id(cid);
    }

    // Attributes (3-way merge on attributes)
    let base_attrs: HashMap<Uuid, &Attribute> =
        base.attributes().iter().map(|a| (a.id(), a)).collect();
    let our_attrs: HashMap<Uuid, &Attribute> =
        ours.attributes().iter().map(|a| (a.id(), a)).collect();
    let their_attrs: HashMap<Uuid, &Attribute> =
        theirs.attributes().iter().map(|a| (a.id(), a)).collect();

    let all_attr_ids: HashSet<Uuid> = base_attrs
        .keys()
        .chain(our_attrs.keys())
        .chain(their_attrs.keys())
        .copied()
        .collect();

    for aid in all_attr_ids {
        let b = base_attrs.get(&aid).copied();
        let o = our_attrs.get(&aid).copied();
        let t = their_attrs.get(&aid).copied();

        match (b, o, t) {
            (None, Some(a_our), None) => merged.add_attribute(a_our.clone()),
            (None, None, Some(a_their)) => merged.add_attribute(a_their.clone()),
            (None, Some(a_our), Some(_)) => {
                merged.add_attribute(a_our.clone());
            }
            (Some(a_base), None, t_opt) => {
                if let Some(a_their) = t_opt {
                    if a_their != a_base {
                        merged.add_attribute(a_their.clone());
                    }
                }
            }
            (Some(a_base), Some(a_our), None) => {
                if a_our != a_base {
                    merged.add_attribute(a_our.clone());
                }
            }
            (Some(_), Some(a_our), Some(_)) => {
                merged.add_attribute(a_our.clone());
            }
            (None, None, None) => {}
        }
    }

    merged
}

fn merge_concept_edges(
    base: &ModelProject,
    ours: &ModelProject,
    theirs: &ModelProject,
    merged: &mut ModelProject,
) {
    let valid_cids: HashSet<Uuid> = merged.concepts().iter().map(|c| c.id()).collect();

    let base_edges: Vec<_> = base.concept_graph().edges().to_vec();
    let our_edges: Vec<_> = ours.concept_graph().edges().to_vec();
    let their_edges: Vec<_> = theirs.concept_graph().edges().to_vec();

    let mut result_edges: Vec<DiagramEdge> = Vec::new();

    for edge in our_edges {
        if valid_cids.contains(&edge.from())
            && valid_cids.contains(&edge.to())
            && !result_edges
                .iter()
                .any(|e| e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind())
        {
            result_edges.push(edge);
        }
    }

    for edge in their_edges {
        if valid_cids.contains(&edge.from()) && valid_cids.contains(&edge.to()) {
            let was_in_base = base_edges
                .iter()
                .any(|e| e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind());
            if !was_in_base
                && !result_edges.iter().any(|e| {
                    e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind()
                })
            {
                result_edges.push(edge);
            }
        }
    }

    *merged.concept_graph_mut().edges_mut() = result_edges;
}

fn merge_class_edges(
    base: &ModelProject,
    ours: &ModelProject,
    theirs: &ModelProject,
    merged: &mut ModelProject,
) {
    let valid_class_ids: HashSet<Uuid> = merged
        .information_model()
        .classes()
        .iter()
        .map(|c| c.id())
        .collect();

    let base_edges: Vec<_> = base.information_graph().edges().to_vec();
    let our_edges: Vec<_> = ours.information_graph().edges().to_vec();
    let their_edges: Vec<_> = theirs.information_graph().edges().to_vec();

    let mut result_edges: Vec<ClassDiagramEdge> = Vec::new();

    for edge in our_edges {
        if valid_class_ids.contains(&edge.from())
            && valid_class_ids.contains(&edge.to())
            && !result_edges
                .iter()
                .any(|e| e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind())
        {
            result_edges.push(edge);
        }
    }

    for edge in their_edges {
        if valid_class_ids.contains(&edge.from()) && valid_class_ids.contains(&edge.to()) {
            let was_in_base = base_edges
                .iter()
                .any(|e| e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind());
            if !was_in_base
                && !result_edges.iter().any(|e| {
                    e.from() == edge.from() && e.to() == edge.to() && e.kind() == edge.kind()
                })
            {
                result_edges.push(edge);
            }
        }
    }

    *merged.information_graph_mut().edges_mut() = result_edges;
}

fn merge_diagram_nodes(
    _base: &ModelProject,
    ours: &ModelProject,
    theirs: &ModelProject,
    merged: &mut ModelProject,
) {
    // For concept nodes: bevar ours koordinater hvis de findes, ellers theirs
    let our_nodes: HashMap<Uuid, &DiagramNode> = ours
        .concept_graph()
        .nodes()
        .iter()
        .map(|n| (n.id(), n))
        .collect();
    let their_nodes: HashMap<Uuid, &DiagramNode> = theirs
        .concept_graph()
        .nodes()
        .iter()
        .map(|n| (n.id(), n))
        .collect();

    let mut merged_nodes = Vec::new();
    for concept in merged.concepts() {
        if let Some(node) = our_nodes.get(&concept.id()) {
            merged_nodes.push((*node).clone());
        } else if let Some(node) = their_nodes.get(&concept.id()) {
            merged_nodes.push((*node).clone());
        }
    }
    *merged.concept_graph_mut().nodes_mut() = merged_nodes;

    // For class nodes: bevar ours koordinater hvis de findes, ellers theirs
    let our_class_nodes: HashMap<Uuid, &ClassDiagramNode> = ours
        .information_graph()
        .nodes()
        .iter()
        .map(|n| (n.class_id(), n))
        .collect();
    let their_class_nodes: HashMap<Uuid, &ClassDiagramNode> = theirs
        .information_graph()
        .nodes()
        .iter()
        .map(|n| (n.class_id(), n))
        .collect();

    let mut merged_class_nodes = Vec::new();
    for class in merged.information_model().classes() {
        if let Some(node) = our_class_nodes.get(&class.id()) {
            merged_class_nodes.push((*node).clone());
        } else if let Some(node) = their_class_nodes.get(&class.id()) {
            merged_class_nodes.push((*node).clone());
        }
    }
    *merged.information_graph_mut().nodes_mut() = merged_class_nodes;
}
