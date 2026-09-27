---
type: Task Package
title: "Task 053: Fix Windows CI Compilation and Git Service PathBuf Resolution"
description: "Løs Windows CI kompileringsfejl i GitService forårsaget af manglende PathBuf import, og gør Windows-kandidatudledning universelt type-tjekket og testbar på alle platforme."
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T23:18:00Z" }
tags: [ci, windows, git, compilation, pathbuf]
---

# Task 053: Fix Windows CI Compilation and Git Service PathBuf Resolution

**Status**: `ACTIVE`
**Intent**: `🐛 BUG FIX`
**Oprettet**: `2026-09-27`

## 🎯 Formål
Windows CI jobbet (`kant-windows-x86_64`) i `.github/workflows/release.yml` fejlede med `error[E0433]: cannot find type PathBuf in this scope` i `src/features/git/service.rs:102:17` under tag-byg for `v0.3.0`.
Formålet er at:
1. Importere og anvende `PathBuf` korrekt i Windows-blokken i `GitService`.
2. Udtrække kandidatudledningen for Windows Git (`windows_git_candidates`) til en universelt kompileret hjælpefunktion, så den type-tjekkes og testes på Linux/macOS uden at vente på Windows CI.
3. Tilføje verifikationstest for Windows kandidatstier.
4. Sikre 0 advarsler og 100% test-pass rate på alle platforme.

## 📋 Acceptance Criteria
- [ ] **AC1 - Cross-Platform Windows Git Kandidater**: `GitService::windows_git_candidates(local_app_data: Option<&str>) -> Vec<std::path::PathBuf>` er defineret og kompileres på alle styresystemer, hvilket forhindrer skjulte platformsspecifikke typefejl.
- [ ] **AC2 - Windows CI Kompilation**: `src/features/git/service.rs` kompilerer fejlfrit for `cfg(target_os = "windows")` uden manglende `PathBuf` typefejl.
- [ ] **AC3 - Kandidat Test Verifikation**: Test verificerer at `windows_git_candidates(None)` returnerer `Program Files` og `Program Files (x86)` kandidater, og `windows_git_candidates(Some(...))` tilføjer `%LOCALAPPDATA%` stien.
- [ ] **AC4 - Nul Advarsler & Bevaret Cross-Platform Adfærd**: Ingen `unused_imports` eller clippy-fejl på Linux/macOS, og fuld bagudkompatibilitet for eksisterende git integration.

## 🚫 Must NOT
- Må IKKE introducere `unused_imports` advarsler på Linux/macOS.
- Må IKKE ændre eksisterende adfærd for Git-detektering på Linux eller macOS.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke til udbedring af Windows CI fejl.

## 🧪 Verifikation
- `cargo check --workspace --tests`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `xgauntlet check-spec`
