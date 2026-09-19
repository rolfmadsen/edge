---
type: Architectural Decision Record
title: 'ADR 003: Deterministisk Projektpersistens, Atomisk Skrivning og Autosave'
status: accepted
tags: [architecture, adr, persistence, storage, autosave, git, json]
---

# 3. Deterministisk Projektpersistens, Atomisk Skrivning og Autosave

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
Projektet `edge` understøtter udarbejdelse af FDA-modeller. For at sikre mod datatab og understøtte versionsstyring i Git ("Model-as-Code") er der brug for en robust persistensløsning.
Brugeren ønsker automatisk gemning (Autosave) af ændringer, men filerne må under ingen omstændigheder korrumperes ved nedbrud eller indeholde uvaliderede, ufærdige kladder. Samtidig skal modeller kunne gemmes og åbnes eksplicit fra vilkårlige stier.

## Decision
1. **Deterministisk JSON-format (`.edge.json`)**:
   - Modelprojekter serialiseres som pretty-printed JSON med standard 2-space indentation.
   - Nøgler og begreber serialiseres i deterministisk rækkefølge, så Git-diffs altid er rene og entydige ved commits og pull requests.

2. **Atomisk Skrivning (Crash-Resistent)**:
   - Filer skrives aldrig direkte til målfilen. I stedet skrives der først til en midlertidig fil (`<sti>.tmp`), hvorefter den omdøbes atomisk via operativsystemets `fs::rename`. Dette sikrer, at afbrydelser (f.eks. strømsvigt) aldrig kan efterlade en korrupt eller halvskrevet projektfil.

3. **Operation-baseret Autosave (Todelingsmodellen: Kladde vs. Commit)**:
   - Mens et begreb indtastes i editoren, holdes data i den lokale editor-tilstand for at fange valideringsfejl.
   - Så snart ændringen committes til `ModelProject` (ved "Gem begreb", redigering eller sletning), udløses autosave øjeblikkeligt og atomisk til disk.

4. **Automatisk Genoptagelse**:
   - Hvis der findes en standard projektfil (`model.edge.json`) i arbejdsmappen ved applikationens start, indlæses den automatisk som udgangspunkt.

## Consequences
- **Positive**:
  - Nul datatab: Brugeren behøver aldrig huske at trykke "Gem".
  - Fuld Git-kompatibilitet: Filer kan committes, diffes og merges i Git.
  - Ingen database- eller server-afhængighed: Rent Zero-Daemon og Zero Ambient Authority.
- **Trade-offs**:
  - Samtidig redigering af den samme fil på disken fra flere processer håndteres ikke med fillåsning i denne fase; dette løses i den fremtidige server-synkroniseringsfase.
