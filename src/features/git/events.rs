use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeAction {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainChangeEvent {
    pub action: ChangeAction,
    pub entity_type: String,
    pub entity_name: String,
    pub entity_id: Option<Uuid>,
    pub details: String,
}

pub struct DomainEventMapper;

impl DomainEventMapper {
    /// Oversætter en filændring i `.kant/` formatet til en meningsfuld domænehændelse.
    pub fn map_file_change(
        action: ChangeAction,
        file_path: &str,
        file_content: Option<&str>,
    ) -> Option<DomainChangeEvent> {
        let normalized = file_path.trim_start_matches("./").trim_start_matches('/');

        let path = if let Some(stripped) = normalized.strip_prefix(".kant/") {
            stripped
        } else if normalized == ".kant" || normalized.starts_with(".kant") {
            normalized
        } else {
            return None;
        };

        let (entity_type, entity_id, entity_name) =
            if path == "metadata.json" || path.ends_with("/metadata.json") {
                let name = file_content
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(c).ok())
                    .and_then(|v| {
                        v.get("name")
                            .and_then(|n| n.as_str())
                            .or_else(|| v.get("title").and_then(|t| t.as_str()))
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "Modelmetadata".to_string());
                ("Metadata".to_string(), None, name)
            } else if let Some(rest) = path.strip_prefix("concepts/") {
                let id = rest
                    .strip_suffix(".json")
                    .and_then(|s| Uuid::parse_str(s).ok());
                let name = file_content
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(c).ok())
                    .and_then(|v| {
                        v.get("preferred_term")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| {
                        id.map(|u| u.to_string())
                            .unwrap_or_else(|| "Begreb".to_string())
                    });
                ("Begreb".to_string(), id, name)
            } else if let Some(rest) = path.strip_prefix("classes/") {
                let id = rest
                    .strip_suffix(".json")
                    .and_then(|s| Uuid::parse_str(s).ok());
                let name = file_content
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(c).ok())
                    .and_then(|v| {
                        v.get("name")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| {
                        id.map(|u| u.to_string())
                            .unwrap_or_else(|| "Klasse".to_string())
                    });
                ("Klasse".to_string(), id, name)
            } else if let Some(rest) = path.strip_prefix("relations/") {
                let id = rest
                    .strip_suffix(".json")
                    .and_then(|s| Uuid::parse_str(s).ok());
                let name = file_content
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(c).ok())
                    .and_then(|v| {
                        v.get("label")
                            .and_then(|l| l.as_str())
                            .map(|s| s.to_string())
                            .or_else(|| {
                                v.get("relation_type")
                                    .and_then(|t| t.as_str())
                                    .map(|s| s.to_string())
                            })
                    })
                    .unwrap_or_else(|| {
                        id.map(|u| u.to_string())
                            .unwrap_or_else(|| "Relation".to_string())
                    });
                ("Relation".to_string(), id, name)
            } else {
                let rest = path.strip_prefix("diagrams/")?;
                let id = rest
                    .strip_suffix(".json")
                    .and_then(|s| Uuid::parse_str(s).ok());
                let name = file_content
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(c).ok())
                    .and_then(|v| {
                        v.get("name")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| {
                        id.map(|u| u.to_string())
                            .unwrap_or_else(|| "Diagram".to_string())
                    });
                ("Diagram".to_string(), id, name)
            };

        let details = match action {
            ChangeAction::Added => {
                format!("Oprettet {} '{}'", entity_type.to_lowercase(), entity_name)
            }
            ChangeAction::Modified => {
                format!("Opdateret {} '{}'", entity_type.to_lowercase(), entity_name)
            }
            ChangeAction::Deleted => {
                format!("Slettet {} '{}'", entity_type.to_lowercase(), entity_name)
            }
        };

        Some(DomainChangeEvent {
            action,
            entity_type,
            entity_name,
            entity_id,
            details,
        })
    }

    pub fn generate_commit_summary(events: &[DomainChangeEvent]) -> String {
        generate_commit_summary(events)
    }
}

