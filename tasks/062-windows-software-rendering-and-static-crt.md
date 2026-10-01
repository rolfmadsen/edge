---
type: Task Package
title: "Task 062: Windows Software Rendering Default & Static CRT Linkage"
description: "Konfiguration af tiny-skia som standard rendering backend på Windows til eliminering af Intel GPU swapchain stalls samt statisk CRT-linking mod VCRUNTIME140.dll afhængigheder"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-01T17:41:00Z" }
tags: [windows, rendering, tiny-skia, crt-static, msvc, release]
---

# Task 062: Windows Software Rendering Default & Static CRT Linkage

**Status**: `DONE`
**Intent**: `🐛 BUG FIX`
**Oprettet**: `2026-10-01`

## 🎯 Formål
Løse to fundamentale platformsproblemer ved afvikling af Kant på Windows 11 uden gætterier og uden at akkumulere unødig UI-kode:
1. **Intel GPU Swapchain Stall & Sort Skærm**: Konfigurere `tiny-skia` (software rendering) som standard på Windows, medmindre brugeren eksplicit overstyrer via `ICED_BACKEND`. Dette omgår Intels kendte DirectX 12 driverfejl ved vinduesmaksimering på fx Lenovo T14 Gen 3 (Iris Xe).
2. **Statisk C-Runtime Linking**: Konfigurere Windows MSVC release-buildet i GitHub Actions med `-C target-feature=+crt-static`, så `kant.exe` er fuldstændigt selvkørende og ikke crasher med manglende `VCRUNTIME140.dll` på rene Windows-maskiner.

## 📋 Acceptance Criteria
- [x] **AC1 - Backend Default Resolution**: En deterministisk funktion `resolve_default_iced_backend(is_windows: bool, existing_backend: Option<&str>) -> Option<&'static str>` returnerer `Some("tiny-skia")` på Windows når ingen backend er sat, og `None` hvis `ICED_BACKEND` allerede er sat eller på ikke-Windows platforme.
- [x] **AC2 - Runtime Bootstrap**: `main.rs` kalder initialiseringslogikken før `iced::application` startes, så Windows-afvikling altid starter med `tiny-skia` som sikker standard.
- [x] **AC3 - Statisk CRT Release Byg**: `.github/workflows/release.yml` bygger til `x86_64-pc-windows-msvc` med `RUSTFLAGS="-C target-feature=+crt-static"`.
- [x] **AC4 - Automatiserede Tests**: Enhedstest i test-suiten der verificerer backend-udvælgelsen og miljøvariabel-kontrakten.

## 🚫 Must NOT
- Må IKKE ændre eksisterende styling- eller widget-kode i `src/ui/`.
- Må IKKE fjerne muligheden for at brugere kan overstyre med `$env:ICED_BACKEND="wgpu"`.
- Må IKKE bryde Unix/macOS platforme eller eksisterende CI workflows.

## 📝 Revisions
- 2026-10-01: Oprettet opgavepakke efter empirisk isolering af Intel Iris Xe DX12 swapchain fejl og VCRUNTIME140.dll mangel i VirtualBox og Lenovo T14.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
