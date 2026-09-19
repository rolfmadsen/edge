---
type: Task Package
title: "Task 003: Projektpersistens, Autosave & Git-Format"
description: "Etablering af deterministisk projektpersistens med autosave ved modelændringer, atomisk skrivning og åbn/gem filhåndtering"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T09:47:36Z" }
tags: [task-lifecycle, intent, scaffolding, rust, persistence, storage, autosave]
---

# Task 003: Projektpersistens, Autosave & Git-Format

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Etablere deterministisk og atomisk filpersistens for FDA modelprojekter (`.edge.json`), automatisk gemning (Autosave) når begreber oprettes, redigeres eller slettes, automatisk indlæsning ved opstart, samt eksplicit "Åbn..." og "Gem som..." filhåndtering.

## 📋 Acceptance Criteria
- [x] `ProjectStorage` i `src/features/model/storage.rs` understøtter deterministisk serialisering (`save_to_file`) og deserialisering (`load_from_file`) med `serde_json` og fuld FDA-validering.
- [x] Atomisk filskrivning i `ProjectStorage` (skrivning til midlertidig fil `.tmp` efterfulgt af atomisk rename) forhindrer filkorruption.
- [x] `App` i `src/ui/app.rs` understøtter automatisk indlæsning af standard projektfil (`model.edge.json`) ved opstart, hvis den findes.
- [x] `App` udfører automatisk gemning (Autosave) til den aktive projektfil, når `ModelProject` muteres (ved oprettelse, redigering og sletning af begreber).
- [x] UI i `src/ui/app.rs` stiller knapper til rådighed for "📁 Åbn...", "💾 Gem", "Gem som..." samt viser diskret gemt-status i statusbaren.
- [x] 100% test pass rate på unit-, persistens- og TEA-accepttests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE overskrive projektfiler med uvaliderede eller syntaktisk ødelagte data (Fail-Closed).
- Må IKKE introducere baggrundsprocesser eller dæmoner jf. Zero-Daemon invarianten.
- Må IKKE foretage remote publication handlinger (`git push`).
- Må IKKE blokere UI eller gå i panik ved fil-IO fejl (fejl håndteres type-sikkert og vises i statusbaren).

## 📝 Revisions
- 2026-09-19: Task oprettet for projektpersistens, autosave og filhåndtering.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 003-project-persistence`
- `xgauntlet verify --task 003-project-persistence`
