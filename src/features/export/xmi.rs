use crate::features::concept_model::RelationKind;
use crate::features::information_model::{InformationDataType, PrimitiveType};
use crate::features::model::ModelProject;
use std::collections::BTreeMap;
use uuid::Uuid;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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

/// Eksporterer hele projektets informationsmodel til valid OMG UML 2.1 / XMI 2.1
/// formateret til direkte import i Sparx Enterprise Architect og andre UML værktøjer.
pub fn export_to_xmi_2_1(project: &ModelProject) -> String {
    let mut out = String::with_capacity(16 * 1024);
    let meta = project.metadata();
    let model_name = escape_xml(meta.name());
    let model_desc = escape_xml(meta.description());

    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<xmi:XMI xmi:version=\"2.1\" xmlns:uml=\"http://schema.omg.org/spec/UML/2.1\" xmlns:xmi=\"http://schema.omg.org/spec/XMI/2.1\">\n");
    out.push_str("  <xmi:Documentation exporter=\"Kant\" exporterVersion=\"0.3.2\"/>\n");

    let model_slug = sanitize_identifier(&meta.name().replace(' ', "_").to_lowercase());
    let model_id = format!("model_{}", model_slug);
    out.push_str(&format!(
        "  <uml:Model xmi:type=\"uml:Model\" xmi:id=\"{}\" name=\"{}\">\n",
        model_id, model_name
    ));

    if !model_desc.is_empty() {
        out.push_str(&format!(
            "    <ownedComment xmi:type=\"uml:Comment\" xmi:id=\"comment_{}\">\n      <body>{}</body>\n    </ownedComment>\n",
            model_id, model_desc
        ));
    }

    // Informationsmodel Pakke
    out.push_str("    <packagedElement xmi:type=\"uml:Package\" xmi:id=\"pkg_information_model\" name=\"Informationsmodel\">\n");

    // Standard Primitive Types
    for prim in PrimitiveType::ALL {
        let prim_id = format!("prim_{}", prim.as_str());
        out.push_str(&format!(
            "      <packagedElement xmi:type=\"uml:PrimitiveType\" xmi:id=\"{}\" name=\"{}\"/>\n",
            prim_id,
            prim.as_str()
        ));
    }

    let info_model = project.information_model();

    // Enumerations
    let mut sorted_enums: Vec<_> = info_model.enumerations().iter().collect();
    sorted_enums.sort_by(|a, b| a.name().cmp(b.name()));
    for en in sorted_enums {
        let enum_id = format!("enum_{}", en.id());
        let enum_name = escape_xml(en.name());
        out.push_str(&format!(
            "      <packagedElement xmi:type=\"uml:Enumeration\" xmi:id=\"{}\" name=\"{}\">\n",
            enum_id, enum_name
        ));
        if let Some(def) = en.definition() {
            if !def.is_empty() {
                out.push_str(&format!(
                    "        <ownedComment xmi:type=\"uml:Comment\" xmi:id=\"comment_{}\">\n          <body>{}</body>\n        </ownedComment>\n",
                    enum_id, escape_xml(def)
                ));
            }
        }
        for (idx, val) in en.values().iter().enumerate() {
            let lit_id = format!("lit_{}_{}", en.id(), idx);
            out.push_str(&format!(
                "        <ownedLiteral xmi:type=\"uml:EnumerationLiteral\" xmi:id=\"{}\" name=\"{}\"/>\n",
                lit_id, escape_xml(val)
            ));
        }
        out.push_str("      </packagedElement>\n");
    }

    // Structured Data Types
    let mut sorted_structs: Vec<_> = info_model.structured_types().iter().collect();
    sorted_structs.sort_by(|a, b| a.name().cmp(b.name()));
    for st in sorted_structs {
        let struct_id = format!("datatype_{}", st.id());
        let struct_name = escape_xml(st.name());
        out.push_str(&format!(
            "      <packagedElement xmi:type=\"uml:DataType\" xmi:id=\"{}\" name=\"{}\">\n",
            struct_id, struct_name
        ));
        for attr in st.attributes() {
            let attr_id = format!("attr_{}", attr.id());
            let attr_name = escape_xml(attr.name());
            let type_id = match attr.data_type() {
                InformationDataType::Primitive(p) => format!("prim_{}", p.as_str()),
                InformationDataType::Enumeration { enumeration_id } => format!("enum_{}", enumeration_id),
                InformationDataType::Structured { structured_id } => format!("datatype_{}", structured_id),
            };
            out.push_str(&format!(
                "        <ownedAttribute xmi:type=\"uml:Property\" xmi:id=\"{}\" name=\"{}\" type=\"{}\" visibility=\"public\">\n",
                attr_id, attr_name, type_id
            ));
            let mult = attr.multiplicity();
            let upper_str = match mult.upper() {
                Some(u) => u.to_string(),
                None => "*".to_string(),
            };
            out.push_str(&format!(
                "          <lowerValue xmi:type=\"uml:LiteralInteger\" xmi:id=\"lower_{}\" value=\"{}\"/>\n",
                attr.id(), mult.lower()
            ));
            out.push_str(&format!(
                "          <upperValue xmi:type=\"uml:LiteralUnlimitedNatural\" xmi:id=\"upper_{}\" value=\"{}\"/>\n",
                attr.id(), upper_str
            ));
            out.push_str("        </ownedAttribute>\n");
        }
        out.push_str("      </packagedElement>\n");
    }

    // Byg mapping fra NodeId til class Uuid i information_graph
    let mut node_to_class: BTreeMap<Uuid, Uuid> = BTreeMap::new();
    for node in project.information_graph().nodes() {
        node_to_class.insert(node.id(), node.class_id());
    }

    // Find generaliseringer pr. klasse fra information_graph
    let mut class_generalizations: BTreeMap<Uuid, Vec<(Uuid, Uuid)>> = BTreeMap::new();
    for edge in project.information_graph().edges() {
        if edge.kind() == RelationKind::Generalization {
            if let (Some(&child_class_id), Some(&parent_class_id)) = (
                node_to_class.get(&edge.from()),
                node_to_class.get(&edge.to()),
            ) {
                class_generalizations
                    .entry(child_class_id)
                    .or_default()
                    .push((edge.from(), parent_class_id));
            }
        }
    }

    // Klasser
    let mut sorted_classes: Vec<_> = info_model.classes().iter().collect();
    sorted_classes.sort_by(|a, b| a.name().cmp(b.name()));
    for cls in sorted_classes {
        let class_id = format!("class_{}", cls.id());
        let class_name = escape_xml(cls.name());
        let is_abstract_str = if cls.is_abstract() { "true" } else { "false" };

        out.push_str(&format!(
            "      <packagedElement xmi:type=\"uml:Class\" xmi:id=\"{}\" name=\"{}\" isAbstract=\"{}\">\n",
            class_id, class_name, is_abstract_str
        ));

        if let Some(desc) = cls.description() {
            if !desc.is_empty() {
                out.push_str(&format!(
                    "        <ownedComment xmi:type=\"uml:Comment\" xmi:id=\"comment_{}\">\n          <body>{}</body>\n        </ownedComment>\n",
                    cls.id(), escape_xml(desc)
                ));
            }
        }

        // Attributter
        for attr in cls.attributes() {
            let attr_id = format!("attr_{}", attr.id());
            let attr_name = escape_xml(attr.name());
            let type_id = match attr.data_type() {
                InformationDataType::Primitive(p) => format!("prim_{}", p.as_str()),
                InformationDataType::Enumeration { enumeration_id } => format!("enum_{}", enumeration_id),
                InformationDataType::Structured { structured_id } => format!("datatype_{}", structured_id),
            };
            out.push_str(&format!(
                "        <ownedAttribute xmi:type=\"uml:Property\" xmi:id=\"{}\" name=\"{}\" type=\"{}\" visibility=\"public\">\n",
                attr_id, attr_name, type_id
            ));
            let mult = attr.multiplicity();
            let upper_str = match mult.upper() {
                Some(u) => u.to_string(),
                None => "*".to_string(),
            };
            out.push_str(&format!(
                "          <lowerValue xmi:type=\"uml:LiteralInteger\" xmi:id=\"lower_{}\" value=\"{}\"/>\n",
                attr.id(), mult.lower()
            ));
            out.push_str(&format!(
                "          <upperValue xmi:type=\"uml:LiteralUnlimitedNatural\" xmi:id=\"upper_{}\" value=\"{}\"/>\n",
                attr.id(), upper_str
            ));
            out.push_str("        </ownedAttribute>\n");
        }

        // Generaliseringer for denne klasse
        if let Some(gens) = class_generalizations.get(&cls.id()) {
            for (idx, (_, parent_id)) in gens.iter().enumerate() {
                let gen_id = format!("gen_{}_{}", cls.id(), idx);
                let target_class_id = format!("class_{}", parent_id);
                out.push_str(&format!(
                    "        <generalization xmi:type=\"uml:Generalization\" xmi:id=\"{}\" general=\"{}\"/>\n",
                    gen_id, target_class_id
                ));
            }
        }

        out.push_str("      </packagedElement>\n");
    }

    // Associationer og Kompositioner
    for (idx, edge) in project.information_graph().edges().iter().enumerate() {
        if edge.kind() == RelationKind::Association || edge.kind() == RelationKind::Composition {
            if let (Some(&src_class_id), Some(&tgt_class_id)) = (
                node_to_class.get(&edge.from()),
                node_to_class.get(&edge.to()),
            ) {
                let assoc_id = format!("assoc_{}_{}", edge.from(), idx);
                let assoc_name = escape_xml(edge.label().unwrap_or(""));
                let is_comp = edge.kind() == RelationKind::Composition;
                let agg_str = if is_comp { "aggregation=\"composite\"" } else { "" };

                let src_prop_id = format!("prop_src_{}", assoc_id);
                let tgt_prop_id = format!("prop_tgt_{}", assoc_id);

                out.push_str(&format!(
                    "      <packagedElement xmi:type=\"uml:Association\" xmi:id=\"{}\" name=\"{}\">\n",
                    assoc_id, assoc_name
                ));
                out.push_str(&format!(
                    "        <memberEnd xmi:idref=\"{}\"/>\n        <memberEnd xmi:idref=\"{}\"/>\n",
                    src_prop_id, tgt_prop_id
                ));

                // Source end
                let src_mult = edge.source_multiplicity().unwrap_or_else(crate::features::information_model::Multiplicity::zero_or_more);
                let src_upper = match src_mult.upper() {
                    Some(u) => u.to_string(),
                    None => "*".to_string(),
                };
                out.push_str(&format!(
                    "        <ownedEnd xmi:type=\"uml:Property\" xmi:id=\"{}\" type=\"class_{}\" visibility=\"public\" {}>\n",
                    src_prop_id, src_class_id, agg_str
                ));
                out.push_str(&format!(
                    "          <lowerValue xmi:type=\"uml:LiteralInteger\" xmi:id=\"lower_{}\" value=\"{}\"/>\n",
                    src_prop_id, src_mult.lower()
                ));
                out.push_str(&format!(
                    "          <upperValue xmi:type=\"uml:LiteralUnlimitedNatural\" xmi:id=\"upper_{}\" value=\"{}\"/>\n",
                    src_prop_id, src_upper
                ));
                out.push_str("        </ownedEnd>\n");

                // Target end
                let tgt_mult = edge.target_multiplicity().unwrap_or_else(crate::features::information_model::Multiplicity::exactly_one);
                let tgt_upper = match tgt_mult.upper() {
                    Some(u) => u.to_string(),
                    None => "*".to_string(),
                };
                out.push_str(&format!(
                    "        <ownedEnd xmi:type=\"uml:Property\" xmi:id=\"{}\" type=\"class_{}\" visibility=\"public\">\n",
                    tgt_prop_id, tgt_class_id
                ));
                out.push_str(&format!(
                    "          <lowerValue xmi:type=\"uml:LiteralInteger\" xmi:id=\"lower_{}\" value=\"{}\"/>\n",
                    tgt_prop_id, tgt_mult.lower()
                ));
                out.push_str(&format!(
                    "          <upperValue xmi:type=\"uml:LiteralUnlimitedNatural\" xmi:id=\"upper_{}\" value=\"{}\"/>\n",
                    tgt_prop_id, tgt_upper
                ));
                out.push_str("        </ownedEnd>\n");

                out.push_str("      </packagedElement>\n");
            }
        }
    }

    out.push_str("    </packagedElement>\n");
    out.push_str("  </uml:Model>\n");
    out.push_str("</xmi:XMI>\n");

    out
}
