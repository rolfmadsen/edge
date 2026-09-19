---
type: Task Package
title: "Task 007: Interaktiv Relation-håndtering, Drag-to-Connect & Edges"
description: "Visuel oprettelse af relationer ved at trække mellem noder, klikbare kanter, tastatursletning og inline associations-redigering"
status: open
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T15:16:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, relations, drag-to-connect, uml]
---

# Task 007: Interaktiv Relation-håndtering, Drag-to-Connect & Edges

**Status**: `SPEC`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Erstatte den statiske dialog-baserede oprettelse af relationer med en moderne visuel "Drag-to-Connect" arbejdsgang direkte på lærredet, samt gøre relationer (kanter) fuldt interaktive (klikbare, markérbare, sletbare via tastaturet, og med mulighed for direkte redigering af associationsnavne).

## 📋 Acceptance Criteria
- [ ] `GraphCanvas` viser visuelle forbindelsespunkter (connection handles) på noder ved hover eller markering.
- [ ] Brugeren kan trække med musen fra et forbindelsespunkt på Node A til Node B, ledsaget af en synlig interaktiv elastik-linje (connection preview).
- [ ] Når trækket slippes over Node B, vises en hurtig mini-vælger ved markøren: `[ ⮞ Generalisering ]` og `[ ── Association ]`.
- [ ] Hvis Association vælges, aktiveres straks et fokuseret tekstinput til associationsnavnet i naturligt sprog jf. FDA Modelreglerne Tabel 1.
- [ ] Relationer (kanter og labels) kan markeres ved klik på linjen, hvilket fremhæver kanten på canvaset og viser dens detaljer i inspektørpanelet.
- [ ] Tryk på `Delete` eller `Backspace` på tastaturet sletter den markerede relation eller node.
- [ ] Dobbeltklik på et associationsnavn på canvaset tillader direkte inline-redigering af navnet.
- [ ] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE tillade oprettelse af relationer til samme node (self-loop).
- Må IKKE tillade relationer, der peger på ugyldige eller slettede noder (dangling edges).
- Må IKKE blokere almindelig node-drag ved klik uden for forbindelsespunkterne.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for interaktiv relation-håndtering og drag-to-connect.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 007-canvas-interactive-edges-drag-connect`
- `xgauntlet verify --task 007-canvas-interactive-edges-drag-connect`
