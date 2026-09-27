---
type: Task Package
title: "Task 044: Dekomponeret Model Storage Engine (.kant/) & Deterministisk Serialisering"
description: "Dekomponering af FDA modelprojekter i fast mappestruktur (.kant/) med deterministisk JSON-sortering for eliminering af Git-mergekonflikter og fuld bagudkompatibilitet"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:10:00Z" }
tags: [storage, decomposed-format, json, determinism, persistence, git, serialization]
---

# Task 044: Dekomponeret Model Storage Engine (.kant/) & Deterministisk Serialisering

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/features/model/storage.rs`, `src/features/model/decomposed.rs`, `src/features/model/mod.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Gøre det muligt for Kant at gemme og indlæse et FDA modelprojekt i et dekomponeret, deterministisk format inspireret af GRAFICO i stedet for udelukkende én monolitisk fil (`model.kant.json`). Dette eliminerer op mod 95 % af alle potentielle merge-konflikter ved versionsstyring og samarbejde:

1. **Dekomponeret Filstruktur (`.kant/`)**:
   ```text
   <project-root>/
     ├── .kant/
     │   ├── metadata.json          # Titel, version, beskrivelse, forfatter, status, domæne
     │   ├── concepts/              # Forretningsbegreber (1 fil pr. begreb: <uuid>.json)
     │   ├── classes/               # Informationsklasser (1 fil pr. klasse: <uuid>.json)
     │   ├── relations/             # Relationer og associationer (1 fil pr. relation: <uuid>.json)
     │   └── diagrams/              # Diagram-layouts, koordinater og porte
     └── model.kant.json            # (Valgfrit standalone eksport-snapshot)
   ```
2. **Deterministisk Serialisering**:
   - JSON-objektnøgler sorteres alfabetisk.
   - Arrays med under-entiteter (fx attributter i en klasse eller noder/kanter) sorteres deterministisk.
   - Formatering med 2 mellemrum og en afsluttende newline (`\n`).
   - Garanterer 100% idempotent output: at gemme modellen to gange i træk uden ændringer genererer et tomt `git diff`.
3. **Transparent Indlæsning & Synkroniseret Oprydning**:
   - `ProjectStorage` kan indlæse både monolitiske filer og dekomponerede kataloger.
   - Ved genskrivning af et opdateret projekt til `.kant/` fjernes filer for entiteter, der er slettet fra modellen, så der ikke opstår zombie-filer.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Dekomponeret Skrivning (.kant/)**: `ProjectStorage::save_to_directory(project, root_path)` gemmer projektet i `.kant/` med undermapperne `concepts/`, `classes/`, `relations/` og `diagrams/` samt `metadata.json`.
- [ ] **AC2 - Deterministisk JSON Formatering**: Alle JSON-filer skrives med alfabetisk sorterede nøgler og deterministisk sorterede arrays. Idempotente gen-skrivninger producerer identiske filer bit-for-bit.
- [ ] **AC3 - Fuld Dekomponeret Indlæsning & Roundtrip**: `ProjectStorage::load_from_directory(root_path)` indlæser alle entiteter og diagrammer og genskaber et komplet og semantisk ækvivalent `ModelProject`.
- [ ] **AC4 - Transparent Formatdetektion**: `ProjectStorage::load(path)` kan automatisk detektere om stien peger på en `.kant.json` fil, en `.kant/` mappe eller en projektmappe indeholdende `.kant/`, og indlæse korrekt.
- [ ] **AC5 - Synkroniseret Oprydning af Slettede Elementer**: Når et begreb, en klasse eller en relation er slettet fra modellen, slettes dens tilsvarende `<uuid>.json` fil fra disk ved næste `save_to_directory`.
- [ ] **AC6 - Fail-Closed Validering & Atomicitet**: Samtlige begreber valideres før skrivning og ved indlæsning. Ugyldige data afvises uden at efterlade korrupt tilstand.
- [ ] **AC7 - Verifikation via Accepttest**: `test_task_044_decomposed_storage_and_deterministic_serialization` beviser at dekomponering, roundtrip, slette-oprydning og determinisme fungerer fejlfrit.

---

## 🚫 Must NOT
- Må IKKE bryde bagudkompatibilitet med monolitiske `.kant.json` filer.
- Må IKKE skrive vilkårligt sorterede nøgler eller arrays, der skaber git diff støj.
- Må IKKE efterlade slettede entiteter som zombie-filer i `.kant/`.
- Må IKKE omgå FDA valideringsregler ved gem/indlæs.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 044 efter arkitektur-sparring.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_044`
- `cargo clippy --all-targets`
- `cargo fmt --check`
