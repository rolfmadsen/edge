---
type: Task Package
title: "Task 064: Drag-and-Drop fra Palet til Canvas og Værktøjslinje Harmonering"
description: "Drag-and-drop af begreber, klasser og enumerations fra venstrepalet til diagramcanvas i Begrebsdiagram og Informationsmodel, flytning af + Opret enumeration til canvas-værktøjslinjen, samt sanering af Snap: Til/Fra knap og gitter-snapping"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-01T21:05:00Z" }
tags: [canvas, drag-and-drop, palette, ui, toolbar, ergonomics, information-model, concept-model]
---

# Task 064: Drag-and-Drop fra Palet til Canvas og Værktøjslinje Harmonering

**Status**: `DONE`  
**Intent**: `🔄 ENHANCEMENT`  
**Oprettet**: `2026-10-01`  
**Scope**: `src/ui/`, `src/features/`, `tests/`

---

## 🎯 Formål
Forbedre diagram-ergonomien og ensrette brugergrænsefladen på tværs af Begrebsdiagram og Informationsmodel:

1. **Værktøjslinje-harmonisering (Informationsmodel)**:
   - Flyt knappen `+ Opret enumeration` til canvas-værktøjslinjen i Informationsmodel (Fane 3), så den står side om side med `+ Opret klasse` og `+ Opret Relation`.
   - Ensret venstrepaletten, så sektionen for Enumerationer matcher Klasser uden en afvigende `+ Opret`-knap i palet-overskriften.

2. **Drag-and-Drop fra Palet til Canvas**:
   - Gør det muligt at trække ikke-tilføjede begreber (i Fane 2 Begrebsdiagram), klasser og enumerationer (i Fane 3 Informationsmodel) direkte fra venstrepanelet og slippe dem på lærredet.
   - Den nye diagramnode placeres præcist ved markørens slip-position (omregnet til canvas-verdenskoordinater under hensyntagen til zoom og pan).
   - Bevar det eksisterende lille `+`-ikon i paletten som et hurtigt 1-klik alternativ.

3. **Always-On Snap-to-Grid ("Snap: Til/Fra" fjernet)**:
   - Fjern "Snap: Til/Fra"-knappen fra både Begrebsdiagram og Informationsmodel værktøjslinjerne, så snap-to-grid bliver permanent aktiv som default (Always-On) og ikke kan fravælges ved en fejl.
   - Gitter-snapping (20 px) håndhæves konsekvent ved drop fra palet, oprettelse og node-flytninger.

---

## 📋 Acceptance Criteria
- [x] **AC1 - Canvas Værktøjslinje Harmonering**: Knappen `+ Opret enumeration` er placeret i værktøjslinjen over Informationsmodel-canvas sammen med `+ Opret klasse` og `+ Opret Relation`. Venstrepalettens header for Enumerationer er ensrettet med Klasser (ingen overflødig `+ Opret`-knap i headeren).
- [x] **AC2 - Drag-and-Drop af Begreber på Begrebsdiagram**: Ikke-placerede begreber i Begrebsdiagrammets palet kan trækkes ud på diagramcanvas og placeres ved markørens slip-position med magnetisk snapping til 20 px gitteret (verdenskoordinater under hensyntagen til zoom og pan).
- [x] **AC3 - Drag-and-Drop af Klasser og Enumerationer på Informationsmodel**: Ikke-placerede klasser og enumerationer i Informationsmodellens palet kan trækkes ud på UML-canvas og placeres ved markørens slip-position med magnetisk snapping til 20 px gitteret.
- [x] **AC4 - Bevarelse af 1-klik `+` Tilføjelse**: Det eksisterende `+` ikon i paletten for u-tilføjede elementer bevares som et hurtigt 1-klik alternativ, der placerer elementet i viewportens centrum eller ledigt område.
- [x] **AC5 - Always-On Snap-to-Grid**: Knappen "Snap: Til/Fra" er fjernet fra værktøjslinjen, så gitter-snapping er en fast, permanent standard (Always-On), der ikke kan deaktiveres. Både drop og flytning snapper magnetisk til 20 px gitteret.
- [x] **AC6 - Accepttest Verifikation**: En samlet accepttest `test_task_064_drag_and_drop_from_palette_to_canvas_and_toolbar_refinements` validerer værktøjslinjeknapper, drop-adfærd og den permanente Always-On gitter-snapping.

---

## 🚫 Must NOT
- Må IKKE fjerne det eksisterende `+` ikon i paletten; drag-and-drop skal være et intuitivt supplement.
- Må IKKE tillade duplikerede noder for elementer, der allerede er placeret på canvas.
- Almindelige klik på et element i paletten for at vælge det må IKKE utilsigtet trigge drop-handlinger eller flytte eksisterende noder.
- Ingen eksterne tunge afhængigheder må tilføjes; benyt eksisterende Iced event/subscription arkitektur.

---

## 📝 Revisions
- 2026-10-01: Oprettet opgavepakke efter brugerdialog om værktøjslinje, DND og fjernelse af grid snapping.

---

## 🧪 Verifikation
```bash
cargo test --test acceptance test_task_064
xgauntlet check-spec -t 064
xgauntlet verify -t 064 -s
```
