---
type: Task Package
title: "Task 065: Windows Rendering Diagnostics & WGPU Restoration"
description: "Evidensbaseret diagnosticering af Windows 11 rendering via lokal struktureret logning, in-app diagnostik, udbedring af CSS skygge/alfa artefakter og genopretning af WGPU som hardware-accelereret standard"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-03T10:54:00Z" }
tags: [windows, rendering, diagnostics, wgpu, logging, theme, artifacts]
---

# Task 065: Windows Rendering Diagnostics & WGPU Restoration

**Status**: `DONE`
**Intent**: `🐛 BUG FIX`
**Oprettet**: `2026-10-03`

## 🎯 Formål
Eliminere visuelle artefakter (sorte containerfelter og skygge-blokke) samt genoprette flydende hardware-accelereret ydelse på Windows 11 (fx Lenovo T14 Gen 2), baseret på empirisk evidens frem for gætteri:
1. **Lokal struktureret logning & diagnostik**:
   - Implementere letvægts, robust fil- og konsollogning (skriver til `%APPDATA%\Kant\logs\kant.log` på Windows og standard app-logsti på Unix).
   - Logge miljøparametre, aktiv Iced-backend, skærmopløsning/DPI, operativsystem og opstartstider.
   - CLI-flag `--diagnostics` / `--debug` til inspektion i terminal.
2. **In-app Diagnostik-modal**:
   - Tilføje "System- og grafikdiagnostik..." i Hjælp-menuen, der viser platforminfo, aktiv backend, seneste loglinjer samt knap til at kopiere fuld diagnostisk rapport til udklipsholderen.
3. **Genopretning af WGPU som standard**:
   - Rulle den forhastede tvungne `tiny-skia` CPU-software-fallback tilbage i `resolve_default_iced_backend()`, så Windows som udgangspunkt udnytter GPU-hardwareacceleration (`wgpu`).
   - Bevare `ICED_BACKEND=tiny-skia` som valgfri overstyring for fejlfinding.
4. **Udbedring af sorte CSS-artefakter i styling**:
   - Udskifte semi-transparente `Color::from_rgba` baggrunde på centrale containere og sidepaneler med fuldt opake farver (`ThemeColors::SURFACE_CARD` / `Color::WHITE` / `SURFACE_BG`).
   - Hærde `Shadow`-definitioner i `card_container_style` mod sort udfald/blending-fejl på tværs af backends.

## 📋 Acceptance Criteria
- [x] **AC1 - Backend Default Resolution (WGPU Restoration)**: `resolve_default_iced_backend(is_windows: bool, existing_backend: Option<&str>)` gennemtvinger IKKE længere blindt `tiny-skia`. Hvis `existing_backend` er `None`, returneres `None` (hvorved Iced benytter standard `wgpu`), mens eksplicit `ICED_BACKEND` (fx `"tiny-skia"` eller `"wgpu"`) fortsat respekteres.
- [x] **AC2 - Struktureret Lokal Logning & Diagnostikmodul**: Nyt modul `kant::features::diagnostics` (eller `src/ui/diagnostics.rs`) initialiserer logning til lokal logfil og in-memory buffer, opfanger opstartsinformation (OS, arkitektur, miljøvariable, valgt backend) og genererer en formateret systemrapport.
- [x] **AC3 - CLI Startup Argumenter**: Hvis appen startes med `--diagnostics` eller `-d`, udskrives systemdiagnostikken struktureret til stdout/konsol.
- [x] **AC4 - In-App Diagnostik Modal under Hjælp**: Under "Hjælp"-menuen findes menupunktet "🔍 System- og grafikdiagnostik...", som åbner en modal med platformdata, aktiv backend, log-udsnit og en "Kopier rapport"-handling.
- [x] **AC5 - Hærdet Opak Styling & Ingen Sorte Skygge-Artefakter**: Container styles (`card_container_style`, `floating_panel_style`, `base_layout`) anvender 100% opake baggrunde, så manglende eller defekt software-alpha-blending ikke resulterer i sorte paneler.
- [x] **AC6 - Regression & Acceptance Tests**: Automatiserede accepttests i `tests/acceptance.rs` verificerer backend-resolution, diagnostisk rapportgenerering og logging uden fejl.

## 🚫 Must NOT
- Zero Ambient Authority: Ingen netværkstelemetri, ingen eksterne HTTP/socket-kald med diagnosedata. Alle logs forbliver 100% lokale på maskinen.
- Zero-Daemon: Ingen baggrunds-daemons eller hængende tråde til logskrivning.
- Må IKKE bryde headless tests (`cargo test --workspace`).
- Må IKKE bryde Linux eller macOS understøttelse.

## 📝 Revisions
- 2026-10-03: Oprettet opgavepakke efter brugerobservation af sorte paneler og latency-regressioner på Lenovo T14 Gen 2 (Windows 11).
- 2026-10-03: Implementeret diagnostikmodul, genoprettet WGPU default, udbedret styling til fuld opacitet og verificeret via multi-layer test gauntlet. Markerct som DONE.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
