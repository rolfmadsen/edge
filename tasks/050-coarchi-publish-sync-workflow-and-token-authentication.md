---
type: Task Package
title: "Task 050: coArchi Synkroniseringsworkflow, Push & Token Autentifikation"
description: "Fuldendt coArchi-modeludgivelse (Commit -> Auto-Pull/3-vejs merge -> Auto-Push), fjernelse af forældede Hent seneste knapper i modalen, dedikeret Personal Access Token understøttelse i GitConnectionModal og gennemskuelig synkroniseringsfeedback."
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T17:46:00Z" }
tags: [git, coarchi, push, sync, token, auth, conflict-resolution, iced]
---

# Task 050: coArchi Synkroniseringsworkflow, Push & Token Autentifikation

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE` / 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/features/git/service.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Bring Kants Git-integration 100% på niveau med det etablerede **coArchi** workflow for forretnings- og enterprisearkitekter:

1. **coArchi "Udgiv model" Pipeline (One-Click Publish & Sync)**:
   - I dialogen "Udgiv model" fjernes den vildledende knap "Hent seneste". Modellen skal kun have `[ Annuller ]` og `[ Udgiv model ]`.
   - Ved bekræftelse af "Udgiv model" (`Message::ConfirmPublish`):
     - Trin 1: Gem og commit ændringer lokalt (`GitService::publish_model`).
     - Trin 2: Hvis remote (`origin`) er konfigureret, foretag automatisk `GitService::pull_model`.
     - Trin 3: Hvis der opstår modstridende feltændringer, åbnes Kants visuelle konfliktløser (`ConflictResolverModalState`).
     - Trin 4: Hvis fletningen er ren (eller efter konfliktløsning), skubbes ændringerne automatisk til remote (`GitService::push_model`).
2. **GitService Push & Branch Detection (`push_model`)**:
   - `GitService::push_model(repo_path, remote)` skubber aktuel branch til remote med `GIT_TERMINAL_PROMPT=0`.
   - Returnerer strukturerede fejl (f.eks. `GitError::AuthenticationRequired` eller brugervenlig forklaring, hvis legitimationsoplysninger mangler).
3. **Personal Access Token (PAT) i `GitConnectionModal`**:
   - Tilføj et dedikeret felt til **Personal Access Token** (maskeret tekstfelt / password-mode eller bullet-visning) i `GitConnectionModalState`.
   - Gemmer tokenet sikkert i remote URL (`https://<token>@github.com/...`) eller Git credential, og stripper tokenet ved visning, så brugeren altid ser en ren URL og et separat tokenfelt.
4. **Brugervenlig Synkroniseringsfeedback**:
   - Tydelig statusmeddelelse efter udgivelse (fx "✅ Modellen er udgivet og synkroniseret med fjernlageret").
   - Hvis autentifikation mangler, præsenteres en klar vejledning til at angive token i forbindelsesdialogen frem for tavshed.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Ren Udgiv-dialog uden "Hent seneste"**: `view_publish_modal` indeholder kun `[ Annuller ]` og `[ Udgiv model ]`.
- [ ] **AC2 - GitService Push Model**: `GitService::push_model` kan skubbe den lokale models commits til et remote repository.
- [ ] **AC3 - Fuld coArchi Udgivelseskæde**: Ved bekræftelse i Udgiv-dialogen foretages automatisk commit, pull (med 3-vejs merge) og push.
- [ ] **AC4 - Token-håndtering i GitConnection**: `GitConnectionModalState` har et token-felt; lagring konfigurerer adgangstoken transparent, så push/pull autoriseres automatisk.
- [ ] **AC5 - Verifikation via Accepttest**: `tests/acceptance.rs` indeholder en dedikeret accepttest for den samlede coArchi-udgivelsessekvens og token-håndtering.

---

## 🚫 Must NOT
- Må IKKE vise rå Git-fejlkoder ("fatal: Authentication failed", "refusing to merge unrelated histories") uden forretningsvenlig forklaring.
- Må IKKE hænge GUI i uendelig ventetid på terminal-prompts (`GIT_TERMINAL_PROMPT=0`).
- Må IKKE afsløre tokens i plain text i URL-feltet.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 050 for fuld coArchi modeludgivelse og token-autentifikation.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_050`
- `cargo clippy --all-targets`
- `cargo fmt --check`
