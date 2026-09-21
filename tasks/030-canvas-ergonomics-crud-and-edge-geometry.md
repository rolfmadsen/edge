---
type: Task Package
title: "Task 030: Canvas Ergonomi, Opret/Slet & Relations-geometri"
description: "Forbedre og professionalisere lærredsinteraktionen, sletningsarbejdsgange og relationsgeometrien på tværs af Begrebsmodellen og Informationsmodellen"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-20T16:14:00Z" }
tags: [canvas, ergonomics, crud, geometry]
---

# Task 030: Canvas Ergonomi, Opret/Slet & Relations-geometri

**Status**: DONE  
**Intent**: 🔄 ENHANCEMENT  

## 🎯 Formål
Forbedre og professionalisere lærredsinteraktionen, sletningsarbejdsgange og relationsgeometrien på tværs af Begrebsmodellen (Fane 2) og Informationsmodellen (Fane 3):
1. Fjerne redundante zoomknapper fra top-værktøjslinjen på canvas.
2. Tilføje `+ Opret begreb` og `+ Opret klasse` i canvas-værktøjslinjen, som opretter elementet centreret i viewporten.
3. Rette dobbeltklik på lærredet så elementer placeres præcist under cursoren (og centreret) frem for vilkårlig gitteroffset.
4. Ensrette relationshoveder og -markører (14.0 x 14.0) og symmetrisk linjestart/slut for ens knækhøjde.
5. Fravælge valgt relation ved enkeltklik på tomt canvas.
6. Tilføje skraldespandsikon `🗑️` i venstre palet for begreber og klasser, der ikke benyttes på lærredet, så de kan slettes permanent.
7. Rette statustekster til henholdsvis "x begreber på diagram • y relationer" og "x klasser på diagram • y relationer".

## 📋 Acceptance Criteria
- [x] **AC1 (Renset værktøjslinje)**: Hverken Begrebsmodellen eller Informationsmodellen viser `-`, `xxx%` eller `+` i lærredets øverste værktøjslinje.
- [x] **AC2 (Hurtigoprettelse i centrum)**: `+ Opret begreb` og `+ Opret klasse` i værktøjslinjen åbner oprettelse centreret i canvas-udsnittet.
- [x] **AC3 (Præcist dobbeltklik)**: Dobbeltklik på canvas i Informationsmodellen opretter klassen på klikpositionen frem for i en fast gitterberegning. I Begrebsmodellen centreres den nye node over klikket.
- [x] **AC4 (Ensartet relations-geometri)**: Generalisering og Komposition anvender samme ikonlængde/bredde (14.0) og symmetrisk stub-forskydning så ortogonale knæk flugter.
- [x] **AC5 (Deselect relation ved klik)**: Klik på tomt lærred i Begrebs- og Informationsmodellen nulstiller valgt relation/edge (`selected_edge = None`, `selected_info_edge = None`).
- [x] **AC6 (Permanent sletning fra palet)**: Elementer i venstre palet der har `!is_on_canvas` viser både `+` og `🗑️`. Klik på `🗑️` sletter begrebet/klassen permanent.
- [x] **AC7 (Ensartet tæller-tekst)**: Begrebsmodellen viser `"x begreber på diagram • y relationer"`, og Informationsmodellen viser `"x klasser på diagram • y relationer"`.

## 🚫 Must NOT
- Må IKKE ødelægge de svævende ergonomi-kontroller / minimap i nederste højre hjørne.
- Må IKKE tillade sletning af elementer via `🗑️`, hvis de aktuelt er placeret på lærredet (skal forblive `✓`).
- Må IKKE bryde FDA-regler for UML-relationer eller modelintegritet.

## 📝 Revisions
- 2026-09-20: Oprettet med 7 delkriterier godkendt af brugeren.

## 🧪 Verifikation
- `cargo test --test acceptance test_task_030`
- `cargo clippy --all-targets`
- `cargo fmt --check`
