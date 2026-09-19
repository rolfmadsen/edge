---
type: Architectural Decision Record
title: 'ADR 001: Rust, Iced Framework og Domæne/UI-Separation for FDA Modellering'
status: accepted
tags: [architecture, adr, iced, fda, domain-driven-design]
---

# 1. Rust, Iced Framework og Domæne/UI-Separation for FDA Modellering

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
Projektet `edge` har til formål at udvikle en desktop-applikation, der understøtter den metodiske udarbejdelse af begrebsdefinitioner, begrebsmodeller og informationsmodeller efter principperne i Fællesoffentlig Digital Arkitektur (FDA) jf. Digitaliseringsstyrelsens Modelregler.

Arbejdsgangen følger en sekventiel progression:
1. Udarbejdelse af begrebsliste (tabelformat jf. Bilag D/E)
2. Udarbejdelse af begrebsmodel (visuel graf med noder, generaliseringer og associationer)
3. Udarbejdelse af informationsmodel (klasser, attributter, datatyper og multipliciteter)

Der stilles krav om høj ydeevne, cross-platform understøttelse, nul runtime-overhead, høj pålidelighed og automatiseret deterministisk verifikation (`xgauntlet verify`).

## Decision
1. **Teknologistack**:
   - Vi vælger **Rust** som sprog på grund af type-sikkerhed, robust hukommelseshåndtering og høj performance.
   - Vi vælger **Iced (v0.14)** som GUI-framework, baseret på *The Elm Architecture* (TEA: `State`, `Message`, `Update`, `View`).
   - Som rendering backend anvender vi `wgpu` som standard med `tiny-skia` som softwarefallback.

2. **Arkitektonisk adskillelse (Screaming Architecture & DDD)**:
   - Al FDA-domænelogik (begreber, metadata, strukturerede definitioner, relationer og valideringsregler) placeres i rene, GUI-uafhængige moduler under `src/features/`.
   - UI-laget i `src/ui/` afhænger af domænelaget, men domænelaget har absolut ingen kendskab til Iced eller grafiske widgets.
   - Dette sikrer, at 100% af forretningslogik, validering og serialisering kan testes i headless enheds- og integrationstests uden display-server eller GPU.

3. **Data- og filformat**:
   - Modelprojekter serialiseres som menneskelæsbar JSON/YAML, hvilket understøtter versionsstyring i Git og muliggør fremtidig eksport til maskinlæsbare FDA-kataloger (f.eks. JSON-LD, RDF/Turtle, CSV).

## Consequences
- **Positive konsekvenser**:
  - Lynhurtig automatiseret verifikation under CI/Gauntlet.
  - Klar ansvarsfordeling mellem domæneinvarianter og brugergrænseflade.
  - Komplet sporbarhed fra rå begreber til grafnoder og videre til informationsmodel-attributter.
- **Trade-offs / Opmærksomhedspunkter**:
  - Iceds TEA-model kræver, at al interaktion kanaliseres gennem eksplicitte beskeder (`Message`), hvilket minimerer utilsigtede sideeffekter, men kræver disciplineret beskeddesign for komplekse operationer som graf-manipulation.
