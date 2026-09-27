---
type: Task Package
title: "Task 048: Model Graph Tidslinje, Element-Historik & Visuel Konflikthåndtering"
description: "Visuel tidslinje for modelhistorik, element-specifik revisionshistorik (Time Travel audit) og brugervenlig visuel konfliktløsning ved 3-vejs modstridende feltændringer"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:35:00Z" }
tags: [git, timeline, model-graph, element-history, conflict-resolver, audit, time-travel]
---

# Task 048: Model Graph Tidslinje, Element-Historik & Visuel Konflikthåndtering

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/ui/app.rs`, `src/features/git/`, `src/features/model/merge.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Fuldende den førsteklasses Git-modelintegration i Edge (Kant) med domæneorienteret tidslinje, element-specifik revision og interaktiv konfliktløsning:

1. **Modelhistorik & Tidslinje (Model Graph Timeline)**:
   - En dedikeret visning ("Modelhistorik"), der viser modellens commits som en visuel tidslinje.
   - Hver version viser forfatter, tidsstempel, brugerens note og strukturerede domænehændelser (🟢 Oprettet, 🟡 Opdateret, 🔴 Fjernet).
2. **Element-specifik Historik (Audit / Time Travel)**:
   - Mulighed for at inspicere historikken for et udpeget element (Begreb eller Klasse via UUID).
   - Kalder `GitService::get_element_history` og viser præcis de versioner, hvor det pågældende element blev oprettet eller modificeret.
3. **Visuel Konflikthåndtering (3-Way Conflict Resolver)**:
   - Når en fletning resulterer i modstridende feltændringer (`ModelConflict`), vises en visuel afgørelsesdialog for arkitekten.
   - For hvert felt vises "Lokal version" (`ours`), "Server version" (`theirs`) og "Oprindelig version" (`base`).
   - Brugeren kan med ét klik vælge hvilken version der skal gælde.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Modelhistorik Tidslinje**: Appen kan åbne modelhistorikken (`Message::OpenModelHistoryModal`) og vise seneste versioner som domænehændelser.
- [ ] **AC2 - Element-specifik Historik**: Appen kan hente og vise historik specifikt filtreret på et begrebs eller en klasses UUID (`Message::OpenElementHistoryModal(Uuid)`).
- [ ] **AC3 - Visuel Konfliktløser Modal**: Ved modstridende felter kan appen præsentere `ModelConflict` i en dialog (`Message::OpenConflictResolverModal`) og lade brugeren vælge vinder-værdi (`Message::ResolveConflict(...)`).
- [ ] **AC4 - Verifikation via Accepttest**: Test i `tests/acceptance.rs` beviser at tidslinje, element-historik og konfliktløsning fungerer i appens state maskine.

---

## 🚫 Must NOT
- Må IKKE vise rå Git-hashes eller git-diff markører (`<<<<<<< HEAD`).
- Må IKKE tabe data ved konfliktløsning.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 048 for afslutning af Git model-integrationen.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_048`
- `cargo clippy --all-targets`
- `cargo fmt --check`
