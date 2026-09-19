---
type: Architectural Decision Record
title: 'ADR 002: Begrebsliste CRUD, Iced Tabel & Theme Integration'
status: accepted
tags: [architecture, adr, iced, fda, crud, theme, concepts]
---

# 2. Begrebsliste CRUD, Iced Tabel & Theme Integration

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
Projektet `edge` understøtter Fællesoffentlig Digital Arkitektur (FDA) Modellering jf. `docs/fda_modelleringsvejledning.md`.
Første faglige arbejdstrin er udarbejdelse af en begrebsliste (tabelformat jf. Bilag D og E).
Brugeren har brug for at oprette, gennemse, redigere og slette begreber i en overskuelig tabel med de 12 standardiserede metadatafelter, med primært fokus på: Foretrukken dansk term, Definition, Kilder og Tilhører emneområde. Samtidig skal brugerfladen fremstå moderne og indbydende frem for basalt råt rå-layout, og overholde FDA's visuelle anbefalinger fra vejledningens Kapitel 7.3.

## Decision
1. **Domænelag (DDD)**:
   - `ModelProject` i `src/features/model/mod.rs` fungerer som aggregate root for `Concept`-entiteter (`concepts: Vec<Concept>`).
   - CRUD-operationer (`add_concept`, `update_concept`, `remove_concept`, `get_concept`) håndhæver FDA-invarianter via `ConceptValidator`.
   - Domænelaget forbliver 100% uafhængigt af Iced og GUI-widgets.

2. **Iced UI Arkitektur (TEA)**:
   - UI opdeles i to dedikerede komponenter i `src/ui/`:
     - `concept_table`: Håndterer visning af begreber i tabelform med søgefiltrering, kolonneopdeling og handlingsknapper.
     - `concept_editor`: Håndterer en dedikeret formular til oprettelse og redigering af et begrebs felter med direkte valideringsfeedback.
   - TEA-beskeder (`Message`) orkestrerer dataflowet entydigt uden mutable delte tilstande.

3. **Visuel Profil & Theme**:
   - Vi benytter Iceds native `iced::Theme::Light` som generelt tema via `App::theme`.
   - Vi beriger farvepaletten i `src/ui/theme.rs` med de officielle FDA-farver fra vejledningens Kapitel 7.3:
     - Sandfarvet (`#FEFAF7`) til egne/lokale begreber i emneområdet.
     - Blå (`#87CDEB`) til genbrugte/indlånte begreber fra andre modeller.
     - Kontrasterende, elegante kanter og status-farver for at sikre et professionelt, moderne udtryk.

## Consequences
- **Positive**:
  - Høj modularitet: Tabel og editor kan videreudvikles eller erstattes uafhængigt.
  - Komplet headless testbarhed af både domæneoperationer og UI TEA-beskeder.
  - Fuld overensstemmelse med de officielle FDA-regler for begrebslister.
- **Trade-offs**:
  - Alle 12 felter fra Bilag D & E i én enkelt vandret tabelrække ville give horisontal scroll og dårlig UX; derfor kombineres et fokuseret tabeloverblik med et detaljeret editor/inspektør-panel.
