use serde::{Deserialize, Serialize};

/// Mål for navnekonventionstjek jf. FDA Modelregel 19
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamingTarget {
    Class,
    Enumeration,
    StructuredDataType,
    Attribute,
    AssociationEnd,
}

/// Autoritative navnekonventioner jf. FDA Modelregel 19
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamingConvention {
    UpperCamelCase,
    LowerCamelCase,
}

/// Specifik årsag til at et navn ikke overholder FDA Modelregel 19
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamingIssueKind {
    EmptyName,
    NotUpperCamelCase,
    NotLowerCamelCase,
    ContainsDisallowedCharacters,
    ContainsWhitespace,
    ContainsUnderscore,
    ContainsHyphen,
    LeadingDigit,
}

/// Alvorlighedsgrad for en navneadvarsel (altid ikke-blokerende for persistering)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamingSeverity {
    Warning,
    Suggestion,
}

/// En observeret uoverensstemmelse med FDA Modelregel 19
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingIssue {
    pub target: NamingTarget,
    pub convention: NamingConvention,
    pub kind: NamingIssueKind,
    pub severity: NamingSeverity,
    pub message: String,
    pub rule: &'static str,
    pub suggested_fix: Option<String>,
}

pub struct NamingLinter;

pub fn is_upper_camel_case(s: &str) -> bool {
    NamingLinter::is_upper_camel_case(s)
}

pub fn is_lower_camel_case(s: &str) -> bool {
    NamingLinter::is_lower_camel_case(s)
}

impl NamingLinter {
    pub const RULE_FDA_19_CLASS: &'static str = "FDA Modelregel 19 (§19.1)";
    pub const RULE_FDA_19_ATTR: &'static str = "FDA Modelregel 19 (§19.2)";
    pub const RULE_FDA_TABEL_B: &'static str = "FDA Tabel B";

