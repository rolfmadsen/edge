use crate::features::concept_model::{ConceptGraph, DiagramEdge, DiagramNode, RelationKind};
use crate::features::concepts::Concept;
use crate::features::information_model::{ClassGraph, InformationModel};
use crate::ui::edge_router::EdgeRouter;

/// Hjælpefunktion til at escape tekst til sikker XML/SVG.
pub fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Truncater en streng og tilføjer '…' hvis den overstiger max_len.
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.chars().count() > max_len {
        let truncated: String = text.chars().take(max_len.saturating_sub(1)).collect();
        format!("{}…", truncated)
    } else {
        text.to_string()
    }
}

/// Eksporterer begrebsmodellen (ConceptGraph) til standardiseret vektor-SVG med FDA-styling.
pub fn export_concept_model_svg(graph: &ConceptGraph, _concepts: &[Concept]) -> String {
    let nodes = graph.nodes();
    let edges = graph.edges();

    // 1. Beregn afpasset bounding box for lærredet
    let (view_x, view_y, view_w, view_h) = if nodes.is_empty() {
        (0.0, 0.0, 800.0, 600.0)
    } else {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for n in nodes {
            min_x = min_x.min(n.x());
            min_y = min_y.min(n.y());
            max_x = max_x.max(n.x() + n.width());
            max_y = max_y.max(n.y() + n.height());
        }

        let margin = 50.0;
        let vx = min_x - margin;
        let vy = min_y - margin;
        let vw = (max_x - min_x) + (margin * 2.0);
        let vh = (max_y - min_y) + (margin * 2.0);
        (vx, vy, vw.max(400.0), vh.max(300.0))
    };

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{:.1} {:.1} {:.1} {:.1}\" width=\"{:.0}\" height=\"{:.0}\">\n",
        view_x, view_y, view_w, view_h, view_w, view_h
    ));

    // Styling og definitioner
    svg.push_str("  <defs>\n");
    svg.push_str(
        "    <filter id=\"drop-shadow\" x=\"-10%\" y=\"-10%\" width=\"130%\" height=\"130%\">\n",
    );
    svg.push_str("      <feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"3\" flood-opacity=\"0.08\" flood-color=\"#0F172A\" />\n");
    svg.push_str("    </filter>\n");
    svg.push_str("  </defs>\n");

    // Baggrund
    svg.push_str(&format!(
        "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#F8FAFC\" />\n",
        view_x, view_y, view_w, view_h
    ));

    // 2. Deterministisk ortogonal ruteberegning
    let routed_edges = EdgeRouter::route_edges(nodes, edges);

    // 3. Tegn forbindelseslinjer (Edges)
    for routed in &routed_edges {
        if routed.points.len() < 2 {
            continue;
        }

        let mut path_d = String::new();
        for (i, pt) in routed.points.iter().enumerate() {
            if i == 0 {
                path_d.push_str(&format!("M {:.1} {:.1}", pt.x, pt.y));
            } else {
                path_d.push_str(&format!(" L {:.1} {:.1}", pt.x, pt.y));
            }
        }

        let (dash_attr, stroke_color, stroke_width) = match routed.kind {
            RelationKind::Dependency => (" stroke-dasharray=\"6,4\"", "#64748B", "1.5"),
            RelationKind::Association => ("", "#475569", "1.5"),
            _ => ("", "#334155", "1.5"),
        };

        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"{} />\n",
            path_d, stroke_color, stroke_width, dash_attr
        ));

        // Generaliseringspil (lukket hvid trekant jf. UML & FDA)
        if let Some(ref arrow) = routed.arrow_head {
            if routed.kind == RelationKind::Generalization {
                svg.push_str(&format!(
                    "  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"#FFFFFF\" stroke=\"#334155\" stroke-width=\"1.5\" />\n",
                    arrow.tip.x, arrow.tip.y, arrow.left.x, arrow.left.y, arrow.right.x, arrow.right.y
                ));
            } else if routed.kind == RelationKind::Association {
                svg.push_str(&format!(
                    "  <polyline points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"#475569\" stroke-width=\"1.8\" stroke-linecap=\"round\" />\n",
                    arrow.left.x, arrow.left.y, arrow.tip.x, arrow.tip.y, arrow.right.x, arrow.right.y
                ));
            }
        }

        // Kompositionsdiamant (solid mørk diamant jf. UML)
        if let Some(ref diamond) = routed.source_diamond {
            svg.push_str(&format!(
                "  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"#1E293B\" stroke=\"#1E293B\" stroke-width=\"1.5\" />\n",
                diamond.tip.x, diamond.tip.y, diamond.left.x, diamond.left.y, diamond.back.x, diamond.back.y, diamond.right.x, diamond.right.y
            ));
        }

        // Halv-pil for rettet association
        if let Some(ref half) = routed.half_arrow {
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#475569\" stroke-width=\"1.8\" stroke-linecap=\"round\" />\n",
                half.tip.x, half.tip.y, half.barb.x, half.barb.y
            ));
        }

        // Relationens label hvis angivet
        if let (Some(label), Some(pos)) = (routed.label.as_deref(), routed.label_pos) {
            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"11\" fill=\"#475569\" text-anchor=\"middle\">{}</text>\n",
                pos.x, pos.y - 4.0, xml_escape(label)
            ));
        }
    }

    // 4. Tegn Begrebsnoder
    for node in nodes {
        let (fill_color, border_color) = if node.is_local() {
            ("#FBF3E8", "#D1B894") // FDA Sand
        } else {
            ("#E8F2FB", "#94B8D1") // Indlånt (blå)
        };

        let cx = node.x() + (node.width() / 2.0);
        let cy = node.y() + (node.height() / 2.0);

        svg.push_str(&format!("  <g id=\"node-{}\">\n", node.id()));
        svg.push_str(&format!(
            "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"8\" ry=\"8\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\" filter=\"url(#drop-shadow)\" />\n",
            node.x(), node.y(), node.width(), node.height(), fill_color, border_color
        ));
        svg.push_str(&format!(
            "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"11\" fill=\"#64748B\" text-anchor=\"middle\">«begreb»</text>\n",
            cx, node.y() + 24.0
        ));
        svg.push_str(&format!(
            "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"14\" font-weight=\"600\" fill=\"#0F172A\" text-anchor=\"middle\">{}</text>\n",
            cx, cy + 12.0, xml_escape(&truncate_text(node.label(), 22))
        ));
        svg.push_str("  </g>\n");
    }

    svg.push_str("</svg>");
    svg
}

