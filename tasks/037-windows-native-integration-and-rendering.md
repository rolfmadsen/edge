---
type: Task Package
title: "Task 037: Windows Native Integration & Rendering Optimering"
description: "Cross-platform native fildialog via rfd, Windows ressource- og app-ikoner og swapchain optimering for Aero Snap"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-21T18:30:00Z" }
tags: [windows, native, rfd, icons, rendering]
---

# Task 037: Windows Native Integration & Rendering Optimering

**Status**: `DONE`  
**Intent**: 🐛 `BUG FIX`  
**Dato**: `2026-09-21`  
**Scope**: `src/ui/file_dialog.rs`, `src/main.rs`, `Cargo.toml`, `build.rs`, `assets/icons/`, `docs/adr/010-cross-platform-native-dialogs-and-os-packaging.md`  

---

## 🎯 Formål
Løser tre identificerede Windows-specifikke fejl og forbedrer desktop-oplevelsen markant:
1. **Fildialog (Stifinder)**: Erstatte hårdkodet `zenity` subprocess i `src/ui/file_dialog.rs` med native cross-platform dialoger via `rfd` (Rust File Dialog), så Stifinder åbner pålideligt ved *Filer > Gem som...*, *Filer > Åbn...* og knappen *🖥️ Gennemse...*.
2. **Applikationsikon**:
   - Indlejre multi-size `kant.ico` i Windows `.exe` PE-header via `build.rs` og `winres`.
   - Konfigurere runtime vinduesikon i `iced::window::Settings` for proceslinje og titellinje.
3. **Resize / Aero Snap optimering**:
   - Deaktivere 4x MSAA antialiasing som standard (`.antialiasing(false)`) i `src/main.rs` for at undgå tunge GPU-reallokeringsstød under dynamisk resize og eliminere sort skærm under Aero Snap (Win+Arrow / drag to edge).

---

## 📋 Acceptance Criteria
- [x] **AC1 - Native Fildialog (`rfd`)**: `pick_file_to_open()` og `pick_file_to_save()` i `src/ui/file_dialog.rs` anvender `rfd::FileDialog` i stedet for `zenity`. Filtre for `*.kant.json`, `*.edge.json`, `*.json` og `*` bevares.
- [x] **AC2 - Windows Ressource-Indlejring**: `build.rs` konfigurerer `winres` under `cfg(windows)` med `assets/icons/kant.ico`. `kant.ico` indeholder gyldige ikoner (16x16, 32x32, 48x48, 256x256).
- [x] **AC3 - Runtime Vinduesikon**: `src/main.rs` initialiserer `iced::application` med `iced::window::Settings` indeholdende et gyldigt `iced::window::Icon`.
- [x] **AC4 - Resize-Optimering**: `src/main.rs` er konfigureret med `.antialiasing(false)` for at forhindre WGPU/DX12 swapchain stalls på Windows.
- [x] **AC5 - Headless & Cross-Platform Invarianter**: Alle enheds- og accepttests i `cargo test --workspace` kører fejlfrit uden at forsøge at åbne grafiske vinduer eller afhænge af `zenity`.

---

## 🚫 Must NOT
- Må IKKE fejle eller blokere i headless testmiljøer (CI/CD) uden display-server.
- Må IKKE introducere eksterne runtime subprocesser (f.eks. PowerShell eller Zenity) på Windows.
- Må IKKE bryde eksisterende projektfilformater (`.kant.json` / `.edge.json`) eller atomisk gemning.

---

## 📝 Revisions
- *2026-09-21*: Oprettet på baggrund af brugertest på Windows 11.

---

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
