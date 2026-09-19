---
type: Task Package
title: "Task 004: Begrebsmodel Interaktiv Grafkomponent & Canvas"
description: "Etablering af interaktiv begrebsdiagram-graf i Iced Canvas med FDA-farver, generaliseringer, associationer og node-drag"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T12:06:00Z" }
tags: [task-lifecycle, intent, scaffolding, rust, iced, canvas, graph, fda-model]
---

# Task 004: Begrebsmodel Interaktiv Grafkomponent & Canvas

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Etablere en interaktiv visuel begrebsdiagram-komponent under Fane 3 ("3. Begrebsmodel (Graf)") ved hjælp af `iced::widget::canvas`, der renderer begreber som noder med FDA-farvekoder (sandfarvet `#FEFAF7` for lokale og blå `#87CDEB` for indlånte), understøtter UML-generaliseringer (hvid trekant-pil mod superklasse) og associationer med associationsnavne jf. FDA Modelleringsvejledning Kapitel 5 & 7, tillader drag-and-drop af noder, oprettelse af relationer og synkronisering med projektpersistensen.

## 📋 Acceptance Criteria
- [x] `ModelProject` i `src/features/model/mod.rs` indeholder `concept_graph: ConceptGraph`, som serialiseres og deserialiseres deterministisk via `ProjectStorage`.
- [x] `ConceptGraph` i `src/features/concept_model/mod.rs` udvides med metoder til synkronisering med `project.concepts()`, kaskadesletning af relationer når begreber fjernes, samt oprettelse og fjernelse af relationer (Generalisering & Association).
- [x] `src/ui/graph_canvas.rs` implementerer `iced::widget::canvas::Program` og renderer:
  - Noder med afrundede hjørner, FDA-farver (Sand `#FEFAF7` / Blå `#87CDEB`), mørk kant og foretrukken term centreret.
  - Generaliseringer med forbindelseslinje og hvid lukket trekant-pil pegende mod superklassen jf. FDA Tabel 1.
  - Associationer med forbindelseslinje og tekst-label for associationsnavnet i naturligt sprog jf. FDA Tabel 1.
- [x] Interaktivitet på lærredet:
  - Noder kan vælges ved klik (fremhævet kant) og trækkes rundt (drag-and-drop) med musen.
  - Valgt node viser sine FDA-detaljer (foretrukken term, definition, tilhører emneområde, kilder) i et inspektionspanel.
- [x] Værktøjslinje under Fane 3 stiller knapper til rådighed for:
  - "⟳ Synkroniser begreber": Tilføjer eventuelle nye begreber fra listen til grafen i et overskueligt gitter.
  - "＋ Ny relation": Formular/dialog til at vælge Kilde, Mål og Relationstype (Generalisering eller Association med navn).
  - "✕ Slet relation": Mulighed for at fjerne en eksisterende relation.
- [x] Ændringer i grafens layout (drag drop) og relationer udløser automatisk autosave til projektfilen.
- [x] 100% test pass rate på unit-, graf- og accepttests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE introducere eksterne tunge graph-rendering biblioteker eller eksterne dæmoner (Zero-Daemon).
- Må IKKE tillade dangling edges (relationer der refererer til ikke-eksisterende begreber).
- Må IKKE blande informationsmodel-felter (attributter/datatyper) ind i begrebsmodellen jf. FDA faseopdeling.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for Begrebsmodel Grafkomponent & Canvas.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 004-concept-graph-canvas`
- `xgauntlet verify --task 004-concept-graph-canvas`
