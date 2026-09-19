---
type: Task Package
title: "Task 008: Canvas Ergonomi, Zoom, Pan & Magnetisk Gitter"
description: "Prik-gitter (dot-grid), magnetisk snap-to-grid, zoom via scrollhjul og fri panorering over uendeligt canvas"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T15:17:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, ergonomics, zoom, pan, grid]
---

# Task 008: Canvas Ergonomi, Zoom, Pan & Magnetisk Gitter

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Give begrebsmodelleringen på lærredet professionel desktop-ergonomi og layout-præcision. Dette omfatter et visuelt prik-gitter (dot-grid), magnetisk snapping af noder til gitteret for at sikre snorlige diagrammer, samt understøttelse af trinløs zoom (via musens scrollhjul) og fri panorering (via mellemrumstast eller midterste museknap), så brugeren ubesværet kan overskue og organisere store modeller.

## 📋 Acceptance Criteria
- [x] `GraphCanvas` renderer et diskret, performant prik-gitter (dot-grid, 20px raster) i baggrunden, der tilpasser sig zoom-niveauet med LOD (Level of Detail så det ikke støjer ved udzoomning).
- [x] Nodernes dimensioner er låst til eksakte multipla af gitterstørrelsen (fx bredde 180px [9x20], højde 80px [4x20]), så alle 4 hjørner af noden rammer gitterpunkter præcist.
- [x] Magnetisk "Snap-to-Grid": Noders positioner snapper automatisk til nærmeste gitterpunkt (x, y som multipla af 20px) under træk eller ved slip, med toggle i værktøjslinjen.
- [x] Panorering (pan/scroll): Understøtter almindelig scroll, Shift + scroll til horisontal panorering, Space + venstre musetræk (Hand/grab cursor) og midterste museknap.
- [x] Zoom: Default 100% (1.0), min 25% (0.25) og max 150% (1.5). Ctrl/Cmd + scroll forankret i markørposition samt genvejstaster Ctrl/Cmd + `+`, `-`, `0`.
- [x] Værktøjslinjen under Fane 3 indeholder knapper til "Nulstil visning" (zoom 100%, pan 0,0), Snap-to-Grid toggle og statusindikator for aktuel zoom-procent.
- [x] Matematisk afkobling: Alle hit-tests, node-positioner og oprettelser regnes i verdenskoordinater via en dedikeret `CanvasViewport` transformation.
- [x] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE introducere unødig CPU/GPU belastning under rendering af gitteret (skal udnytte `canvas::Geometry` caching effektivt).
- Må IKKE forskyde koordinaterne i den underliggende datamodel utilsigtet (verdenskoordinater vs. skærmkoordinater skal være matematisk præcist afkoblet).
- Må IKKE tillade zoom ind ud over 150% eller zoom ud under 25%, for at undgå desorientering og numerisk ustabilitet.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for canvas ergonomi, zoom, pan og magnetisk gitter.
- 2026-09-19: Specificeret nodedimensioner som gitter-multipla (180x80 px), standardiseret genvejstaster (Space+drag, Shift+scroll, Ctrl+scroll/keys) og defineret zoom-grænser (25% - 150%, default 100%).

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 008-canvas-ergonomics-zoom-pan-grid`
- `xgauntlet verify --task 008-canvas-ergonomics-zoom-pan-grid`
