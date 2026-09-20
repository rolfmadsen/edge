use kant::features::concepts::{BelongsToDomain, Concept, ConceptValidator, ValidationError};
use kant::features::information_model::Multiplicity;
use proptest::prelude::*;

proptest! {
    #[test]
    fn proptest_concept_validation_accepts_valid_strings(
        term in "[a-zA-Z0-9æøåÆØÅ ]{1,50}",
        def in "[a-zA-Z0-9æøåÆØÅ .,!?-]{1,200}",
    ) {
        let trimmed_term = term.trim();
        let trimmed_def = def.trim();

        let concept = Concept::new(term.clone(), def.clone(), BelongsToDomain::Yes);
        let result = ConceptValidator::validate(&concept);

        if !trimmed_term.is_empty() && !trimmed_def.is_empty() {
            prop_assert!(result.is_ok(), "Forventede Ok for gyldig term og definition");
        } else {
            prop_assert!(result.is_err(), "Forventede Err når term eller def er tom efter trim");
        }
    }

    #[test]
    fn proptest_concept_validation_rejects_empty_term(
        whitespace in "[ \t\n\r]{0,10}",
        def in "[a-zA-Z0-9]{1,50}",
    ) {
        let concept = Concept::new(whitespace, def, BelongsToDomain::Yes);
        let result = ConceptValidator::validate(&concept);
        prop_assert_eq!(result, Err(ValidationError::MissingRequiredField("Foretrukken dansk term")));
    }

    #[test]
    fn proptest_concept_validation_rejects_empty_definition(
        term in "[a-zA-Z0-9]{1,50}",
        whitespace in "[ \t\n\r]{0,10}",
    ) {
        let concept = Concept::new(term, whitespace, BelongsToDomain::Yes);
        let result = ConceptValidator::validate(&concept);
        prop_assert_eq!(result, Err(ValidationError::MissingRequiredField("Definition")));
    }

    #[test]
    fn proptest_multiplicity_display_string(lower in 0u32..100, span in 0u32..50) {
        let upper = lower + span;
        let m = Multiplicity::new(lower, Some(upper));
        let display = m.to_display_string();

        if lower == upper {
            prop_assert_eq!(display, format!("{}", lower));
        } else {
            prop_assert_eq!(display, format!("{}..{}", lower, upper));
        }
    }
}
