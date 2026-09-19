---
type: Task Package
title: "Task 012: Unified Diagram Canvas og Begrebsmodel Studio"
description: "Unificering af graph_canvas.rs og information_canvas.rs til én fælles DiagramCanvas-komponent med pluggable node-rendering, samt harmonisering af Begrebsmodellen (Fane 3) til 3-delt Canvas Studio layout."
status: active
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T18:52:00Z" }
tags: [task-lifecycle, intent, unified-canvas, concept-model, studio-layout, tdd, refactor]
---

# Task 012: Unified Diagram Canvas og Begrebsmodel Studio

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-19`

## 🎯 Formål
1. **Unificeret DiagramCanvas-komponent (`src/ui/diagram_canvas.rs`)**:
   - Eliminere parallel kode og redundante hændelseshåndteringer mellem `graph_canvas.rs` og `information_canvas.rs`.
   - Samle fælles viewport-styring (pan, zoom, reset, snap-to-grid), hændelsesprocessering (middle-pan, space-pan, drag-and-drop med offset-korrektion, double-click) og ortogonal Manhattan-routing i én genbrugelig kerne.
   - Understøtte pluggable node-rendering via fleksible closures eller traits, så Begrebsmodellen kan tegne sine FDA-kasser/piller og Informationsmodellen kan tegne sine 3-sektions UML-klassekasser.
2. **Harmoniseret Begrebsmodel Studio (Fane 3, `src/ui/concept_model_view.rs`)**:
   - Etablere det 3-delte Canvas Studio layout (ADR 007) på Begrebsmodellen:
     - **Venstre Palet (Repository Browser, ~240px)**: Oversigt over projektets begreber, søgefelt, placeringsindikator på canvas (`[✓]` / `[+]`), samt handlinger til oprettelse og diagram-tilføjelse.
     - **Center Canvas (Fill)**: Interaktivt diagram-arbejdsområde drevet af den unificerede `DiagramCanvas`.
     - **Højre Inspector (~290-300px)**: Kontekstpanel til hurtig redigering af det valgte begreb, kendskab til tilknyttede relationer samt mulighed for at fjerne noden fra diagrammet uden at slette begrebet fra projektet.

## 📋 Acceptance Criteria
- [ ] **Unified `DiagramCanvas` (`src/ui/diagram_canvas.rs`)**:
  - `CanvasNode` og `CanvasEdge` abstraktioner eller generiske adaptere til noder og relationer.
  - 100% fælles event-handling: Zoom, Pan (Space og Middle click), Node Drag (uden jump-offset), Snap to Grid, Double Click.
  - 100% fælles baggrundsgrid og ortogonal edge-routing (`EdgeRouter`) med korrekte UML-pile (Generalisering, Komposition, Association) og labels.
  - Pluggable node-rendering, der lader hver model definere sit eget visuelle udtryk (FDA koncept vs UML 3-sektions klasse).
  - Bagudkompatibilitet for eksisterende imports (`CanvasViewport` m.fl.).
- [ ] **Begrebsmodel Studio 3-delt Layout (`src/ui/concept_model_view.rs`)**:
  - Udflytning af Fane 3 præsentationslogik fra `app.rs` til dedikeret `concept_model_view.rs`.
  - Venstre palet med begrebsliste, søgning og `[✓]` (på diagram) / `[+]` (tilføj til diagram).
  - Center canvas med toolbar (Zoom, Snap, Opret Relation, Reset) og relation modal overlay.
  - Højre inspector med visning/hurtigredigering af begreb, relationsoversigt og "Fjern fra diagram" knap.
- [ ] **Model & Controller integration**:
  - `ConceptGraph::is_concept_on_diagram(&self, concept_id: Uuid) -> bool`.
  - Støtte for at fjerne en node fra canvas (`RemoveConceptFromDiagram`) uden at slette begrebet fra projektet.
  - Søgning i begrebspaletten.
- [ ] **Fuld Verifikation & Nul Regressionsfejl**:
  - Ny acceptancetest `test_task_012_unified_diagram_canvas_and_concept_studio_layout` i `tests/acceptance.rs`.
  - Alle eksisterende 26 enheds-, accept- og proptests forbliver 100% grønne.
  - `cargo clippy -- -D warnings` og `cargo fmt --check` uden fejl.

## 🚫 Must NOT
- Må IKKE ændre serde-serialiseringsformat for eksisterende `model.edge.json` filer.
- Må IKKE slette begreber fra projektet, når brugeren blot fjerner en node fra diagrammet.
- Må IKKE introducere regressionsfejl i Informationsmodellen (Fane 4) eller Begrebslisten (Fane 2).
- Må IKKE foretage remote push (`git push`) eller destruktive git-handlinger.

## 📝 Revisions
- 2026-09-19: Oprettet Task 012 efter brugerbestilling.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
