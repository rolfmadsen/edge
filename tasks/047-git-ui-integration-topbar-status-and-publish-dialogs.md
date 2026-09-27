---
type: Task Package
title: "Task 047: Git UI Integration: Topbar Status & Udgiv/Hent Dialoger"
description: "Integration af Git statusindikator i topbar samt modal dialoger for 'Udgiv model' med ændringsoverblik og 'Hent seneste' uden teknisk Git-jargon"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:28:00Z" }
tags: [git, ui, iced, topbar, publish-dialog, pull, status-pill]
---

# Task 047: Git UI Integration: Topbar Status & Udgiv/Hent Dialoger

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/ui/app.rs`, `src/ui/theme.rs`, `src/features/git/`, `tests/acceptance.rs`

---

## 🎯 Formål
Integrere Git backend-tjenesten (`GitService`) direkte i Kants Iced GUI, så modellerere og arkitekter får gennemsigtighed og tryghed uden at blive konfronteret med rå Git-terminologi:

1. **Topbar Git Statusindikator (`pill badge`)**:
   - Viser aktuel status i topbarens højre side ved siden af projektnavn/collab:
     - 🟢 **Synkroniseret** (Modellen stemmer overens med serveren)
     - 🟡 **Lokale ændringer** (Ugemte/staged ændringer klar til udgivelse)
     - 🔵 **{N} klar til udgivelse** (Lokale commits klar til push)
     - ⬇️ **Nyt på serveren** (Nye ændringer tilgængelige fra server)
     - ⚪ **Lokal model** (Ikke tilknyttet versionsstyring)
2. **"Udgiv model" Dialog (Commit & Push)**:
   - Åbnes ved klik på status badge eller menupunkt "Udgiv model...".
   - Viser dynamisk forhåndsvisning af domænehændelser (🟢 Oprettet, 🟡 Opdateret, 🔴 Fjernet).
   - Valgfri besked til kolleger med automatisk genereret forslag (fx *"Oprettet 1 klasse, opdateret 1 begreb"*).
   - Enkel, fremtrædende handlingsknap: "Udgiv model".
3. **"Hent seneste" Handling (Pull & Semantisk Merge)**:
   - Automatisk hentning af seneste modelversion fra server med uforstyrret 3-vejs fusion (`merge_models`).
   - Pæn notifikation om resultatet ("Modellen blev opdateret uden konflikter" eller "Flettet med bemærkninger").
4. **"Tilknyt / Initialiser modelarkiv"**:
   - Mulighed for at initialisere Git-styring på eksisterende model direkte fra appen med ét klik.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Topbar Status Badge**: Topbaren i appen gengiver en visuel status-pill baseret på `RepoSyncStatus` (Synkroniseret, Lokale ændringer, Klar til udgivelse).
- [ ] **AC2 - Udgiv Model Modal**: Dialog med automatisk ændringsoversigt (domænehændelser) og mulighed for tilpasset note.
- [ ] **AC3 - Udgivelse E2E Workflow**: Bekræftelse i dialogen kalder `GitService::publish_model`, opdaterer topbarens status og lukker modalen.
- [ ] **AC4 - Initialisering fra UI**: Hvis modellen ikke er versionsstyret, tilbyder UI en knap til "Aktivér versionsstyring", der kalder `GitService::init_repository`.
- [ ] **AC5 - Verifikation via Accepttest**: Test i `tests/acceptance.rs` validerer UI-appens håndtering af statusopdateringer, modal-åbning og udgivelsesbeskeder.

---

## 🚫 Must NOT
- Må IKKE vise rå Git-begreber i UI (forbudte ord: "commit", "HEAD", "rebase", "fast-forward", "cherry-pick").
- Må IKKE blokere GUI under netværks- eller I/O-kald.
- Må IKKE miste brugerens lokale modifikationer ved fletning.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 047 efter gennemførelse af Task 044, 045 og 046.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_047`
- `cargo clippy --all-targets`
- `cargo fmt --check`
