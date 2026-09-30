use crate::features::concepts::BelongsToDomain;
use crate::features::export::svg::{
    export_concept_model_svg, export_information_model_svg, xml_escape,
};
use crate::features::model::ModelProject;

/// Eksporterer en samlet officiel FDA indleveringsrapport i Markdown format.
pub fn export_model_report_markdown(project: &ModelProject) -> String {
    let meta = project.metadata();
    let mut out = String::new();

    // Hovedtitel og beskrivelse
    out.push_str(&format!("# {}\n\n", meta.name()));
    if !meta.description().is_empty() {
        out.push_str(&format!("> {}\n\n", meta.description()));
    }

    // Indholdsfortegnelse
    out.push_str("## Indholdsfortegnelse\n\n");
    out.push_str("1. [Tabel D: Modelmetadata](#1-tabel-d-modelmetadata)\n");
    out.push_str("2. [Lovgrundlag & Juridiske Kilder](#2-lovgrundlag--juridiske-kilder)\n");
    out.push_str("3. [Begrebskatalog (Bilag D & E)](#3-begrebskatalog-bilag-d--e)\n");
    out.push_str("4. [Begrebsmodel & Relationer](#4-begrebsmodel--relationer)\n");
    out.push_str(
        "5. [Informationsmodel & Datastrukturer](#5-informationsmodel--datastrukturer)\n\n",
    );

    // Sektion 1: Tabel D Modelmetadata
    out.push_str("## 1. Tabel D: Modelmetadata\n\n");
    out.push_str("| Metadatafelt | Værdi |\n");
    out.push_str("| :--- | :--- |\n");
    out.push_str(&format!("| **Modelnavn** | {} |\n", meta.name()));
    out.push_str(&format!("| **Beskrivelse** | {} |\n", meta.description()));
    out.push_str(&format!("| **Identifikator (URI)** | `{}` |\n", meta.uri()));
    out.push_str(&format!(
        "| **Ansvarlig organisation** | {} |\n",
        meta.responsible_org()
    ));
    out.push_str(&format!("| **Emneområde** | {} |\n", meta.domain_area()));
    out.push_str(&format!("| **Version** | {} |\n", meta.version()));
    out.push_str(&format!("| **Modelstatus** | {} |\n", meta.model_status()));
    out.push_str(&format!(
        "| **Godkendelsesstatus** | {} |\n",
        meta.approval_status()
    ));
    if let Some(approver) = meta.approved_by() {
        out.push_str(&format!("| **Godkendt af** | {} |\n", approver));
    }
    out.push_str(&format!("| **Modelomfang** | {} |\n", meta.model_scope()));
    out.push_str(&format!("| **Sprog** | {} |\n", meta.language()));
    out.push_str(&format!(
        "| **Sidst ændret** | {} |\n",
        meta.date_modified()
    ));
    if let Some(notes) = meta.version_notes() {
        out.push_str(&format!("| **Versionsnoter** | {} |\n", notes));
    }
    if let Some(src) = meta.source() {
        out.push_str(&format!("| **Kilde** | {} |\n", src));
    }
    if let Some(derived) = meta.was_derived_from() {
        out.push_str(&format!("| **Afledt af** | `{}` |\n", derived));
    }
    out.push('\n');

    // Sektion 2: Lovgrundlag & Juridiske Kilder
    out.push_str("## 2. Lovgrundlag & Juridiske Kilder\n\n");
    if meta.legal_sources().is_empty() {
        out.push_str("*Ingen specifikke juridiske kilder registreret for denne model.*\n\n");
    } else {
        out.push_str("Følgende lovgrundlag og juridiske kilder danner hjemmel for modellens begreber og forretningsregler:\n\n");
        for ls in meta.legal_sources() {
            out.push_str(&format!("- ⚖️ **{}**\n", ls));
        }
        out.push('\n');
    }

    // Sektion 3: Begrebskatalog
    out.push_str("## 3. Begrebskatalog (Bilag D & E)\n\n");
    if project.concepts().is_empty() {
        out.push_str("*Modellen indeholder endnu ingen begreber.*\n\n");
    } else {
        for c in project.concepts() {
            out.push_str(&format!("### {}\n\n", c.preferred_term()));
            out.push_str(&format!("**Definition**: {}\n\n", c.definition()));

            out.push_str("| Egenskab | Værdi |\n");
            out.push_str("| :--- | :--- |\n");
            if let Some(acc) = c.accepted_term() {
                out.push_str(&format!("| Accepteret term | {} |\n", acc));
            }
            if let Some(dep) = c.deprecated_term() {
                out.push_str(&format!("| Frarådet term | {} |\n", dep));
            }
            if let Some(ex) = c.example() {
                out.push_str(&format!("| Eksempel | {} |\n", ex));
            }
            if let Some(com) = c.comment() {
                out.push_str(&format!("| Kommentar | {} |\n", com));
            }
            if let Some(app) = c.application_note() {
                out.push_str(&format!("| Anvendelsesnote | {} |\n", app));
            }
            if let Some(ls) = c.legal_source() {
                out.push_str(&format!("| Juridisk kilde | {} |\n", ls));
            }
            if let Some(src) = c.source() {
                out.push_str(&format!("| Kilde | {} |\n", src));
            }
            let belongs_str = match c.belongs_to_domain() {
                BelongsToDomain::Yes => "Ja (Lokalt begreb)".to_string(),
                BelongsToDomain::No => "Nej (Indlånt begreb)".to_string(),
                BelongsToDomain::ModelRef(uri) => format!("Model: {}", uri),
            };
            out.push_str(&format!("| Tilhører emneområde | {} |\n", belongs_str));
            if let Some(id) = c.identifier() {
                out.push_str(&format!("| Identifikator (URI) | `{}` |\n", id));
            }
            if let Some(der) = c.derived_from() {
                out.push_str(&format!("| Afledt af | `{}` |\n", der));
            }
            out.push('\n');
        }
    }

    // Sektion 4: Begrebsmodel & Relationer
    out.push_str("## 4. Begrebsmodel & Relationer\n\n");
    let cg = project.concept_graph();
    out.push_str(&format!(
        "- Antal begrebsnoder på lærredet: **{}**\n",
        cg.node_count()
    ));
    out.push_str(&format!("- Antal relationer: **{}**\n\n", cg.edge_count()));

    if !cg.edges().is_empty() {
        out.push_str("| Kilde (Fra) | Relationstype | Mål (Til) | Label |\n");
        out.push_str("| :--- | :--- | :--- | :--- |\n");
        for edge in cg.edges() {
            let from_name = cg
                .find_node(edge.from())
                .map(|n| n.label())
                .unwrap_or("Ukendt");
            let to_name = cg
                .find_node(edge.to())
                .map(|n| n.label())
                .unwrap_or("Ukendt");
            let label = edge.label().unwrap_or("-");
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                from_name,
                edge.kind(),
                to_name,
                label
            ));
        }
        out.push('\n');
    }

    // Sektion 5: Informationsmodel & Datastrukturer
    out.push_str("## 5. Informationsmodel & Datastrukturer\n\n");
    let im = project.information_model();
    out.push_str(&format!(
        "- Antal informationsklasser: **{}**\n",
        im.classes().len()
    ));
    out.push_str(&format!(
        "- Antal enumerationer: **{}**\n",
        im.enumerations().len()
    ));
    out.push_str(&format!(
        "- Antal strukturerede datatyper: **{}**\n\n",
        im.structured_types().len()
    ));

    for class in im.classes() {
        let stereo = if class.is_abstract() {
            "«abstrakt klasse» "
        } else {
            ""
        };
        out.push_str(&format!("### {}{}\n\n", stereo, class.name()));
        if let Some(desc) = class.description() {
            out.push_str(&format!("{}\n\n", desc));
        }

        if class.attributes().is_empty() {
            out.push_str("*Ingen attributter defineret for denne klasse.*\n\n");
        } else {
            out.push_str("| Attribut | Datatype | Multiplicitet | Koncept-sporing |\n");
            out.push_str("| :--- | :--- | :--- | :--- |\n");
            for attr in class.attributes() {
                let concept_trace = if attr.concept_ids().is_empty() {
                    "-".to_string()
                } else {
                    format!("{} begreb(er)", attr.concept_ids().len())
                };
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {} |\n",
                    attr.name(),
                    attr.data_type().as_str(),
                    attr.multiplicity(),
                    concept_trace
                ));
            }
            out.push('\n');
        }
    }

    for enumeration in im.enumerations() {
        out.push_str(&format!("### «enumeration» {}\n\n", enumeration.name()));
        if let Some(def) = enumeration.definition() {
            out.push_str(&format!("{}\n\n", def));
        }
        out.push_str("Tilladte udfaldsværdier:\n\n");
        for val in enumeration.values() {
            out.push_str(&format!("- `{}`\n", val));
        }
        out.push('\n');
    }

    for st in im.structured_types() {
        out.push_str(&format!("### «dataType» {}\n\n", st.name()));
        if let Some(def) = st.definition() {
            out.push_str(&format!("{}\n\n", def));
        }
        if !st.attributes().is_empty() {
            out.push_str("| Under-attribut | Datatype | Multiplicitet |\n");
            out.push_str("| :--- | :--- | :--- |\n");
            for attr in st.attributes() {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` |\n",
                    attr.name(),
                    attr.data_type().as_str(),
                    attr.multiplicity()
                ));
            }
            out.push('\n');
        }
    }

    out
}

/// Eksporterer en samlet modelrapport som et komplet, stilfuldt HTML5 dokument med indlejrede SVG diagrammer.
pub fn export_model_report_html(project: &ModelProject) -> String {
    let meta = project.metadata();
    let concept_svg = export_concept_model_svg(project.concept_graph(), project.concepts());
    let info_svg =
        export_information_model_svg(project.information_graph(), project.information_model());

    let mut out = String::new();
    out.push_str("<!DOCTYPE html>\n<html lang=\"da\">\n<head>\n");
    out.push_str("  <meta charset=\"utf-8\">\n");
    out.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    out.push_str(&format!("  <title>{}</title>\n", xml_escape(meta.name())));
    out.push_str("  <style>\n");
    out.push_str("    :root {\n");
    out.push_str("      --primary: #0F52BA;\n");
    out.push_str("      --primary-light: #EBF2FE;\n");
    out.push_str("      --slate-900: #0F172A;\n");
    out.push_str("      --slate-800: #1E293B;\n");
    out.push_str("      --slate-700: #334155;\n");
    out.push_str("      --slate-600: #475569;\n");
    out.push_str("      --slate-200: #E2E8F0;\n");
    out.push_str("      --slate-100: #F1F5F9;\n");
    out.push_str("      --fda-sand: #FBF3E8;\n");
    out.push_str("      --fda-blue: #E8F2FB;\n");
    out.push_str("      --fda-green: #E8FDE3;\n");
    out.push_str("      --fda-yellow: #FBF9C6;\n");
    out.push_str("    }\n");
    out.push_str("    body {\n");
    out.push_str("      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;\n");
    out.push_str("      line-height: 1.6;\n");
    out.push_str("      color: var(--slate-800);\n");
    out.push_str("      background-color: #FAFAFA;\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      padding: 40px 20px;\n");
    out.push_str("    }\n");
    out.push_str("    .report-container {\n");
    out.push_str("      max-width: 960px;\n");
    out.push_str("      margin: 0 auto;\n");
    out.push_str("      background: #FFFFFF;\n");
    out.push_str("      padding: 48px;\n");
    out.push_str("      border-radius: 12px;\n");
    out.push_str("      box-shadow: 0 4px 20px rgba(0,0,0,0.06);\n");
    out.push_str("    }\n");
    out.push_str("    h1, h2, h3, h4 { color: var(--slate-900); font-weight: 700; }\n");
    out.push_str("    h1 { font-size: 2.2rem; border-bottom: 2px solid var(--primary); padding-bottom: 12px; margin-top: 0; }\n");
    out.push_str("    h2 { font-size: 1.5rem; margin-top: 40px; border-bottom: 1px solid var(--slate-200); padding-bottom: 8px; }\n");
    out.push_str("    h3 { font-size: 1.2rem; margin-top: 24px; }\n");
    out.push_str("    .lead-description { font-size: 1.1rem; color: var(--slate-600); margin-bottom: 30px; font-style: italic; }\n");
    out.push_str("    .toc { background: var(--slate-100); border-radius: 8px; padding: 20px 30px; margin-bottom: 40px; }\n");
    out.push_str("    .toc ul { margin: 8px 0 0 0; padding-left: 20px; }\n");
    out.push_str("    .toc li { margin-bottom: 6px; }\n");
    out.push_str(
        "    .toc a { color: var(--primary); text-decoration: none; font-weight: 500; }\n",
    );
    out.push_str("    .toc a:hover { text-decoration: underline; }\n");
    out.push_str("    table { width: 100%; border-collapse: collapse; margin: 16px 0 28px 0; font-size: 0.95rem; }\n");
    out.push_str("    th, td { border: 1px solid var(--slate-200); padding: 10px 14px; text-align: left; }\n");
    out.push_str(
        "    th { background: var(--slate-100); color: var(--slate-700); font-weight: 600; }\n",
    );
    out.push_str("    tr:nth-child(even) { background-color: #FAFCFE; }\n");
    out.push_str("    .badge { display: inline-block; padding: 3px 8px; border-radius: 4px; font-size: 0.85rem; font-weight: 600; }\n");
    out.push_str(
        "    .badge-primary { background: var(--primary-light); color: var(--primary); }\n",
    );
    out.push_str("    .diagram-card { border: 1px solid var(--slate-200); border-radius: 8px; overflow: hidden; margin: 20px 0; background: #FFF; text-align: center; padding: 16px; }\n");
    out.push_str("    .diagram-card svg { max-width: 100%; height: auto; }\n");
    out.push_str("    code { font-family: ui-monospace, SFMono-Regular, Consolas, monospace; background: var(--slate-100); padding: 2px 6px; border-radius: 4px; font-size: 0.9em; }\n");
    out.push_str("    @media print {\n");
    out.push_str("      body { background: #FFF; padding: 0; }\n");
    out.push_str(
        "      .report-container { box-shadow: none; padding: 0; width: 100%; max-width: 100%; }\n",
    );
    out.push_str("      h2 { page-break-before: always; }\n");
    out.push_str("    }\n");
    out.push_str("  </style>\n");
    out.push_str("</head>\n<body>\n");
    out.push_str("  <div class=\"report-container\">\n");

    // Hovedoverskrift
    out.push_str(&format!("    <h1>{}</h1>\n", xml_escape(meta.name())));
    if !meta.description().is_empty() {
        out.push_str(&format!(
            "    <p class=\"lead-description\">{}</p>\n",
            xml_escape(meta.description())
        ));
    }

    // Indholdsfortegnelse
    out.push_str("    <div class=\"toc\">\n");
    out.push_str("      <strong>Indholdsfortegnelse</strong>\n");
    out.push_str("      <ul>\n");
    out.push_str("        <li><a href=\"#sec-metadata\">1. Tabel D: Modelmetadata</a></li>\n");
    out.push_str(
        "        <li><a href=\"#sec-legal\">2. Lovgrundlag &amp; Juridiske Kilder</a></li>\n",
    );
    out.push_str(
        "        <li><a href=\"#sec-concepts\">3. Begrebskatalog (Bilag D &amp; E)</a></li>\n",
    );
    out.push_str("        <li><a href=\"#sec-concept-model\">4. Begrebsmodel (Diagram)</a></li>\n");
    out.push_str("        <li><a href=\"#sec-info-model\">5. Informationsmodel (UML Diagram &amp; Klasser)</a></li>\n");
    out.push_str("      </ul>\n");
    out.push_str("    </div>\n");

    // Sektion 1: Metadata
    out.push_str("    <h2 id=\"sec-metadata\">1. Tabel D: Modelmetadata</h2>\n");
    out.push_str("    <table>\n");
    out.push_str(
        "      <thead><tr><th style=\"width: 30%;\">Metadatafelt</th><th>Værdi</th></tr></thead>\n",
    );
    out.push_str("      <tbody>\n");
    out.push_str(&format!(
        "        <tr><td><strong>Modelnavn</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.name())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Beskrivelse</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.description())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Identifikator (URI)</strong></td><td><code>{}</code></td></tr>\n",
        xml_escape(meta.uri())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Ansvarlig organisation</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.responsible_org())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Emneområde</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.domain_area())
    ));
    out.push_str(&format!("        <tr><td><strong>Version</strong></td><td><span class=\"badge badge-primary\">{}</span></td></tr>\n", xml_escape(meta.version())));
    out.push_str(&format!(
        "        <tr><td><strong>Modelstatus</strong></td><td>{}</td></tr>\n",
        xml_escape(&meta.model_status().to_string())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Godkendelsesstatus</strong></td><td>{}</td></tr>\n",
        xml_escape(&meta.approval_status().to_string())
    ));
    if let Some(appr) = meta.approved_by() {
        out.push_str(&format!(
            "        <tr><td><strong>Godkendt af</strong></td><td>{}</td></tr>\n",
            xml_escape(appr)
        ));
    }
    out.push_str(&format!(
        "        <tr><td><strong>Modelomfang</strong></td><td>{}</td></tr>\n",
        xml_escape(&meta.model_scope().to_string())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Sprog</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.language())
    ));
    out.push_str(&format!(
        "        <tr><td><strong>Sidst ændret</strong></td><td>{}</td></tr>\n",
        xml_escape(meta.date_modified())
    ));
    if let Some(notes) = meta.version_notes() {
        out.push_str(&format!(
            "        <tr><td><strong>Versionsnoter</strong></td><td>{}</td></tr>\n",
            xml_escape(notes)
        ));
    }
    out.push_str("      </tbody>\n    </table>\n");

    // Sektion 2: Lovgrundlag
    out.push_str("    <h2 id=\"sec-legal\">2. Lovgrundlag &amp; Juridiske Kilder</h2>\n");
    if meta.legal_sources().is_empty() {
        out.push_str(
            "    <p><em>Ingen specifikke juridiske kilder angivet for denne model.</em></p>\n",
        );
    } else {
        out.push_str("    <ul>\n");
        for ls in meta.legal_sources() {
            out.push_str(&format!(
                "      <li>⚖️ <strong>{}</strong></li>\n",
                xml_escape(ls)
            ));
        }
        out.push_str("    </ul>\n");
    }

    // Sektion 3: Begrebskatalog
    out.push_str("    <h2 id=\"sec-concepts\">3. Begrebskatalog (Bilag D &amp; E)</h2>\n");
    for c in project.concepts() {
        out.push_str(&format!(
            "    <h3>{}</h3>\n",
            xml_escape(c.preferred_term())
        ));
        out.push_str(&format!(
            "    <p><strong>Definition:</strong> {}</p>\n",
            xml_escape(c.definition())
        ));

        out.push_str("    <table>\n      <tbody>\n");
        if let Some(acc) = c.accepted_term() {
            out.push_str(&format!(
                "        <tr><td style=\"width: 30%;\">Accepteret term</td><td>{}</td></tr>\n",
                xml_escape(acc)
            ));
        }
        if let Some(dep) = c.deprecated_term() {
            out.push_str(&format!(
                "        <tr><td>Frarådet term</td><td>{}</td></tr>\n",
                xml_escape(dep)
            ));
        }
        if let Some(ex) = c.example() {
            out.push_str(&format!(
                "        <tr><td>Eksempel</td><td>{}</td></tr>\n",
                xml_escape(ex)
            ));
        }
        if let Some(com) = c.comment() {
            out.push_str(&format!(
                "        <tr><td>Kommentar</td><td>{}</td></tr>\n",
                xml_escape(com)
            ));
        }
        if let Some(app) = c.application_note() {
            out.push_str(&format!(
                "        <tr><td>Anvendelsesnote</td><td>{}</td></tr>\n",
                xml_escape(app)
            ));
        }
        if let Some(ls) = c.legal_source() {
            out.push_str(&format!(
                "        <tr><td>Juridisk kilde</td><td>{}</td></tr>\n",
                xml_escape(ls)
            ));
        }
        if let Some(id) = c.identifier() {
            out.push_str(&format!(
                "        <tr><td>Identifikator</td><td><code>{}</code></td></tr>\n",
                xml_escape(id)
            ));
        }
        out.push_str("      </tbody>\n    </table>\n");
    }

    // Sektion 4: Begrebsmodel Diagram
    out.push_str("    <h2 id=\"sec-concept-model\">4. Begrebsmodel</h2>\n");
    out.push_str("    <div class=\"diagram-card\">\n");
    out.push_str(&format!("      {}\n", concept_svg));
    out.push_str("    </div>\n");

    // Sektion 5: Informationsmodel Diagram & Klasser
    out.push_str("    <h2 id=\"sec-info-model\">5. Informationsmodel (UML)</h2>\n");
    out.push_str("    <div class=\"diagram-card\">\n");
    out.push_str(&format!("      {}\n", info_svg));
    out.push_str("    </div>\n");

    out.push_str("  </div>\n</body>\n</html>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::model::{ModelMetadata, ModelStatus};

    #[test]
    fn test_report_generation_empty_project() {
        let meta = ModelMetadata::new(
            "Testmodel",
            "Beskrivelse for test",
            "https://data.gov.dk/model/test",
            "Digitaliseringsstyrelsen",
            "Generel",
            "0.1.0",
            ModelStatus::Development,
        );
        let project = ModelProject::new(meta);

        let md = export_model_report_markdown(&project);
        assert!(md.contains("# Testmodel"));
        assert!(md.contains("## Indholdsfortegnelse"));
        assert!(md.contains("Digitaliseringsstyrelsen"));

        let html = export_model_report_html(&project);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>Testmodel</title>"));
    }
}
