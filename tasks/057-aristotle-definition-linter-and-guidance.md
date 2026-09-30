---
type: Task Package
title: "Task 057: Aristotle Definition Linter & Guidance"
description: "Semantisk linter for begrebsdefinitioner efter Aristoteles' formel (genus proximum og differentia specifica) med realtidsvejledning i editoren"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, concepts, aristotle, linter, validation, guidance, definitions]
---

# Task 057: Aristotle Definition Linter & Guidance

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Understøtte udarbejdelsen af strukturerede definitioner jf. FDA Modelreglerne (Kapitel 3, Regler 20, 21 og 22):
1. **Aristoteles' Formel (*definitio per genus et differentiam*)**:
   - Analysere definitioner for at identificere nærmeste overbegreb og adskillende træk.
2. **Formelle FDA Valideringsregler**:
   - **Lille begyndelsesbogstav**: Definitionen skal kunne erstatte termen direkte i en sætning uden tab af mening (ISO 704).
   - **Intet afsluttende punktum**: Definitionen er en frase, ikke en afsluttet helsætning.
   - **Ikke-cirkulær**: Den foretrukne term (eller synonymer) må IKKE indgå i definitionen.
   - **Forbud mod tomme fyldfraser**: Fange indledninger som *"er en"*, *"defineres som"*, *"betyder"*, *"henvisning til"*, *"angivelse af"*.
   - **Forbud mod vage ord**: Advare mod forbeholdsord som *"typisk"*, *"normalt"*, *"ofte"*, *"som regel"*.
   - **Ikke-negativ**: Advare hvis definitionen primært beskriver hvad tingen *ikke* er.
3. **UI Integration & Brugerfeedback**:
   - Indbygge linteren direkte i `ConceptValidator` samt vise inline statusbadges og hjælpetekster i `ConceptEditor` og `ConceptTable`.

## 📋 Acceptance Criteria
- [ ] **AC1 - Aristotle Linter Engine**: En ren modulær hjælpefunktion/struktur `DefinitionLinter::lint(&Concept)` evaluerer definitionen mod FDA-tjeklisten (afsnit 3.4.5).
- [ ] **AC2 - Påvisning af Formateringsfejl**: Linteren fanger og rapporterer advarsel ved: stort begyndelsesbogstav, afsluttende punktum, samt forbudte fraser ("er en", "defineres som").
- [ ] **AC3 - Cirkularitetsdetektering**: Hvis den foretrukne term indgår ordret i definitionen, udstedes en advarsel.
- [ ] **AC4 - Inline UI Feedback**: `ConceptEditor` viser feedback-ikoner og informative tooltip/hjælpebeskeder, så brugeren guides til at skrive FDA-korrekte definitioner.
- [ ] **AC5 - Ikke-blokerende for eksisterende data**: Eksisterende begreber med ældre fraseringer kan stadig gemmes og indlæses (advarsler er vejledende og blokerer ikke filgemning).

## 🚫 Must NOT
- Må IKKE afbryde tastaturinput eller forårsage forsinkelser ved indtastning i tekstfelter.
- Må IKKE afvise valide eksisterende begreber under parsing.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke efter gennemgang af fda_modelleringsvejledning.md Kapitel 3.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
