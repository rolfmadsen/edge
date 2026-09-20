---
type: Architectural Decision Record
title: 'ADR 009: Rebranding from Edge to Kant and Extension Governance'
status: accepted
tags: [architecture, adr, branding, storage]
---

# 009. Rebranding from Edge to Kant and Extension Governance

**Status**: `accepted`  
**Date**: `2026-09-20`  

## Context
Applikationen blev oprindeligt døbt "Edge" for at afspejle visualiseringen af forbindelser (edges) i begrebs- og informationsgrafer. Dette navn skabte imidlertid uhensigtsmæssig forveksling med Microsoft Edge-webbrowseren i desktop- og distributionssammenhænge.

Der opstod derfor behov for at rebrande applikationen til et entydigt, prægnant navn: **Kant**.
Navnet "Kant" forener den danske betydning for en graf-kant/relation med referencen til filosoffen Immanuel Kant og FDA-domænets epistemologiske begrebsafklaring.

I kodebasen optræder ordet "edge" dog i to vidt forskellige kontekster:
1. Som produktnavn, binærnavn, crate-navn og filendelse (`model.edge.json`).
2. Som datalogisk og grafteoretisk terminologi for relationer mellem grafelementer (`Node` vs `Edge`).

## Decision
1. **Produkt- og Crate-omdøbning:**
   - Hoved-crate omdøbes fra `edge` til `kant`.
   - Relay-kassen omdøbes fra `edge-relay` til `kant-relay`.
   - Skrivebordsapplikationens eksekverbare binær navngives `kant` (og `kant.exe`).
   - OS-pakker og ikoner navngives `kant.desktop`, `kant.svg` og `kant_*_amd64.deb`.
2. **Filformat og Bagudkompatibilitet:**
   - Standard filendelse og standardnavn for FDA-modeller fastsættes til `*.kant.json` (standard: `model.kant.json`).
   - Native filvælgere og filscannere SKAL opretholde transparent bagudkompatibilitet for eksisterende `*.edge.json`-filer, så eksisterende brugerdata kan åbnes uden migreringsbarrierer.
3. **Intern Graf-terminologi bevares på engelsk:**
   - Interne grafteoretiske datastrukturer i kildekoden (`ClassDiagramEdge`, `DiagramEdge`, `EdgeRouter`, `CanvasEdge`, `edges: Vec<...>`) bevares på idiomatisk engelsk (`Edge`). Kodebasen må IKKE inficeres med et kunstigt dansk/engelsk sprogmiks (f.eks. `kant_router` eller `DiagramKant`).

## Consequences
- **Positive:**
  - Ingen navnesammenfald med tredjepartsapplikationer (Microsoft Edge).
  - Skarp separation mellem det eksterne produktbrand (Kant) og det interne matematiske domænesprog (Nodes & Edges).
  - Brugerne kan gnidningsløst åbne ældre modelprojekter via bagudkompatibilitetsfilteret.
- **Negative / Trade-offs:**
  - Rust-moduler, tests og import-stier skal opdateres fra `use edge::...` til `use kant::...`.
  - CI/CD build scripts og udrulningsspecifikationer (`koyeb.yaml`, `release.yml`) skal synkroniseres til de nye binære navne.
