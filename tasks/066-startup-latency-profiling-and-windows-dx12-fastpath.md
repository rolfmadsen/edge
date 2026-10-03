---
type: Task Package
title: "Task 066: Startup Latency Profiling & Windows DX12 Fast-Path"
description: "Instrumentere opstartsfaser med relativ tidsstempling (+ms), identificere forsinkelseskilder og aktivere WGPU DX12 fast-path på Windows"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-03T11:35:00Z" }
tags: [startup, profiling, performance, wgpu, dx12, logging, windows]
---

# Task 066: Startup Latency Profiling & Windows DX12 Fast-Path

**Status**: `DONE`
**Intent**: `🐛 BUG FIX`
**Oprettet**: `2026-10-03`

## 🎯 Formål
Finde den præcise rodårsag til ~30 sekunders opstartstid på Windows 11 bærbare (fx Lenovo ThinkPad T14 Gen 2) og eliminere unødige driver-søgninger:
1. **Relativ Tidsstempling & Profilering**:
   - `kant::features::diagnostics`: Udvid logningsformatet til at inkludere proces-relativ tid i millisekunder, fx `[+12ms] [platform] ...`.
   - Logge klare milepæle for processtart, modelindlæsning, Iced GUI initialisering og første tegnede frame.
2. **Windows WGPU DX12 Fast-Path**:
   - På Windows sættes `WGPU_BACKEND=dx12` som standard i `initialize_platform_defaults()`, hvis ingen variabel er sat i forvejen. Dette forhindrer WGPU i at scanne defekte eller langsomme Vulkan ICD-drivere på Intel Iris Xe grafik.
3. **Diagnostik Rapportering**:
   - Udskrive den registrerede opstartstid og WGPU-adapterbackend i systemdiagnostik-rapporten og in-app modalen.

## 📋 Acceptance Criteria
- [x] **AC1 - Relativ tidsstempling i log**: Alle loglinjer i `kant.log` og log-bufferen indeholder relativ tid siden processtart formateret som `[+Xms]`.
- [x] **AC2 - WGPU DX12 Fast-Path på Windows**: `resolve_default_wgpu_backend(is_windows: bool, existing: Option<&str>)` returnerer `Some("dx12")` når `is_windows == true` og ingen backend er sat, og `None` hvis en eksisterende overstyring findes eller på ikke-Windows.
- [x] **AC3 - Opstartsprofilering i Diagnostics**: `SystemDiagnostics` opfanger opstartstids-milepæle og rapporterer total opstartstid til første frame i diagnostik-rapporten.
- [x] **AC4 - Regression & Acceptance Tests**: Automatiserede accepttests i `tests/acceptance.rs` verificerer relativ tidsstempling, backend-resolution for WGPU og milepælsregistrering.

## 🚫 Must NOT
- Zero Ambient Authority: Ingen ekstern telemetri eller netværkskald.
- Zero Overhead: Profilering må maksimalt koste få mikrosekunder via `std::time::Instant`.
- Må IKKE bryde Linux, macOS eller headless test-miljøer.

## 📝 Revisions
- 2026-10-03: Oprettet opgavepakke efter observation af 30s opstartstid på Lenovo T14.
- 2026-10-03: Implementeret relativ tidsstempling, opstartsmilepæle og WGPU DX12 fast-path. Verificeret og afsluttet som DONE.

## 🧪 Verifikation
- `xgauntlet check-spec -t 066`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
