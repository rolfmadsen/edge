---
type: Task Package
title: "Task 008: Canvas Ergonomi, Zoom, Pan & Magnetisk Gitter"
description: "Prik-gitter (dot-grid), magnetisk snap-to-grid, zoom via scrollhjul og fri panorering over uendeligt canvas"
status: open
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T15:17:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, ergonomics, zoom, pan, grid]
---

# Task 008: Canvas Ergonomi, Zoom, Pan & Magnetisk Gitter

**Status**: `SPEC`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Give begrebsmodelleringen på lærredet professionel desktop-ergonomi og layout-præcision. Dette omfatter et visuelt prik-gitter (dot-grid), magnetisk snapping af noder til gitteret for at sikre snorlige diagrammer, samt understøttelse af trinløs zoom (via musens scrollhjul) og fri panorering (via mellemrumstast eller midterste museknap), så brugeren ubesværet kan overskue og organisere store modeller.

## 📋 Acceptance Criteria
- [ ] `GraphCanvas` renderer et diskret, performant prik-gitter (dot-grid) i baggrunden, der tilpasser sig zoom-niveauet.
- [ ] Implementere magnetisk "Snap-to-Grid" (f.eks. 20px raster), så noder automatisk snapper på plads ved træk og slip, med mulighed for at slå snapping til/fra i værktøjslinjen.
- [ ] Understøtte zoom ind og ud via musens scrollhjul med markøren som ankerpunkt (`zoom_factor: f32`).
- [ ] Understøtte panorering (pan/scroll) af hele lærredet ved at trække med mellemrumstasten holdt nede eller via midterste museknap.
- [ ] Værktøjslinjen under Fane 3 indeholder knapper til "Nulstil visning" (zoom 100%, pan 0,0) og statusindikator for aktuel zoom-procent.
- [ ] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE introducere unødig CPU/GPU belastning under rendering af gitteret (skal udnytte `canvas::Geometry` caching effektivt).
- Må IKKE forskyde koordinaterne i den underliggende datamodel utilsigtet (verdenskoordinater vs. skærmkoordinater skal være matematisk præcist afkoblet).
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for canvas ergonomi, zoom, pan og magnetisk gitter.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 008-canvas-ergonomics-zoom-pan-grid`
- `xgauntlet verify --task 008-canvas-ergonomics-zoom-pan-grid`
