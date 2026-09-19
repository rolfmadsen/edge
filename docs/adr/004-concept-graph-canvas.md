---
type: Architectural Decision Record
title: 'ADR 004: Interaktiv Begrebsmodel i Iced Canvas med FDA-semantik'
status: accepted
tags: [architecture, adr, canvas, iced, graph, uml, fda, concepts]
---

# 4. Interaktiv Begrebsmodel i Iced Canvas med FDA-semantik

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
I FDA modelleringsmetodikken (§§18-26, Kapitel 5 og Kapitel 7) udgør den terminologiske begrebsmodel (grafen) broen mellem den lineære begrebsliste og den formelle informationsmodel.
Brugeren skal kunne visualisere begreber som noder med officielle FDA-farver (Sand `#FEFAF7` for lokale og Blå `#87CDEB` for indlånte begreber), etablere UML-relationer (Generaliseringer med hvid trekant-pil og Associationer med associationsnavn i naturligt sprog), flytte noder rundt med musen (drag-and-drop), og bevare grafens koordinater og relationer i projektfilen (`.edge.json`).

## Decision
1. **Brug af `iced::widget::canvas` uden eksterne bindings**:
   - Vi implementerer grafvisningen direkte via Iced's indbyggede 2D canvas API (`canvas::Program`). Dette bevarer Zero-Daemon og offline sandkasse-integriteten uden behov for tunge C-biblioteker som Graphviz.
2. **FDA Semantisk Rendering**:
   - Lokale begreber gengives med FDA Sand (`#FEFAF7`) og mørk ramme.
   - Indlånte begreber gengives med FDA Blå (`#87CDEB`).
   - Generaliseringer tegnes som linjer med hvid, lukket trekant-pil pegende mod superklassen jf. FDA Modelreglerne Tabel 1.
   - Associationer tegnes som forbindelseslinjer med centreret tekst-label i naturligt sprog.
3. **Graf-persistens i `ModelProject`**:
   - `ConceptGraph` integreres direkte i `ModelProject`, så noder, deres $(x, y)$ positioner og relationer serialiseres/deserialiseres deterministisk i projektfilen (`.edge.json`).
4. **Fail-Closed Integritet (Ingen Dangling Edges)**:
   - Sletning af et begreb kaskadeslettes i grafens nodeliste og fjerner alle tilknyttede relationer.

## Consequences
- **Positive**:
  - Høj ydeevne og GPU-accelereret rendering via Iced WGPU.
  - 100% overholdelse af Digitaliseringsstyrelsens FDA-standarder for begrebsdiagrammer.
  - Fuld persistens og synkronisering med autosave.
- **Trade-offs**:
  - Manuel placering af noder initialt med grid-flow; avanceret organisk layout-algoritme udskydes til senere optimering.
