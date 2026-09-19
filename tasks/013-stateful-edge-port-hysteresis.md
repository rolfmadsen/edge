---
type: Task Package
title: "Task 013: Stateful Edge Port Hysteresis & Persistence"
description: "Tilstandsbaseret port-routing med geometriske tærskler og persistens af relationers aktive porte i modellen"
status: active
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T21:10:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, relations, routing, hysteresis, ports]
---

# Task 013: Stateful Edge Port Hysteresis & Persistence

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Forbedre port-routingen for diagram-relationer så relationer ikke flimrer eller skifter port uhensigtsmæssigt, når noder placeres i hjørnekvadranterne. Implementere tilstandsbaseret hysterese baseret på geometriske grænsetærskler (nodens bounding-box forlængelser) og persistere de aktive porte (`source_port` og `target_port`) i datamodellen (`model.edge.json`) med fuld bagudkompatibilitet.

## 📋 Acceptance Criteria
- [ ] `PortSide` (`Top`, `Right`, `Bottom`, `Left`) er en first-class type i domænemodellen med serde-understøttelse.
- [ ] `DiagramEdge` og `ClassDiagramEdge` persisterer `source_port` og `target_port` som valgfrie felter (`Option<PortSide>`).
- [ ] Eksisterende JSON-modeller uden port-felter kan indlæses uden fejl (bagudkompatibilitet).
- [ ] `EdgeRouter::select_ports` bevarer `Right` port på kilden, når målnoden er til højre for kildens højre kant (`target.x >= source.right`), selv hvis noden flyttes op eller ned i hjørnekvadranterne.
- [ ] Først når målnoden trækkes ind over den vertikale grænselinje (`target.x < source.right`), skifter kildens port baseret på relativ vertikal placering (`Bottom` hvis under centrum, `Top` hvis over centrum).
- [ ] `EdgeRouter::select_ports` bevarer `Bottom` port på kilden, når målnoden er under kildens bundkant (`target.y >= source.bottom`), indtil noden trækkes op over bundkanten.
- [ ] Symmetrisk hysterese gælder for `Left` og `Top` porte samt for målnodens tilknyttede port.
- [ ] Generaliseringspile forbinder lateralt til højre/venstre side uden baglæns knuder, når subklassen er forskudt til siden.
- [ ] Flytning af noder på lærredet (`on_node_moved`) opdaterer de aktive porte i grafen, så de gemmes ved autosave.
- [ ] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE bryde eksisterende serialisering eller ødelægge eksisterende `model.edge.json` filer.
- Må IKKE introducere flimmer/jitter langs 45-graders diagonalen.
- Må IKKE tillade regressioner i linjekrydsnings-detektion (bridges) eller slot-offsets.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Oprettet som Model A (Stateful Hysteresis Port Selection & Persistence) godkendt af brugeren.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 013-stateful-edge-port-hysteresis`
- `xgauntlet verify --task 013-stateful-edge-port-hysteresis`
