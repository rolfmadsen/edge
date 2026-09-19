---
type: Task Package
title: "Task 007: Interaktiv Relation-håndtering, Drag-to-Connect & Edges"
description: "Visuel oprettelse af relationer ved at trække mellem noder, klikbare kanter, tastatursletning og inline associations-redigering"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T15:16:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, relations, drag-to-connect, uml]
---

# Task 007: Interaktiv Relation-håndtering, Drag-to-Connect & Edges

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Erstatte den statiske dialog-baserede oprettelse af relationer med en moderne, robust "Drag-to-Connect" arbejdsgang direkte på lærredet:
Når en node vælges, vises et tydeligt forbindelseshåndtag (connect handle), hvorfra brugeren kan trække en elastik-linje og slippe på en målnode. Ved slip oprettes en Association som standard, og relationen markeres straks som et selvstændigt objekt i inspektørpanelet med fokus på label og hurtigt skift af relationstype. Relationer gøres desuden fuldt interaktive (klikbare på linje/label på canvas, sletbare via Delete/Backspace).

## 📋 Acceptance Criteria
- [x] Når en node er markeret på lærredet, vises et synligt forbindelseshåndtag (connect handle).
- [x] Klik-og-træk fra forbindelseshåndtaget på Node A starter en elastik-preview-linje mod musemarkøren.
- [x] Målnode B highlightes som gyldigt slip-mål under træk (forudsat $B \neq A$).
- [x] Ved slip over en målnode oprettes relationen automatisk som `RelationKind::Association`, og relationen markeres straks.
- [x] Den valgte relation åbner en dedikeret relations-inspektør i højre panel med fokus på navnefeltet (label) og hurtig ændring af relationstype (Association, Generalisering, Komposition).
- [x] Relationer på canvaset kan klikkes og markeres direkte som et selvstændigt objekt (visuel fremhævning af linjen).
- [x] Tastaturet (`Delete` eller `Backspace`) sletter det aktuelt markerede objekt (enten valgt relation eller valgt node).
- [x] Dobbeltklik på en relations label eller klik på linjen åbner/aktiverer relations-inspektøren.
- [x] Parallelle relationer mellem noder fordeler porte rumligt (spatial sorting) langs nodens kant, så unødige linjekrydsninger elimineres.
- [x] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE tillade oprettelse af relationer til samme node (self-loop).
- Må IKKE tillade relationer, der peger på ugyldige eller slettede noder (dangling edges).
- Må IKKE blokere almindelig node-drag ved klik uden for forbindelseshåndtaget.
- Må IKKE benytte skrøbelige/flimsy overlay- eller modal-dialoger til oprettelse; inspektørpanelet er kilden til egenskaber.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for interaktiv relation-håndtering og drag-to-connect.
- 2026-09-19: Design afstemt: Connect-handle på valgt node (model 1b), drop på målnode opretter Association som default, relation som selvstændigt objekt, og egenskaber redigeres direkte i inspektørpanelet uden modaler.
- 2026-09-19: Fejlrettelse: Rumlig sortering af forbindelsesporte langs nodens kant implementeret for at forhindre at parallelle relationer krydser hinanden unødigt.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 007-canvas-interactive-edges-drag-connect`
- `xgauntlet verify --task 007-canvas-interactive-edges-drag-connect`
