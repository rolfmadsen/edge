# Verification Report

**Task ID**: `020-harmonized-inspector-and-guidance-panels`  
**Task Title**: Task 020: Harmoniseret Egenskaber- og Vejledningspanel i Begrebs- og Informationsmodel  
**Verdict**: `PASSED`  
**Execution Origin**: `LOCAL`  
**Timestamp**: `2026-09-20T10:35:00Z`  
**Head**: `fe0e3e6`  
**Commit**: `fe0e3e6`  

## Acceptance Criteria

- [x] Titler og header-styling i højre panel er ensartede på tværs af `concept_model_view.rs` og `information_model_view.rs`.
- [x] Tom tilstand (ingen selektion) viser et rent, velstruktureret **VEJLEDNING** panel med tips til henholdsvis grafmodellering og informationsmodellering.
- [x] Selektionstilstand viser et velstruktureret **EGENSKABER** panel med klare kortsektioner.
- [x] Hjælpe- og redigeringsfunktionalitet bevares 100%, men med ensartet typografi og COSMIC-styling.
- [x] 100% test pass rate på `cargo test` og clippy uden advarsler.

---

## Verification Checks

| Check Name | Status | Exit Code |
|---|---|---|
| `lint` (`cargo clippy --all-targets -- -D warnings`) | `PASSED` | `0` |
| `fmt` (`cargo fmt --check`) | `PASSED` | `0` |
| `tests` (`cargo test --workspace`) | `PASSED` | `0` |
| `check` (`cargo check --all-targets`) | `PASSED` | `0` |

---
