---
type: Task Package
title: "Task 020: Harmoniseret Egenskaber- og Vejledningspanel i Begrebs- og Informationsmodel"
description: "Ensretning af højre panel på tværs af Begrebsmodel og Informationsmodel med ensartet navngivning (Egenskaber vs Vejledning), sektionsopbygning og visuelt hierarki"
status: pending
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, ux, inspector, properties, guidance, consistency]
---

# Task 020: Harmoniseret Egenskaber- og Vejledningspanel i Begrebs- og Informationsmodel

**Status**: `PENDING`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Fjerne den kognitive forvirring ved modstridende navngivning i højre sidepanel:
   - I dag: "Studie panel" / "Inspector" (i Begrebsmodel) vs. "UML Klasse inspector" (i Informationsmodel).
2. Etablere en fuldstændig harmoniseret tilgang til højre panel:
   - **Når et element er valgt (Node, Klasse eller Relation)**: Panelet hedder konsekvent **EGENSKABER** (eller "Begrebsegenskaber" / "Klasseegenskaber" med samme header-layout).
   - **Når intet er valgt**: Panelet hedder konsekvent **VEJLEDNING** (eller "Guide / Hjælp") med præcise hjælpetekster tilpasset det pågældende moduleringsfase.
3. Ensrette sektionsopbygningen (f.eks. Generelt, Beskrivelse, Attributter/Egenskaber, Forbindelser) så modellereren genkender strukturen øjeblikkeligt.

## 📋 Acceptance Criteria
- [ ] Titler og header-styling i højre panel er ensartede på tværs af `concept_model_view.rs` og `information_model_view.rs`.
- [ ] Tom tilstand (ingen selektion) viser et rent, velstruktureret **VEJLEDNING** panel med tips til henholdsvis grafmodellering og informationsmodellering.
- [ ] Selektionstilstand viser et velstruktureret **EGENSKABER** panel med klare kortsektioner.
- [ ] Hjælpe- og redigeringsfunktionalitet bevares 100%, men med ensartet typografi og COSMIC-styling.
- [ ] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE fjerne eksisterende redigeringsmuligheder for begreber eller klasser.
- Må IKKE bryde eksisterende widget layout eller scrollable områder.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`
