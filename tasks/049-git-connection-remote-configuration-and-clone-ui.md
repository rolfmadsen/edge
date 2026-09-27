---
type: Task Package
title: "Task 049: Git Forbindelseskonfiguration, Fjernlager & Klon Model UI"
description: "Brugergrænseflade under Filer til opsætning af Git-forbindelse, fjernlager (Remote URL), forfatteridentitet, kloning af eksisterende modeller og direkte adgang til historik og udgivelse"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:47:00Z" }
tags: [git, remote-url, clone, git-ui, settings, file-menu]
---

# Task 049: Git Forbindelseskonfiguration, Fjernlager & Klon Model UI

**Status**: `DONE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/features/git/service.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Gøre Git-forbindelse og fjernlagersamarbejde 100% konfigurerbart direkte fra brugergrænsefladen under menuen **`Filer ▾`**:

1. **Versionsstyring i `Filer ▾` Menulinjen**:
   - Tilføj en dedikeret sektion "VERSIONSSTYRING (GIT)" i `Filer ▾` dropdown med:
     - 🌐 **Git-forbindelse & Fjernlager...** (`Message::OpenGitConnectionModal`)
     - 📦 **Klon model fra Git...** (`Message::OpenGitCloneModal`)
     - ⏳ **Modelhistorik & Tidslinje...** (`Message::OpenModelHistoryModal`)
     - 🚀 **Udgiv modelændringer...** (`Message::OpenPublishModal`)
     - 📥 **Hent seneste opdateringer** (`Message::PullModel`)
2. **Git Forbindelsesdialog (`GitConnectionModal`)**:
   - Viser og redigerer Remote URL (f.eks. `https://github.com/org/model.git` eller SSH).
   - Viser og redigerer forfatteroplysninger (`user.name` og `user.email`), så commits i modelhistorikken krediteres korrekt.
   - Handlinger: "Gem forbindelse", "Fjern fjernlager", "Luk".
3. **Klon Model Dialog (`GitCloneModal`)**:
   - Arkitekten kan indtaste en Git URL og en lokal målmappe.
   - Kloner projektet via `GitService::clone_repository` og åbner automatisk den klonede model i Kant.
4. **GitService Udvidelser**:
   - `get_remote_url(repo_path)` & `set_remote_url(repo_path, url)` & `remove_remote(repo_path)`.
   - `get_user_identity(repo_path)` & `set_user_identity(repo_path, name, email)`.
   - `clone_repository(remote_url, destination_dir)`.

---

## 📋 Acceptance Criteria
- [x] **AC1 - Git-sektion i Filer Menu**: Menulinjen `Filer ▾` indeholder handlingspunkter for Forbindelse, Klon, Historik, Udgiv og Hent.
- [x] **AC2 - Git Forbindelsesdialog**: `GitConnectionModal` tillader indtastning og lagring af Remote URL (`origin`) samt forfatterens navn og e-mail.
- [x] **AC3 - Klon Model Workflow**: `GitCloneModal` kloner et repository fra en angivet URL til en lokal sti og indlæser den klonede model i appen.
- [x] **AC4 - GitService Remote & Identity API**: `GitService` tilbyder pålidelige metoder til remote URL, brugeridentitet og `git clone`.
- [x] **AC5 - Verifikation via Accepttest**: Test i `tests/acceptance.rs` verificerer hele workflowet for remote-konfiguration, kloning og menu-interaktion.

---

## 🚫 Must NOT
- Må IKKE fejle eller crashe hvis et repository mangler `origin` eller `user.name`.
- Må IKKE overskrive eksisterende ikke-tomme destinationsmapper ukontrolleret under klon.
- Må IKKE introducere rå Git-fejlkoder uden brugervenlig forklaring i dialogerne.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 049 baseret på brugerfeedback om manglende Git-konfiguration i UI.
- 2026-09-27: Implementeret Git-sektion i Filer-menu, GitConnectionModal, GitCloneModal og GitService remote/clone API; 100% verificeret med accepttest.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_049`
- `cargo clippy --all-targets`
- `cargo fmt --check`
