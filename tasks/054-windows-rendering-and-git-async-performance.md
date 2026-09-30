---
type: Task Package
title: "Task 054: Windows 11 Rendering Optimization & Git Async Performance"
description: "Eliminering af sort vinduesbaggrund og opstartsfrys på Windows 11 samt asynkron afvikling af Git-operationer og caching af git-binæren"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T20:46:00Z" }
tags: [windows, rendering, wgpu, git, performance, async, iced]
---

# Task 054: Windows 11 Rendering Optimization & Git Async Performance

**Status**: `ACTIVE`
**Intent**: `🐛 BUG FIX`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Løse identificerede Windows 11 rendering- og responstidsfejl samt optimere Git-integrationens afvikling:
1. **Windows 11 Sort Baggrund & Swapchain**: Konfigurere WGPU/Iced swapchain clear color (`.style(...)`) med `ThemeColors::SURFACE_BG` samt specificere hensigtsmæssige standard- og minimumdimensioner i `iced::window::Settings`.
2. **Git Subprocess Caching**: Erstatte gentagne subprocess-kald til `git --version` i `resolve_git_binary()` og `is_git_installed()` med `std::sync::OnceLock`, så binærstien og tilgængelighed kun tjekkes én gang per proces.
3. **Ikke-blokerende Opstarts-synkronisering**: Undgå at blokere appens konstruktør `App::new_with_path()` med synkrone `get_sync_status()`-kald under opstart, så vinduet åbner og kan flyttes/maksimeres omgående uden hak.
4. **Asynkron Git Udgivelse & Push**: Omlægge `Message::ConfirmPublish` fra synkron blokering af Iced UI-hovedtråden til asynkron afvikling via `Task::perform`, med aktiv fremskridtsvisning undervejs.

## 📋 Acceptance Criteria
- [ ] **AC1 - Git Binary & Installation Caching**: `GitService::is_git_installed()` og `GitService::resolve_git_binary()` benytter `OnceLock`, så gentagne kald ikke udfører unødige subprocess spawns.
- [ ] **AC2 - WGPU Root Style & Window Configuration**: `src/main.rs` er konfigureret med `.style(...)` indeholdende `ThemeColors::SURFACE_BG`, og `iced::window::Settings` definerer `size: (1280, 800)` og `min_size: (800, 600)` for at forhindre sorte swapchain-blink ved Aero Snap/maksimering.
- [ ] **AC3 - Hurtig Opstart Uden Synkron Git Blokering**: `App::new_with_path()` udfører ikke synkront tjek af remote sync-status på hovedtråden under boot.
- [ ] **AC4 - Asynkron Publish/Pull/Push Workflow**: `ConfirmPublish` afvikles via asynkron Iced `Task::perform(..., Message::PublishCompleted)` så UI forbliver responsivt ved 60 fps under netværks- og Git-operationer.

## 🚫 Must NOT
- Zero-Daemon invariant: Må IKKE efterlade hængende baggrundsprocesser.
- Zero Ambient Authority: Ingen utilsigtede eksterne netværksforbindelser uden brugerinitiering.
- Må IKKE bryde headless tests (`cargo test --workspace`).
- Må IKKE ændre eksisterende `.kant` filformater eller bryde semantisk 3-vejs merge.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke efter diagnosticering af Git-kald og Windows 11 DWM resize stalls.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