/// Eksporterer informationsmodellen (ClassGraph & InformationModel) til standardiseret vektor-SVG.
pub fn export_information_model_svg(graph: &ClassGraph, model: &InformationModel) -> String {
    let nodes = graph.nodes();
    let edges = graph.edges();

    // 1. Byg samlet mængde af diagramnoder (inklusive automatisk placering af enumerations og datatyper, der endnu ikke har en eksplicit node i grafen)
    let mut diagram_nodes: Vec<DiagramNode> = nodes
        .iter()
        .map(|n| DiagramNode::custom(n.id(), String::new(), n.x(), n.y(), n.width(), n.height()))
        .collect();

    let auto_x = 420.0;
    let mut auto_y = 100.0;
    for enumeration in model.enumerations() {
        if !nodes.iter().any(|n| n.class_id() == enumeration.id()) {
            let h = 70.0 + (enumeration.values().len() as f32 * 20.0).max(30.0);
            diagram_nodes.push(DiagramNode::custom(
                enumeration.id(),
                enumeration.name().to_string(),
                auto_x,
                auto_y,
                200.0,
                h,
            ));
            auto_y += h + 40.0;
        }
    }
    for st in model.structured_types() {
        if !nodes.iter().any(|n| n.class_id() == st.id()) {
            let h = 70.0 + (st.attributes().len() as f32 * 20.0).max(30.0);
            diagram_nodes.push(DiagramNode::custom(
                st.id(),
                st.name().to_string(),
                auto_x,
                auto_y,
                200.0,
                h,
            ));
            auto_y += h + 40.0;
        }
    }

    // 2. Beregn bounding box
    let (view_x, view_y, view_w, view_h) = if diagram_nodes.is_empty() {
        (0.0, 0.0, 900.0, 650.0)
    } else {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for n in &diagram_nodes {
            min_x = min_x.min(n.x());
            min_y = min_y.min(n.y());
            max_x = max_x.max(n.x() + n.width());
            max_y = max_y.max(n.y() + n.height());
        }

        let margin = 50.0;
        let vx = min_x - margin;
        let vy = min_y - margin;
        let vw = (max_x - min_x) + (margin * 2.0);
        let vh = (max_y - min_y) + (margin * 2.0);
        (vx, vy, vw.max(450.0), vh.max(350.0))
    };

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{:.1} {:.1} {:.1} {:.1}\" width=\"{:.0}\" height=\"{:.0}\">\n",
        view_x, view_y, view_w, view_h, view_w, view_h
    ));

    // Styling og filter
    svg.push_str("  <defs>\n");
    svg.push_str(
        "    <filter id=\"class-shadow\" x=\"-10%\" y=\"-10%\" width=\"130%\" height=\"130%\">\n",
    );
    svg.push_str("      <feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"3\" flood-opacity=\"0.08\" flood-color=\"#0F172A\" />\n");
    svg.push_str("    </filter>\n");
    svg.push_str("  </defs>\n");

    // Baggrund
    svg.push_str(&format!(
        "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#F8FAFC\" />\n",
        view_x, view_y, view_w, view_h
    ));

    // 3. Ruteberegning via EdgeRouter
    let diagram_edges: Vec<DiagramEdge> = edges
        .iter()
        .map(|e| {
            DiagramEdge::with_all(
                e.from(),
                e.to(),
                e.kind(),
                e.label().map(|s| s.to_string()),
                e.source_port(),
                e.target_port(),
                Some(e.is_directed()),
            )
        })
        .collect();

    let routed_edges = EdgeRouter::route_edges(&diagram_nodes, &diagram_edges);

    // 4. Forbindelseslinjer
    for routed in &routed_edges {
        if routed.points.len() < 2 {
            continue;
        }

        let mut path_d = String::new();
        for (i, pt) in routed.points.iter().enumerate() {
            if i == 0 {
                path_d.push_str(&format!("M {:.1} {:.1}", pt.x, pt.y));
            } else {
                path_d.push_str(&format!(" L {:.1} {:.1}", pt.x, pt.y));
            }
        }

        let (dash_attr, stroke_color, stroke_width) = match routed.kind {
            RelationKind::Dependency => (" stroke-dasharray=\"6,4\"", "#64748B", "1.5"),
            RelationKind::Association => ("", "#475569", "1.5"),
            _ => ("", "#334155", "1.5"),
        };

        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"{} />\n",
            path_d, stroke_color, stroke_width, dash_attr
        ));

        // Pilehoveder og diamanter
        if let Some(ref arrow) = routed.arrow_head {
            if routed.kind == RelationKind::Generalization {
                svg.push_str(&format!(
                    "  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"#FFFFFF\" stroke=\"#334155\" stroke-width=\"1.5\" />\n",
                    arrow.tip.x, arrow.tip.y, arrow.left.x, arrow.left.y, arrow.right.x, arrow.right.y
                ));
            } else if routed.kind == RelationKind::Association {
                svg.push_str(&format!(
                    "  <polyline points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"#475569\" stroke-width=\"1.8\" stroke-linecap=\"round\" />\n",
                    arrow.left.x, arrow.left.y, arrow.tip.x, arrow.tip.y, arrow.right.x, arrow.right.y
                ));
            }
        }

        if let Some(ref diamond) = routed.source_diamond {
            svg.push_str(&format!(
                "  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"#1E293B\" stroke=\"#1E293B\" stroke-width=\"1.5\" />\n",
                diamond.tip.x, diamond.tip.y, diamond.left.x, diamond.left.y, diamond.back.x, diamond.back.y, diamond.right.x, diamond.right.y
            ));
        }

        // Multiplicitet hvis matchet med oprindelig edge
        if let Some(orig) = edges
            .iter()
            .find(|e| e.from() == routed.from && e.to() == routed.to)
        {
            if let Some(sm) = orig.source_multiplicity() {
                if let Some(first_pt) = routed.points.first() {
                    svg.push_str(&format!(
                        "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"10\" fill=\"#64748B\">{}</text>\n",
                        first_pt.x + 4.0, first_pt.y - 4.0, xml_escape(&sm.to_display_string())
                    ));
                }
            }
            if let Some(tm) = orig.target_multiplicity() {
                if let Some(last_pt) = routed.points.last() {
                    svg.push_str(&format!(
                        "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"10\" fill=\"#64748B\">{}</text>\n",
                        last_pt.x + 4.0, last_pt.y - 4.0, xml_escape(&tm.to_display_string())
                    ));
                }
            }
        }
    }

    // 5. Tegn Noder (Klasser, Enumerationer, Strukturerede datatyper)
    for d_node in &diagram_nodes {
        let cx = d_node.x() + (d_node.width() / 2.0);

        // Find tilknyttet modelentitet (enten via ClassDiagramNode eller direkte id)
        let entity_id = if let Some(c_node) = nodes.iter().find(|n| n.id() == d_node.id()) {
            c_node.class_id()
        } else {
            d_node.id()
        };

        if let Some(class_obj) = model.get_class(entity_id) {
            let is_abstract = class_obj.is_abstract();
            let is_borrowed = !class_obj.is_local();
            let stereotype = if is_abstract {
                "«abstrakt klasse»"
            } else {
                "«klasse»"
            };
            let (fill_color, border_color) = if is_borrowed {
                ("#E8F2FB", "#94B8D1") // Blå (indlånt)
            } else {
                ("#FBF3E8", "#D1B894") // FDA Sand
            };

            svg.push_str(&format!("  <g id=\"class-{}\">\n", class_obj.id()));
            svg.push_str(&format!(
                "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" ry=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\" filter=\"url(#class-shadow)\" />\n",
                d_node.x(), d_node.y(), d_node.width(), d_node.height(), fill_color, border_color
            ));

            // Stereotype & Navn
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"10.5\" fill=\"#475569\" text-anchor=\"middle\">{}</text>\n",
                cx, d_node.y() + 16.0, stereotype
            ));
            let font_style = if is_abstract {
                " font-style=\"italic\""
            } else {
                ""
            };
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"13.5\" font-weight=\"bold\" fill=\"#0F172A\" text-anchor=\"middle\"{}>{}</text>\n",
                cx, d_node.y() + 34.0, font_style, xml_escape(class_obj.name())
            ));

            // Skillelinje
            let div_y = d_node.y() + 44.0;
            svg.push_str(&format!(
                "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.0\" />\n",
                d_node.x(), div_y, d_node.x() + d_node.width(), div_y, border_color
            ));

            // Attributter
            let mut attr_y = div_y + 16.0;
            if class_obj.attributes().is_empty() {
                svg.push_str(&format!(
                    "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"11\" fill=\"#94A3B8\">(ingen attributter)</text>\n",
                    d_node.x() + 12.0, attr_y
                ));
            } else {
                for attr in class_obj.attributes() {
                    let attr_line = format!(
                        "+ {} : {} [{}]",
                        attr.name(),
                        attr.data_type().as_str(),
                        attr.multiplicity()
                    );
                    svg.push_str(&format!(
                        "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"monospace, system-ui, sans-serif\" font-size=\"11.5\" fill=\"#1E293B\">{}</text>\n",
                        d_node.x() + 12.0, attr_y, xml_escape(&truncate_text(&attr_line, 28))
                    ));
                    attr_y += 18.0;
                }
            }
            svg.push_str("  </g>\n");
        } else if let Some(enumeration) = model.get_enumeration(entity_id) {
            // FDA Enum Grøn (#E8FDE3)
            let fill_color = "#E8FDE3";
            let border_color = "#A3D99B";

            svg.push_str(&format!("  <g id=\"enum-{}\">\n", enumeration.id()));
            svg.push_str(&format!(
                "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" ry=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\" filter=\"url(#class-shadow)\" />\n",
                d_node.x(), d_node.y(), d_node.width(), d_node.height(), fill_color, border_color
            ));
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"10.5\" fill=\"#166534\" text-anchor=\"middle\">«enumeration»</text>\n",
                cx, d_node.y() + 16.0
            ));
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"13.5\" font-weight=\"bold\" fill=\"#14532D\" text-anchor=\"middle\">{}</text>\n",
                cx, d_node.y() + 34.0, xml_escape(enumeration.name())
            ));

            let div_y = d_node.y() + 44.0;
            svg.push_str(&format!(
                "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.0\" />\n",
                d_node.x(), div_y, d_node.x() + d_node.width(), div_y, border_color
            ));

            let mut val_y = div_y + 16.0;
            for val in enumeration.values() {
                svg.push_str(&format!(
                    "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"monospace, system-ui, sans-serif\" font-size=\"11.5\" fill=\"#14532D\">+ {}</text>\n",
                    d_node.x() + 12.0, val_y, xml_escape(&truncate_text(val, 28))
                ));
                val_y += 18.0;
            }
            svg.push_str("  </g>\n");
        } else if let Some(structured) = model.get_structured_type(entity_id) {
            // FDA DataType Gul (#FBF9C6)
            let fill_color = "#FBF9C6";
            let border_color = "#E5DF88";

            svg.push_str(&format!("  <g id=\"datatype-{}\">\n", structured.id()));
            svg.push_str(&format!(
                "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" ry=\"6\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1.2\" filter=\"url(#class-shadow)\" />\n",
                d_node.x(), d_node.y(), d_node.width(), d_node.height(), fill_color, border_color
            ));
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"10.5\" fill=\"#854D0E\" text-anchor=\"middle\">«dataType»</text>\n",
                cx, d_node.y() + 16.0
            ));
            svg.push_str(&format!(
                "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"13.5\" font-weight=\"bold\" fill=\"#713F12\" text-anchor=\"middle\">{}</text>\n",
                cx, d_node.y() + 34.0, xml_escape(structured.name())
            ));

            let div_y = d_node.y() + 44.0;
            svg.push_str(&format!(
                "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.0\" />\n",
                d_node.x(), div_y, d_node.x() + d_node.width(), div_y, border_color
            ));

            let mut attr_y = div_y + 16.0;
            for attr in structured.attributes() {
                let attr_line = format!(
                    "+ {} : {} [{}]",
                    attr.name(),
                    attr.data_type().as_str(),
                    attr.multiplicity()
                );
                svg.push_str(&format!(
                    "    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"monospace, system-ui, sans-serif\" font-size=\"11.5\" fill=\"#713F12\">{}</text>\n",
                    d_node.x() + 12.0, attr_y, xml_escape(&truncate_text(&attr_line, 28))
                ));
                attr_y += 18.0;
            }
            svg.push_str("  </g>\n");
        }
    }

    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_escaping() {
        assert_eq!(xml_escape("Normale ord"), "Normale ord");
        assert_eq!(
            xml_escape("A & B < C > 'D' \"E\""),
            "A &amp; B &lt; C &gt; &apos;D&apos; &quot;E&quot;"
        );
    }

    #[test]
    fn test_empty_canvas_svg_generation() {
        let empty_cg = ConceptGraph::new();
        let svg = export_concept_model_svg(&empty_cg, &[]);
        assert!(svg.starts_with("<svg "));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("viewBox=\"0.0 0.0 800.0 600.0\""));

        let empty_ig = ClassGraph::new();
        let empty_im = InformationModel::new();
        let info_svg = export_information_model_svg(&empty_ig, &empty_im);
        assert!(info_svg.starts_with("<svg "));
        assert!(info_svg.ends_with("</svg>"));
        assert!(info_svg.contains("viewBox=\"0.0 0.0 900.0 650.0\""));
    }
}
