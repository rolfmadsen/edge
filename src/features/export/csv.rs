use crate::features::concepts::{BelongsToDomain, Concept};

/// Hjælpefunktion til at escape et CSV-felt i overensstemmelse med RFC-4180.
///
/// Hvis feltet indeholder citationstegn (`"`), komma (`,`) eller linjeskift (`\r`, `\n`),
/// omsluttes feltet med citationstegn, og alle interne citationstegn dubleres (`""`).
pub fn escape_rfc4180(field: &str) -> String {
    let needs_quotes =
        field.contains('"') || field.contains(',') || field.contains('\r') || field.contains('\n');
    if needs_quotes {
        let mut escaped = String::with_capacity(field.len() + 10);
        escaped.push('"');
        for ch in field.chars() {
            if ch == '"' {
                escaped.push_str("\"\"");
            } else {
                escaped.push(ch);
            }
        }
        escaped.push('"');
        escaped
    } else {
        field.to_string()
    }
}

/// Eksporterer en liste af FDA-begreber til RFC-4180 kompatibel CSV med UTF-8 BOM.
///
/// Indeholder præcis de 12 standardiserede kolonner jf. FDA Modelreglerne Bilag D og E:
/// 1. Foretrukken term
/// 2. Accepteret term
/// 3. Frarådet term
/// 4. Definition
/// 5. Eksempel
/// 6. Kommentar
/// 7. Anvendelsesnote
/// 8. Juridisk kilde
/// 9. Kilde
/// 10. Tilhører emneområde
/// 11. Identifikator
/// 12. Afledt af
pub fn export_concepts_to_csv(concepts: &[Concept]) -> String {
    let mut out = String::new();

    // UTF-8 BOM (\u{FEFF}) sikrer at Microsoft Excel og andre værktøjer automatisk detekterer UTF-8 (æ, ø, å)
    out.push('\u{FEFF}');

    // Overskrift med samtlige 12 FDA standardkolonner
    let headers = [
        "Foretrukken term",
        "Accepteret term",
        "Frarådet term",
        "Definition",
        "Eksempel",
        "Kommentar",
        "Anvendelsesnote",
        "Juridisk kilde",
        "Kilde",
        "Tilhører emneområde",
        "Identifikator",
        "Afledt af",
    ];

    let header_line = headers
        .iter()
        .map(|h| format!("\"{}\"", h))
        .collect::<Vec<_>>()
        .join(",");
    out.push_str(&header_line);
    out.push_str("\r\n");

    for c in concepts {
        let belongs_str = match c.belongs_to_domain() {
            BelongsToDomain::Yes => "Ja".to_string(),
            BelongsToDomain::No => "Nej".to_string(),
            BelongsToDomain::ModelRef(uri) => format!("Model: {}", uri),
        };

        let row = [
            escape_rfc4180(c.preferred_term()),
            escape_rfc4180(c.accepted_term().unwrap_or("")),
            escape_rfc4180(c.deprecated_term().unwrap_or("")),
            escape_rfc4180(c.definition()),
            escape_rfc4180(c.example().unwrap_or("")),
            escape_rfc4180(c.comment().unwrap_or("")),
            escape_rfc4180(c.application_note().unwrap_or("")),
            escape_rfc4180(c.legal_source().unwrap_or("")),
            escape_rfc4180(c.source().unwrap_or("")),
            escape_rfc4180(&belongs_str),
            escape_rfc4180(c.identifier().unwrap_or("")),
            escape_rfc4180(c.derived_from().unwrap_or("")),
        ];

        out.push_str(&row.join(","));
        out.push_str("\r\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc4180_escaping() {
        assert_eq!(escape_rfc4180("Enkelt"), "Enkelt");
        assert_eq!(escape_rfc4180("Med, komma"), "\"Med, komma\"");
        assert_eq!(
            escape_rfc4180("Med \"gåseøjne\""),
            "\"Med \"\"gåseøjne\"\"\""
        );
        assert_eq!(escape_rfc4180("Linje1\nLinje2"), "\"Linje1\nLinje2\"");
    }
}
