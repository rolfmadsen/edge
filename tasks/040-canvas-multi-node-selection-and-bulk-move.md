---
type: Task Package
title: "Task 040: Multi-Node Markering og Bulk Forskydning på Diagram Lærred (Ctrl+Klik & Drag-Select)"
description: "Masse-markering via Ctrl/Cmd+Klik og rektangulær drag-select samt synkron parallelforskydning af flere noder på lærredet for begrebs- og informationsmodel"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [canvas, selection, multi-select, bulk-move, drag-select, marquee, ergonomics]
---

# Task 040: Multi-Node Markering og Bulk Forskydning på Diagram Lærred (Ctrl+Klik & Drag-Select)

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/ui/diagram_canvas.rs`, `src/ui/concept_model_view.rs`, `src/ui/information_model_view.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Gøre det muligt for brugeren at markere og flytte flere diagramnoder ad gangen på både Begrebsmodellen (Fane 2) og Informationsmodellen (Fane 3), hvilket er essentielt ved omstrukturering og oprydning af store modeller:

1. **Multi-Node Markering**:
   - **Ctrl / Cmd + Klik**: Toggler udvælgelsen af en node. Hvis noden allerede er valgt, fravælges den. Hvis den ikke er valgt, føjes den til udvalget uden at afmarkere de øvrige noder.
   - **Rektangulær Drag-Select (Marquee / Box-select)**: Klik og træk på tomt lærred (når der ikke panoreres med Space eller midterklik) danner en visuel, semi-transparent markeringsramme. Ved mus-slip (release) tilføjes alle noder, der overlapper eller omsluttes af rektanglet, til markeringen.
   - **Afmarkering**: Enkeltklik på tomt lærred uden Ctrl/Cmd rydder alle markeringer.
2. **Synkron Bulk Forskydning (Drag Move)**:
   - Når brugeren trækker i en af de markerede noder, flyttes samtlige aktuelt markerede noder synkront med samme relative delta $(\Delta x, \Delta y)$.
   - Snap-to-grid bevares for de flyttede noder.
   - Forbundne kanter/relationer opdaterer deres routing i realtid under trækket.
3. **Ergonomi og Samspil med Egenskaber**:
   - Når én node er valgt, vises dens detaljer i Egenskabs-panelet som hidtil.
   - Når flere noder er valgt, vises en summarisk status i Egenskaber (f.eks. "N elementer valgt"), eller panelet forbliver fokuseret på den primære (sidst valgte) node uden at kaste fejl.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Ctrl/Cmd + Klik Multi-select**: Ved klik på en node med tastatur-modifikatoren `Ctrl` eller `Cmd` aktiv toggles nodens tilstedeværelse i udvalget (`selected_node_ids: HashSet<NodeId>`).
- [ ] **AC2 - Marquee / Box Drag-Select**: Klik og træk på tomt lærred genererer en synlig markeringsramme (theme stroke & dæmpet fill). Noder inden for rammen vælges ved release.
- [ ] **AC3 - Synkron Flytning af Noder**: Ved træk i en markeret node forskyder alle aktuelt markerede noder sig med identisk $(\Delta x, \Delta y)$.
- [ ] **AC4 - Grid Snapping ved Bulk Move**: Snap-to-grid beregnes konsistent for positionerne.
- [ ] **AC5 - Afmarkering**: Almindeligt klik på tomt lærred uden `Ctrl`/`Cmd` nulstiller markeringen.
- [ ] **AC6 - Bevarelse af Eksisterende Canvas-Funktioner**: Enkelt-node træk, forbindelseshåndtag (connect-handle drag), dobbeltklik og panorering (Space + træk el. midterklik) fungerer uændret.
- [ ] **AC7 - Verifikation via Accepttest**: `test_task_040_canvas_multi_node_selection_and_bulk_move` beviser at både Ctrl-klik, marquee-udvælgelse og koordinatforskydning af multiple noder fungerer fejlfrit.

---

## 🚫 Must NOT
- Må IKKE interferere med panorerings-gestus (mellemrumstast + træk eller midterklik).
- Må IKKE miste eller forskyde edge-porte/relationer forkert under bulk-flytning.
- Må IKKE blokere for connect-handle interaktion på den primært valgte node.
- Må IKKE give UI freeze ved markering af store mængder noder.

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_040`
- `cargo clippy --all-targets`
- `cargo fmt --check`
