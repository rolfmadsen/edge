---
type: Task Package
title: "Task 067: Canvas Drag Latency Optimization, Drop-Commit Autosave & Telemetry"
description: "Eliminering af drag-lag på Windows og tværs af platforme via drop-commit autosave, snapping-delta filtrering samt performance- og livscyklus-telemetri"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-03T19:59:00Z" }
tags: [canvas, drag-and-drop, performance, telemetry, diagnostics, windows, autosave]
---

# Task 067: Canvas Drag Latency Optimization, Drop-Commit Autosave & Telemetry

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-10-03`
**Scope**: `src/ui/diagram_canvas.rs`, `src/ui/concept_model_view.rs`, `src/ui/information_model_view.rs`, `src/ui/edge_router.rs`, `src/ui/app.rs`, `src/features/diagnostics.rs`, `tests/acceptance.rs`

## 🎯 Formål
Eliminere UI-lag og frysetilstande under drag-and-drop af enkelt- og flermarkerede noder (særligt udtalt på Windows 11 pga. NTFS, Windows Defender og `git.exe` proces-spawning) samt etablere målrettet diagnostik og performance-telemetri:
1. **Drop-Commit Autosave**: Flytte `trigger_autosave()` (og dermed tunge disk-skrivninger og `git status` subprocesser) væk fra kontinuerlige `CursorMoved`-hændelser. Autosave skal kun udføres én gang, når musen slippes (`ButtonReleased`) via `Message::CanvasNodesDragFinished`.
2. **Snapping-Delta Filtrering**: I `diagram_canvas.rs` forhindres udsendelse af redundante TEA-beskeder (`on_nodes_moved` / `on_node_moved`), hvis de beregnede snappede koordinater på 20px-gitteret (`GRID_SIZE`) er uændrede i forhold til forrige udsendelse.
3. **Livscyklus- & Autosave-Telemetri**:
   - Logge ved start af træk (`ButtonPressed`) med antal noder og leader-ID.
   - Logge ved fuldført træk (`ButtonReleased`) med distance og tidsforbrug.
   - Måle og logge tidsforbrug for disk-skrivning (`ProjectStorage::save`) og Git-synkronisering (`GitService::get_sync_status`) under autosave.
4. **Frame-Budget Monitoring for Kantrouting**:
   - `EdgeRouter::route_edges`: Registrere beregningstid og logge advarsel via `diagnostics::log_warn("canvas_perf", ...)`, hvis routing overskrider 16 ms (> 1 frame budget).

## 📋 Acceptance Criteria
- [x] **AC1 - Drop-Commit Autosave**: Under aktiv drag (`CursorMoved`) udføres ingen synkrone disk- eller git-kald. `trigger_autosave()` afvikles udelukkende én gang ved `ButtonReleased` via `Message::CanvasNodesDragFinished`.
- [x] **AC2 - Redundant Motion Delta Filtering**: `DiagramCanvas` udsender ikke `on_nodes_moved` eller `on_node_moved`, hvis de beregnede snappede koordinater er uændrede i forhold til seneste udsendelse under samme drag-session.
- [x] **AC3 - Drag Lifecycle & Autosave Diagnostics Telemetry**: Start og afslutning af drag registreres i `diagnostics::log_info`, og `trigger_autosave()` logger præcist tidsforbrug for hhv. disk-skrivning og git-statuscheck.
- [x] **AC4 - Edge Routing Frame-Budget Monitoring**: Hvis `EdgeRouter::route_edges` tager mere end 16 ms, logges en `canvas_perf` advarsel via `diagnostics::log_warn`.
- [x] **AC5 - Acceptance & Regression Tests**: Automatiserede tests verificerer, at `Message::CanvasNodesDragFinished` persisterer projektet til disk, at snappede koordinater opdateres, og at log-telemetri opfanger forløbet uden fejl.

## 🚫 Must NOT
- Zero Ambient Authority: Ingen netværkstelemetri eller eksterne kald.
- Zero High-Frequency Logging: Der må ALDRIG logges synkront under individuelle `CursorMoved`-events.
- Må IKKE ændre eksisterende diagramformater eller bryde `.kant` filformater.
- Må IKKE bryde eksisterende accepttests (`cargo test --workspace`).

## 📝 Revisions
- 2026-10-03: Oprettet opgavepakke efter analyse af Windows drag-latency og synkron disk/git flaskehals.
- 2026-10-03: Implementeret drop-commit autosave, snapping-delta filtrering, diagnostik-telemetri og frame-budget check. Verificeret med xGauntlet.

## 🧪 Verifikation
- `xgauntlet check-spec -t 067`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
