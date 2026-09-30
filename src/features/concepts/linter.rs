use crate::features::concepts::Concept;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefinitionIssueKind {
    CapitalizedFirstLetter,
    TrailingPeriod,
    CircularTermReference,
    CircularAcceptedTermReference,
    ForbiddenPrefixPhrase,
    VagueWord,
    NegativeDefinition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefinitionSeverity {
    Warning,
    Suggestion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionLintIssue {
    pub kind: DefinitionIssueKind,
    pub severity: DefinitionSeverity,
    pub message: String,
    pub rule: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AristotleBreakdown {
    pub genus_proximum: Option<String>,
    pub differentia_specifica: Option<String>,
}

pub struct DefinitionLinter;

impl DefinitionLinter {
    pub fn lint(concept: &Concept) -> Vec<DefinitionLintIssue> {
        Self::lint_text(
            concept.preferred_term(),
            concept.accepted_term(),
            concept.definition(),
        )
    }

    pub fn lint_text(
        preferred_term: &str,
        accepted_term: Option<&str>,
        definition: &str,
    ) -> Vec<DefinitionLintIssue> {
        let mut issues = Vec::new();
        let trimmed = definition.trim();
        if trimmed.is_empty() {
            return issues;
        }

        // 1. Formatering: Stort begyndelsesbogstav (ISO 704 / Regel 20)
        if let Some(first_char) = trimmed.chars().next() {
            if first_char.is_uppercase() {
                issues.push(DefinitionLintIssue {
                    kind: DefinitionIssueKind::CapitalizedFirstLetter,
                    severity: DefinitionSeverity::Warning,
                    message: "Definitionen bør starte med lille begyndelsesbogstav for at kunne erstatte termen direkte (ISO 704 / Regel 20).".to_string(),
                    rule: "§20",
                });
            }
        }

        // 2. Formatering: Afsluttende punktum (ISO 704 / Regel 20)
        if trimmed.ends_with('.') || trimmed.ends_with(';') {
            issues.push(DefinitionLintIssue {
                kind: DefinitionIssueKind::TrailingPeriod,
                severity: DefinitionSeverity::Warning,
                message: "Definitionen er en frase og bør ikke afsluttes med punktum (ISO 704 / Regel 20).".to_string(),
                rule: "§20",
            });
        }

        // 3. Forbudte fyldfraser / indledninger (Regel 20)
        let lower = trimmed.to_lowercase();
        const FORBIDDEN_PREFIXES: &[(&str, &str)] = &[
            ("er en ", "er en"),
            ("er et ", "er et"),
            ("er den ", "er den"),
            ("er det ", "er det"),
            ("defineres som ", "defineres som"),
            ("betyder ", "betyder"),
            ("det betyder at ", "det betyder at"),
            ("henvisning til ", "henvisning til"),
            ("henviser til ", "henviser til"),
            ("angivelse af ", "angivelse af"),
            ("angiver ", "angiver"),
            ("reference til ", "reference til"),
            ("betegnelse for ", "betegnelse for"),
            ("betegner ", "betegner"),
        ];

        for (prefix, label) in FORBIDDEN_PREFIXES {
            if lower.starts_with(prefix) {
                issues.push(DefinitionLintIssue {
                    kind: DefinitionIssueKind::ForbiddenPrefixPhrase,
                    severity: DefinitionSeverity::Warning,
                    message: format!(
                        "Undgå tomme fyldfraser som indledning: '{}' (Regel 20).",
                        label
                    ),
                    rule: "§20",
                });
                break;
            }
        }

        // 4. Cirkularitetskontrol (Regel 20)
        let term = preferred_term.trim();
        if !term.is_empty() && contains_word(trimmed, term) {
            issues.push(DefinitionLintIssue {
                kind: DefinitionIssueKind::CircularTermReference,
                severity: DefinitionSeverity::Warning,
                message: format!(
                    "Definitionen må ikke være cirkulær ved at indeholde termen '{}' (Regel 20).",
                    term
                ),
                rule: "§20",
            });
        }

        if let Some(accepted) = accepted_term {
            let syn = accepted.trim();
            if !syn.is_empty() && contains_word(trimmed, syn) {
                issues.push(DefinitionLintIssue {
                    kind: DefinitionIssueKind::CircularAcceptedTermReference,
                    severity: DefinitionSeverity::Warning,
                    message: format!(
                        "Definitionen bør ikke indeholde synonymet/den accepterede term '{}' (Regel 20).",
                        syn
                    ),
                    rule: "§20",
                });
            }
        }

        // 5. Vage forbeholdsord (Regler 20 & 21)
        const VAGUE_WORDS: &[&str] = &[
            "typisk",
            "normalt",
            "ofte",
            "som regel",
            "gerne",
            "sædvanligvis",
        ];

        for &vague in VAGUE_WORDS {
            if contains_word(trimmed, vague) {
                issues.push(DefinitionLintIssue {
                    kind: DefinitionIssueKind::VagueWord,
                    severity: DefinitionSeverity::Warning,
                    message: format!(
                        "Karakteristika skal altid gælde; undgå vage forbeholdsord som '{}' (Regel 21).",
                        vague
                    ),
                    rule: "§21",
                });
            }
        }

        // 6. Negative definitioner (Regel 20)
        let is_negative = lower.starts_with("ikke ")
            || lower.starts_with("ikke-")
            || lower.contains(" der ikke ")
            || lower.contains(" som ikke ")
            || lower.contains(" ikke er ")
            || lower.contains(" ikke har ");

        if is_negative {
            issues.push(DefinitionLintIssue {
                kind: DefinitionIssueKind::NegativeDefinition,
                severity: DefinitionSeverity::Warning,
                message: "Definitioner bør være positive og definere hvad begrebet er, frem for hvad det ikke er (Regel 20).".to_string(),
                rule: "§20",
            });
        }

        issues
    }

    pub fn analyze_aristotle(definition: &str) -> AristotleBreakdown {
        let def = definition.trim();
        if def.is_empty() {
            return AristotleBreakdown {
                genus_proximum: None,
                differentia_specifica: None,
            };
        }

        let connectives = [" der ", " som ", " hvor ", " hvori ", " hvis ", " hvormed "];
        let mut earliest_match: Option<(usize, usize)> = None;
        let lower = def.to_lowercase();

        for conn in &connectives {
            if let Some(pos) = lower.find(conn) {
                match earliest_match {
                    None => earliest_match = Some((pos, conn.len())),
                    Some((earliest_pos, _)) if pos < earliest_pos => {
                        earliest_match = Some((pos, conn.len()));
                    }
                    _ => {}
                }
            }
        }

        if let Some((pos, len)) = earliest_match {
            let genus = def[..pos].trim();
            let diff = def[pos + len..].trim();
            AristotleBreakdown {
                genus_proximum: if genus.is_empty() {
                    None
                } else {
                    Some(genus.to_string())
                },
                differentia_specifica: if diff.is_empty() {
                    None
                } else {
                    Some(diff.to_string())
                },
            }
        } else {
            AristotleBreakdown {
                genus_proximum: None,
                differentia_specifica: None,
            }
        }
    }
}

fn contains_word(text: &str, word: &str) -> bool {
    let text_lower = text.to_lowercase();
    let word_lower = word.to_lowercase();
    let word_len = word_lower.len();

    let mut start = 0;
    while let Some(pos) = text_lower[start..].find(&word_lower) {
        let abs_pos = start + pos;
        let before_ok = if abs_pos == 0 {
            true
        } else {
            let prev_char = text_lower[..abs_pos].chars().last().unwrap();
            !prev_char.is_alphanumeric()
        };

        let end_pos = abs_pos + word_len;
        let after_ok = if end_pos >= text_lower.len() {
            true
        } else {
            let next_char = text_lower[end_pos..].chars().next().unwrap();
            !next_char.is_alphanumeric()
        };

        if before_ok && after_ok {
            return true;
        }

        start = abs_pos + 1;
        if start >= text_lower.len() {
            break;
        }
    }
    false
}
