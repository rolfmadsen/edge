---
type: Task Package
title: "Task 056: Standard UML Stereotypes & Concept Canvas Refinement"
description: "Opdatering af UML-rendering til den officielle FDA stereotype «Concept», fjernelse af uofficielle badges og opstramning af begrebsrelationer"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, uml, stereotype, concept-model, canvas, rendering]
---

# Task 056: Standard UML Stereotypes & Concept Canvas Refinement

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Ensrette Kants visuelle UML-udtryk med de fællesoffentlige modelregler (FDA v2.1.0, Kapitel 5.1–5.2 og Kapitel 7):
1. **Regelsat UML-Stereotype (`«Concept»`)**:
   - Erstatte uofficielle badges på diagramlærredet (`«lokalt begreb»`, `«fremmed begreb»` og `«Klasse»`) med den formelle FDA stereotype **`«Concept»`** jf. Regel 03.
   - Adskillelse mellem lokalt og indlånt begreb formidles via FDA-farverne (sandfarvet `#FEFAF7` vs. blå `#87CDEB`) og metadata-tagget `isDefinedBy` (eller kolonne 10 i begrebslisten) i overensstemmelse med Kapitel 7.3.
2. **Korrekt Anvendelse af Begrebsrelationer (Tabel A vs. Tabel B)**:
   - Fjerne `Composition` fra Begrebsmodel-lærredet. Jf. FDA Tabel B og afsnit 5.2.4 er komposition udelukkende tilladt i anvendelsesmodeller (informations- og logiske datamodeller).
   - Begræns begrebsrelationer til: Generalisering (lukket hvid trekant) og Association (linje med læseretning/pil og verbalfrase i nutid).
3. **Navnekonventioner (Regel 19)**:
   - Sikre visning af navne i naturligt sprog med lille begyndelsesbogstav for begreber og relationer på begrebslærredet.
   - Forberede linter for `UpperCamelCase` på informationsklasser og `lowerCamelCase` på attributter og associationsnavne.

## 📋 Acceptance Criteria
- [ ] **AC1 - Stereotype «Concept» på Diagramlærred**: Samtlige begrebskasser og informationsklasser renderer med den officielle stereotype `«Concept»` i stedet for `«lokalt begreb»` eller `«Klasse»`.
- [ ] **AC2 - Ren Begrebsmodel Toolbar**: Værktøjslinjen og relation-vælgeren på Begrebsmodel-fanen tillader kun Generalisering og Association (Komposition er deaktiveret/skjult).
- [ ] **AC3 - Visuel Adskillelse Uden Uofficielle Badges**: Forskellen mellem lokalt og fremmed begreb vises entydigt via sand vs. blå baggrund og border, uden tekstmæssig badge-støj.
- [ ] **AC4 - Bevaret Komposition i Informationsmodellen**: Komposition forbliver fuldt funktionsdygtig i Informationsmodel-fanen jf. FDA Tabel B.
- [ ] **AC5 - Regressionstests & Canvas Render Verifikation**: Alle eksisterende acceptancetests og canvas rendering-tests forbliver grønne.

## 🚫 Must NOT
- Må IKKE ødelægge gemte diagrammer eller relationer i eksisterende projekter.
- Må IKKE forringe performance af Iced-lærredets 60 fps rendering.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke til oprydning i UML-stereotyper og overholdelse af Tabel A/B.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
