---
type: Architectural Decision Record
title: 'ADR 007: Canvas Studio Paradigmet og UML Klassediagram Canvas'
status: accepted
tags: [architecture, adr, canvas, studio-layout, uml, class-diagram, fda, information-model]
---

# 7. Canvas Studio Paradigmet og UML Klassediagram Canvas

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
I `edge` var informationsmodellen i Task 010 initialt udformet som en to-kolonne master-detail formular. Dette optog uhensigtsmæssigt meget skærmplads på statiske formularfelter frem for at give brugeren det visuelle, diagrammatiske overblik over klassestrukturer og relationer, som forventes i FDA-metoden og UML.
Samtidig har der i begrebsmodellen manglet en klar adskillelse mellem det underliggende repositorium (alle definerede elementer) og det konkrete diagram (en fokuseret projektion af udvalgte elementer), hvilket hidtil har resulteret i, at samtlige begreber automatisk blev dumpet på canvas.

## Decision
1. **Ensartet Canvas Studio Layout (3-delt opbygning)**:
   - **Venstre Palet (Repository Browser, ~240px)**: Viser alle projektets elementer, deres tilstedeværelse på det aktive canvas (`[✓]` / `[+]`), søgning og genveje til oprettelse.
   - **Center Canvas (Fill)**: Det dominerende arbejdsområde med interaktiv graf-redigering, pan, zoom, grid, snap og ortogonal routing.
   - **Højre Inspector (Context Panel, ~300px)**: Slank, kontekstafhængig sidebar, som ved valg af et diagramelement viser og redigerer dets attributter, beskrivelse og sporbarhed i realtid.
2. **UML 3-Sektions Klassekasser**:
   - Klasser tegnes på canvas som klassiske 3-rums UML-kasser jf. FDA Modelreglerne:
     - Topsektion: Stereotype `«Klasse»` og klassenavn med fed skrift.
     - Horisontal skillelinje.
     - Attributsektion: Attributter formateret som `+ name : Type [multiplicitet]`.
     - Dynamisk beregnet kassehøjde ud fra antallet af attributter.
3. **Persistens via `information_graph: ClassGraph`**:
   - Diagram-specifikke layout-data (`x, y, width, height` for kasser og associations-/generaliseringskanter) persisteres separat fra den semantiske datamodel via en dedikeret `ClassGraph` i `ModelProject`.
   - `#[serde(default)]` sikrer 100% bagudkompatibilitet med ældre gemte modeller.
4. **Fail-Closed Integritet**:
   - Sletning af en klasse kaskadesletter tilhørende node i `ClassGraph` og alle tilknyttede relationer for at eliminere "dangling edges".

## Consequences
- **Positive**:
  - Markant bedre brugeroplevelse og udnyttelse af skærmarealet med canvas i centrum.
  - Ensartet workflow mellem begrebsmodel og informationsmodel.
  - Fuld overensstemmelse med UML- og FDA-standarder for informationsmodeldiagrammer.
- **Trade-offs**:
  - Kræver synkronisering mellem den semantiske model (`InformationClass`) og dens visuelle projektion (`ClassDiagramNode`).