    /// Validerer om en streng opfylder UpperCamelCase (PascalCase):
    /// - Må ikke være tom.
    /// - Første tegn skal være stort bogstav (`c.is_uppercase()`).
    /// - Alle tegn skal være alfanumeriske (bogstaver inkl. æ, ø, å og tal).
    /// - Ingen mellemrum, understreger, bindestreger eller specialtegn.
    pub fn is_upper_camel_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut chars = s.chars();
        let first = match chars.next() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_uppercase() {
            return false;
        }
        for c in chars {
            if !c.is_alphanumeric() {
                return false;
            }
        }
        true
    }

    /// Validerer om en streng opfylder lowerCamelCase:
    /// - Må ikke være tom.
    /// - Første tegn skal være lille bogstav (`c.is_lowercase()`).
    /// - Alle tegn skal være alfanumeriske (bogstaver inkl. æ, ø, å og tal).
    /// - Ingen mellemrum, understreger, bindestreger eller specialtegn.
    pub fn is_lower_camel_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut chars = s.chars();
        let first = match chars.next() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_lowercase() {
            return false;
        }
        for c in chars {
            if !c.is_alphanumeric() {
                return false;
            }
        }
        true
    }

    /// Genererer et forslag til et gyldigt UpperCamelCase-navn baseret på input
    pub fn suggest_upper_camel_case(s: &str) -> String {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let parts: Vec<&str> = trimmed
            .split(|c: char| !c.is_alphanumeric())
            .filter(|p| !p.is_empty())
            .collect();

        if parts.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        for part in parts {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                for uc in first.to_uppercase() {
                    result.push(uc);
                }
                for rem in chars {
                    result.push(rem);
                }
            }
        }
        result
    }

    /// Genererer et forslag til et gyldigt lowerCamelCase-navn baseret på input
    pub fn suggest_lower_camel_case(s: &str) -> String {
        let upper = Self::suggest_upper_camel_case(s);
        if upper.is_empty() {
            return String::new();
        }
        let mut chars = upper.chars();
        let mut result = String::new();
        if let Some(first) = chars.next() {
            for lc in first.to_lowercase() {
                result.push(lc);
            }
            for rem in chars {
                result.push(rem);
            }
        }
        result
    }

    /// Tjekker et klassenavn jf. FDA Modelregel 19
    pub fn check_class_name(name: &str) -> Option<NamingIssue> {
        Self::check_upper_camel_case(
            name,
            NamingTarget::Class,
            "Klassenavn",
            Self::RULE_FDA_19_CLASS,
        )
    }

    /// Tjekker et enumerationsnavn jf. FDA Modelregel 19
    pub fn check_enumeration_name(name: &str) -> Option<NamingIssue> {
        Self::check_upper_camel_case(
            name,
            NamingTarget::Enumeration,
            "Enumerationsnavn",
            Self::RULE_FDA_19_CLASS,
        )
    }

    /// Tjekker et struktureret datatype-navn jf. FDA Modelregel 19
    pub fn check_structured_type_name(name: &str) -> Option<NamingIssue> {
        Self::check_upper_camel_case(
            name,
            NamingTarget::StructuredDataType,
            "Datatypenavn",
            Self::RULE_FDA_19_CLASS,
        )
    }

    fn check_upper_camel_case(
        name: &str,
        target: NamingTarget,
        entity_label: &str,
        rule: &'static str,
    ) -> Option<NamingIssue> {
        if Self::is_upper_camel_case(name) {
            return None;
        }

        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Some(NamingIssue {
                target,
                convention: NamingConvention::UpperCamelCase,
                kind: NamingIssueKind::EmptyName,
                severity: NamingSeverity::Warning,
                message: format!("{} må ikke være tomt.", entity_label),
                rule,
                suggested_fix: None,
            });
        }

        let kind = if name.contains(char::is_whitespace) {
            NamingIssueKind::ContainsWhitespace
        } else if name.contains('_') {
            NamingIssueKind::ContainsUnderscore
        } else if name.contains('-') {
            NamingIssueKind::ContainsHyphen
        } else if name.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            NamingIssueKind::LeadingDigit
        } else if name
            .chars()
            .next()
            .map(|c| c.is_lowercase())
            .unwrap_or(false)
        {
            NamingIssueKind::NotUpperCamelCase
        } else if name.chars().any(|c| !c.is_alphanumeric()) {
            NamingIssueKind::ContainsDisallowedCharacters
        } else {
            NamingIssueKind::NotUpperCamelCase
        };

        let suggested_fix = {
            let fix = Self::suggest_upper_camel_case(name);
            if fix.is_empty() || fix == name {
                None
            } else {
                Some(fix)
            }
        };

        Some(NamingIssue {
            target,
            convention: NamingConvention::UpperCamelCase,
            kind,
            severity: NamingSeverity::Warning,
            message: format!(
                "{}et '{}' overholder ikke UpperCamelCase (FDA Modelregel 19).",
                entity_label, name
            ),
            rule,
            suggested_fix,
        })
    }

    /// Tjekker et attributnavn jf. FDA Modelregel 19
    pub fn check_attribute_name(name: &str) -> Option<NamingIssue> {
        if Self::is_lower_camel_case(name) {
            return None;
        }

        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Some(NamingIssue {
                target: NamingTarget::Attribute,
                convention: NamingConvention::LowerCamelCase,
                kind: NamingIssueKind::EmptyName,
                severity: NamingSeverity::Warning,
                message: "Attributnavn må ikke være tomt.".to_string(),
                rule: Self::RULE_FDA_19_ATTR,
                suggested_fix: None,
            });
        }

        let kind = if name.contains(char::is_whitespace) {
            NamingIssueKind::ContainsWhitespace
        } else if name.contains('_') {
            NamingIssueKind::ContainsUnderscore
        } else if name.contains('-') {
            NamingIssueKind::ContainsHyphen
        } else if name.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            NamingIssueKind::LeadingDigit
        } else if name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            NamingIssueKind::NotLowerCamelCase
        } else if name.chars().any(|c| !c.is_alphanumeric()) {
            NamingIssueKind::ContainsDisallowedCharacters
        } else {
            NamingIssueKind::NotLowerCamelCase
        };

        let suggested_fix = {
            let fix = Self::suggest_lower_camel_case(name);
            if fix.is_empty() || fix == name {
                None
            } else {
                Some(fix)
            }
        };

        Some(NamingIssue {
            target: NamingTarget::Attribute,
            convention: NamingConvention::LowerCamelCase,
            kind,
            severity: NamingSeverity::Warning,
            message: format!(
                "Attributnavnet '{}' overholder ikke lowerCamelCase (FDA Modelregel 19).",
                name
            ),
            rule: Self::RULE_FDA_19_ATTR,
            suggested_fix,
        })
    }

    /// Tjekker en enumerationsværdi jf. FDA Tabel B (skal være lowerCamelCase)
    pub fn check_enumeration_value(value: &str) -> Option<NamingIssue> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Some(NamingIssue {
                target: NamingTarget::Enumeration,
                convention: NamingConvention::LowerCamelCase,
                kind: NamingIssueKind::EmptyName,
                severity: NamingSeverity::Warning,
                message: "Enumerationsværdi må ikke være tom.".to_string(),
                rule: Self::RULE_FDA_TABEL_B,
                suggested_fix: None,
            });
        }

        if Self::is_lower_camel_case(trimmed) {
            return None;
        }

        let kind = if value.contains(char::is_whitespace) {
            NamingIssueKind::ContainsWhitespace
        } else if value.contains('_') {
            NamingIssueKind::ContainsUnderscore
        } else if value.contains('-') {
            NamingIssueKind::ContainsHyphen
        } else if value
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false)
        {
            NamingIssueKind::LeadingDigit
        } else if value
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            NamingIssueKind::NotLowerCamelCase
        } else if value.chars().any(|c| !c.is_alphanumeric()) {
            NamingIssueKind::ContainsDisallowedCharacters
        } else {
            NamingIssueKind::NotLowerCamelCase
        };

        let suggested_fix = {
            let fix = Self::suggest_lower_camel_case(value);
            if fix.is_empty() || fix == value {
                None
            } else {
                Some(fix)
            }
        };

        Some(NamingIssue {
            target: NamingTarget::Enumeration,
            convention: NamingConvention::LowerCamelCase,
            kind,
            severity: NamingSeverity::Warning,
            message: format!(
                "Enumerationsværdien '{}' overholder ikke lowerCamelCase (FDA Tabel B).",
                value
            ),
            rule: Self::RULE_FDA_TABEL_B,
            suggested_fix,
        })
    }

    /// Tjekker en associationslabel (valgfri, men hvis sat skal den være lowerCamelCase)
    pub fn check_association_label(label: &str) -> Option<NamingIssue> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            return None; // Valgfri ifølge UML og FDA
        }

        if Self::is_lower_camel_case(trimmed) {
            return None;
        }

        let kind = if label.contains(char::is_whitespace) {
            NamingIssueKind::ContainsWhitespace
        } else if label.contains('_') {
            NamingIssueKind::ContainsUnderscore
        } else if label.contains('-') {
            NamingIssueKind::ContainsHyphen
        } else if label
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false)
        {
            NamingIssueKind::LeadingDigit
        } else if label
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            NamingIssueKind::NotLowerCamelCase
        } else if label.chars().any(|c| !c.is_alphanumeric()) {
            NamingIssueKind::ContainsDisallowedCharacters
        } else {
            NamingIssueKind::NotLowerCamelCase
        };

        let suggested_fix = {
            let fix = Self::suggest_lower_camel_case(label);
            if fix.is_empty() || fix == label {
                None
            } else {
                Some(fix)
            }
        };

        Some(NamingIssue {
            target: NamingTarget::AssociationEnd,
            convention: NamingConvention::LowerCamelCase,
            kind,
            severity: NamingSeverity::Warning,
            message: format!(
                "Associationsnavnet '{}' overholder ikke lowerCamelCase (FDA Modelregel 19).",
                label
            ),
            rule: Self::RULE_FDA_19_ATTR,
            suggested_fix,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_camel_case_validation() {
        assert!(NamingLinter::is_upper_camel_case("EthjuletCykel"));
        assert!(NamingLinter::is_upper_camel_case("Køretøj"));
        assert!(NamingLinter::is_upper_camel_case("ÆbleTræ"));
        assert!(NamingLinter::is_upper_camel_case("ÅrligRapport"));
        assert!(NamingLinter::is_upper_camel_case("CPR"));
        assert!(NamingLinter::is_upper_camel_case("Klasse1"));

        assert!(!NamingLinter::is_upper_camel_case(""));
        assert!(!NamingLinter::is_upper_camel_case("ethjulet_cykel"));
        assert!(!NamingLinter::is_upper_camel_case("Ethjulet Cykel"));
        assert!(!NamingLinter::is_upper_camel_case("cykel"));
        assert!(!NamingLinter::is_upper_camel_case("1Klasse"));
        assert!(!NamingLinter::is_upper_camel_case("Klasse!"));
    }

    #[test]
    fn test_lower_camel_case_validation() {
        assert!(NamingLinter::is_lower_camel_case("stelnummer"));
        assert!(NamingLinter::is_lower_camel_case("maxPassagerer"));
        assert!(NamingLinter::is_lower_camel_case("førsteRegistrering"));
        assert!(NamingLinter::is_lower_camel_case("cvrNummer"));
        assert!(NamingLinter::is_lower_camel_case("køretøjsIdentifikator"));

        assert!(!NamingLinter::is_lower_camel_case(""));
        assert!(!NamingLinter::is_lower_camel_case("Stelnummer"));
        assert!(!NamingLinter::is_lower_camel_case("stel_nummer"));
        assert!(!NamingLinter::is_lower_camel_case("max passagerer"));
        assert!(!NamingLinter::is_lower_camel_case("1stel"));
        assert!(!NamingLinter::is_lower_camel_case("stel-nummer"));
    }

    #[test]
    fn test_suggestions() {
        assert_eq!(
            NamingLinter::suggest_upper_camel_case("ethjulet_cykel"),
            "EthjuletCykel"
        );
        assert_eq!(
            NamingLinter::suggest_upper_camel_case("ethjulet cykel"),
            "EthjuletCykel"
        );
        assert_eq!(NamingLinter::suggest_upper_camel_case("cykel"), "Cykel");

        assert_eq!(
            NamingLinter::suggest_lower_camel_case("Stelnummer"),
            "stelnummer"
        );
        assert_eq!(
            NamingLinter::suggest_lower_camel_case("stel_nummer"),
            "stelNummer"
        );
        assert_eq!(
            NamingLinter::suggest_lower_camel_case("max passagerer"),
            "maxPassagerer"
        );
        assert_eq!(
            NamingLinter::suggest_lower_camel_case("Omfatter"),
            "omfatter"
        );
        assert_eq!(
            NamingLinter::suggest_lower_camel_case("omfatter_del"),
            "omfatterDel"
        );
    }

    #[test]
    fn test_check_methods() {
        assert!(NamingLinter::check_class_name("Cykel").is_none());
        let class_issue = NamingLinter::check_class_name("cykel").unwrap();
        assert_eq!(class_issue.target, NamingTarget::Class);
        assert_eq!(class_issue.suggested_fix.as_deref(), Some("Cykel"));

        assert!(NamingLinter::check_attribute_name("stelnummer").is_none());
        let attr_issue = NamingLinter::check_attribute_name("Stelnummer").unwrap();
        assert_eq!(attr_issue.target, NamingTarget::Attribute);
        assert_eq!(attr_issue.suggested_fix.as_deref(), Some("stelnummer"));

        assert!(NamingLinter::check_association_label("").is_none());
        assert!(NamingLinter::check_association_label("ejer").is_none());
        let assoc_issue = NamingLinter::check_association_label("Ejer").unwrap();
        assert_eq!(assoc_issue.target, NamingTarget::AssociationEnd);
        assert_eq!(assoc_issue.suggested_fix.as_deref(), Some("ejer"));
    }
}