fn format_action_group(
    action_verb: &str,
    begreber: usize,
    klasser: usize,
    relationer: usize,
    diagrammer: usize,
    metadata: bool,
) -> Option<String> {
    let mut parts = Vec::new();
    if begreber > 0 {
        parts.push(format!(
            "{} {}",
            begreber,
            if begreber == 1 { "begreb" } else { "begreber" }
        ));
    }
    if klasser > 0 {
        parts.push(format!(
            "{} {}",
            klasser,
            if klasser == 1 { "klasse" } else { "klasser" }
        ));
    }
    if relationer > 0 {
        parts.push(format!(
            "{} {}",
            relationer,
            if relationer == 1 {
                "relation"
            } else {
                "relationer"
            }
        ));
    }
    if diagrammer > 0 {
        parts.push(format!(
            "{} {}",
            diagrammer,
            if diagrammer == 1 {
                "diagram"
            } else {
                "diagrammer"
            }
        ));
    }
    if metadata {
        parts.push("modelmetadata".to_string());
    }

    if parts.is_empty() {
        None
    } else {
        Some(format!("{} {}", action_verb, parts.join(", ")))
    }
}

/// Genererer en forretnings- og domæneorienteret dansk commit-opsummering.
pub fn generate_commit_summary(events: &[DomainChangeEvent]) -> String {
    let mut added_b = 0;
    let mut added_k = 0;
    let mut added_r = 0;
    let mut added_d = 0;
    let mut added_m = false;

    let mut mod_b = 0;
    let mut mod_k = 0;
    let mut mod_r = 0;
    let mut mod_d = 0;
    let mut mod_m = false;

    let mut del_b = 0;
    let mut del_k = 0;
    let mut del_r = 0;
    let mut del_d = 0;

    for ev in events {
        match ev.action {
            ChangeAction::Added => match ev.entity_type.as_str() {
                "Begreb" => added_b += 1,
                "Klasse" => added_k += 1,
                "Relation" => added_r += 1,
                "Diagram" => added_d += 1,
                "Metadata" => added_m = true,
                _ => {}
            },
            ChangeAction::Modified => match ev.entity_type.as_str() {
                "Begreb" => mod_b += 1,
                "Klasse" => mod_k += 1,
                "Relation" => mod_r += 1,
                "Diagram" => mod_d += 1,
                "Metadata" => mod_m = true,
                _ => {}
            },
            ChangeAction::Deleted => match ev.entity_type.as_str() {
                "Begreb" => del_b += 1,
                "Klasse" => del_k += 1,
                "Relation" => del_r += 1,
                "Diagram" => del_d += 1,
                _ => {}
            },
        }
    }

    let mut phrases = Vec::new();
    if let Some(s) = format_action_group("Oprettet", added_b, added_k, added_r, added_d, added_m) {
        phrases.push(s);
    }
    if let Some(s) = format_action_group("Opdateret", mod_b, mod_k, mod_r, mod_d, mod_m) {
        phrases.push(s);
    }
    if let Some(s) = format_action_group("Fjernet", del_b, del_k, del_r, del_d, false) {
        phrases.push(s);
    }

    if phrases.is_empty() {
        "Opdatering af model".to_string()
    } else {
        let mut result = String::new();
        for (i, phrase) in phrases.iter().enumerate() {
            if i == 0 {
                result.push_str(phrase);
            } else {
                result.push_str(", ");
                let mut chars = phrase.chars();
                if let Some(first) = chars.next() {
                    for c in first.to_lowercase() {
                        result.push(c);
                    }
                    result.push_str(chars.as_str());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_commit_summary() {
        let events = vec![
            DomainChangeEvent {
                action: ChangeAction::Added,
                entity_type: "Klasse".into(),
                entity_name: "Køretøj".into(),
                entity_id: None,
                details: "".into(),
            },
            DomainChangeEvent {
                action: ChangeAction::Modified,
                entity_type: "Begreb".into(),
                entity_name: "Køretøj".into(),
                entity_id: None,
                details: "".into(),
            },
        ];

        let summary = generate_commit_summary(&events);
        assert_eq!(summary, "Oprettet 1 klasse, opdateret 1 begreb");
    }
}
