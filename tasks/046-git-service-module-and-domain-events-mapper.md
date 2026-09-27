---
type: Task Package
title: "Task 046: Git Service Modul & Domænehændelses-Mapper (Backend)"
description: "Ikke-blokerende Git backend service med understøttelse af init, status, commit, pull med semantisk merge og oversættelse af commits til forretningsorienterede modelhændelser"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:22:00Z" }
tags: [git, backend, domain-events, changelog, audit, history, workflows]
---

# Task 046: Git Service Modul & Domænehændelses-Mapper (Backend)

**Status**: `DONE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/features/git/mod.rs`, `src/features/git/service.rs`, `src/features/git/events.rs`, `src/features/mod.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Implementere en selvstændig, ikke-blokerende Git backend-service (`src/features/git/`), der varetager alle Git-arbejdsgange (Init, Status, Udgiv/Commit, Hent/Pull) og oversætter rå Git-filændringer til meningsfulde, forretningsorienterede modelhændelser:

1. **Git Service & Statuskontrol**:
   - `RepoSyncStatus`: Beregner præcis tilstand mellem lokal model og remote (Synkroniseret, Uudgivne ændringer, Nye ændringer på server, Divergeret, Uinitialiseret).
   - `init_repository`: Opretter og initialiserer et Git-lager i modelmappen.
   - `publish_model`: Gemmer til `.kant/`, stager ændringer og opretter commit med brugerens note (eller auto-genereret resumé).
   - `pull_model`: Henter seneste ændringer fra serveren; hvis divergeret, udføres semantisk 3-vejs merge (`merge_models`) frem for linjebaseret Git-merge.
2. **Domænehændelses-Mapper (Commits -> Model Events)**:
   - Oversætter commits og diffs under `.kant/` til semantiske hændelser:
     - 🟢 **Tilføjet**: Begreb, Klasse eller Relation
     - 🟡 **Ændret**: Begreb, Klasse, Relation, Diagramlayout eller Metadata
     - 🔴 **Fjernet**: Begreb, Klasse eller Relation
   - Automatisk versionsnote-generator: Fx *"Oprettet 2 begreber, ændret 1 relation"*.
3. **Element-specifik Historik ("Time Travel" & Audit)**:
   - Hentning af komplet revisionshistorik specifikt for en given entitets UUID (`.kant/**/<uuid>.json`).
4. **Fail-Safe & Cross-Platform Integritet**:
   - Hvis Git ikke er installeret på maskinen, returneres `GitError::GitNotInstalled` pænt uden panics.
   - Kører fejlfrit på Windows, macOS og Linux.

---

## 📋 Acceptance Criteria
- [x] **AC1 - Repository Initialisering**: `GitService::init_repository(path)` opretter et gyldigt Git-repository og foretager en initial commit af `.kant/` strukturen.
- [x] **AC2 - Synkroniseringsstatus**: `GitService::get_sync_status(path)` detekterer korrekt om der er uudgivne commits eller synkroniseret tilstand.
- [x] **AC3 - Modeludgivelse (Commit)**: `GitService::publish_model(path, project, message)` gemmer projektet i `.kant/`, stager filerne og opretter en commit med korrekt besked og forfatter.
- [x] **AC4 - Domænehændelses-Mapper**: `DomainEventMapper` oversætter ændrede `.kant/` filer i en commit til semantiske `DomainChangeEvent` (🟢 Tilføjet, 🟡 Ændret, 🔴 Fjernet) med entitetsnavn.
- [x] **AC5 - Auto-genereret Versionsnote**: `generate_commit_summary(events)` producerer en præcis, domæneorienteret dansk opsummering af ændringerne.
- [x] **AC6 - Element-specifik Revisionshistorik**: `GitService::get_element_history(path, entity_uuid)` returnerer alle commits, der har berørt det specifikke element.
- [x] **AC7 - Verifikation via Accepttest**: `test_task_046_git_service_and_domain_event_mapping` i `tests/acceptance.rs` beviser samtlige funktioner mod et lokalt test-repository.

---

## 🚫 Must NOT
- Må IKKE bruge rå Git-terminologi i domænehændelserne (ingen hash-koder eller rå filstier til brugeren).
- Må IKKE panikke hvis Git ikke er tilgængeligt på systemet.
- Må IKKE blokere applikationstråden under Git-operationer.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 046 efter gennemførelse af Task 044 og Task 045.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_046`
- `cargo clippy --all-targets`
- `cargo fmt --check`
