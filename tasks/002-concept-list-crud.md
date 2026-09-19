---
type: Task Package
title: "Task 002: Begrebsliste Tabel & CRUD jf. FDA Bilag D & E"
description: "Etablering af tabel-komponenten til Begrebsliste med CRUD-funktioner og felterne foretrukken term, definition, kilder og emneområde jf. FDA Modelreglerne"
status: active
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T09:17:51Z" }
tags: [task-lifecycle, intent, scaffolding, rust, concepts, fda, iced]
---

# Task 002: Begrebsliste Tabel & CRUD jf. FDA Bilag D & E

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Etablere en funktionel og æstetisk tiltalende tabel-komponent i Iced til FDA Begrebslisten (Bilag D & E) med fuld CRUD-funktionalitet (opret, vis, rediger, slet), filtrering og understøttelse af felterne: foretrukken term, definition, kilder og emneområde med tilhørende FDA-farvekodning jf. modelleringsvejledningen.

## 📋 Acceptance Criteria
- [ ] `ModelProject` udvides med `concepts: Vec<Concept>` og fuld domæne-CRUD (`add_concept`, `update_concept`, `remove_concept`, `get_concept`) underlagt `ConceptValidator`.
- [ ] `Concept` og `BelongsToDomain` i `src/features/concepts/` understøtter samtlige felter fra FDA Bilag D & E samt hjælpemetoder til domænestatus.
- [ ] UI-tabelkomponenten i `src/ui/concept_table.rs` viser en responsiv og moderne tabel med kolonnerne Foretrukken term, Definition, Kilder, Emneområde (med FDA-badge) og Handlinger.
- [ ] UI-editorkomponenten i `src/ui/concept_editor.rs` tilbyder en struktureret formular til oprettelse og redigering af begreber med valideringsfeedback.
- [ ] `App` i `src/ui/app.rs` understøtter den fulde TEA CRUD-livscyklus, realtidssøgning/filtrering, samt integration med Iceds native `Theme::Light` og FDA-farvepalet.
- [ ] Samtlige accepttests, property-tests, clippy uden advarsler og gauntlet verifikation passerer 100%.

## 🚫 Must NOT
- Må IKKE bryde Zero-Daemon og Zero Ambient Authority principperne.
- Må IKKE sammenblande FDA-valideringslogik direkte i Iced UI widgets (ren domæneseparation).
- Må IKKE tillade gemning af begreber uden obligatoriske felter (foretrukken term og definition).
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for Begrebsliste Tabel & CRUD jf. FDA Bilag D & E.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 002-concept-list-crud`
- `xgauntlet verify --task 002-concept-list-crud`
