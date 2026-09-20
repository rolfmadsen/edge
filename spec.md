---
type: System Specification
title: Specification - Kant Architecture & Capabilities
description: Macro system architecture, philosophy, and invariants for Kant
status: active
generated: { by: process:xgauntlet-init, at: "2026-09-06T18:00:00Z" }
tags: [specification, architecture, invariants]
---

# Specification: Kant Architecture & Capabilities

## 🎯 Philosophy & Core Capabilities
1. **Clean Architecture & Autonomous Features**:
   - Autonome feature-moduler med klare ansvarsområder og veldefinerede grænseflader (Screaming Architecture).
2. **FDA Modelregler & Metodisk Progression**:
   - Understøttelse af Digitaliseringsstyrelsens regler for begrebs- og datamodellering (v2.1).
   - Tretrins arbejdsgang: Begrebsliste (tabel) -> Begrebsmodel (graf) -> Informationsmodel (klasser/attributter).
3. **The Elm Architecture (TEA) Desktop UI**:
   - Reaktiv og type-sikker brugerflade i Iced adskilt fra det rene domænelag.
4. **Deterministic Verification**:
   - Multi-layer verifikations-pipeline styret af deklarativ konfiguration (`gauntlet.toml`).

## 📐 Architecture & Feature Modules
- `src/features/model/`: Overordnet FDA modelcontainer og metadata (§06-§13).
- `src/features/concepts/`: Begreber, termer, synonymer og strukturerede definitioner (Bilag D & E).
- `src/features/concept_model/`: Begrebsmodel-graf med noder, generaliseringer og associationer.
- `src/features/information_model/`: Informationsmodel med klasser, attributter, standard datatyper og multipliciteter.
- `src/features/collab/`: E2EE krypto, sessionsbilletter, WebSocket sync og mutation bridge.
- `crates/kant-relay/`: Ultralet, in-memory, blind Axum WebSocket pub/sub relay.
- `src/ui/`: Iced shell, faner, tema og præsentationslogik.
- `tasks/`: Eksekverbare opgavepakker med acceptkriterier.
- `docs/adr/`: Arkitektoniske beslutningsreferater.

## 🚫 Must NOT (System Invariants)
- Må IKKE introducere skjulte runtime-afhængigheder eller udokumenterede baggrundsprocesser (Zero-Daemon).
- Må IKKE omgå deklarative verifikationslag eller ignorere fejlede tests.
- Må IKKE sammenblande FDA forretningsregler og validering med UI-renderingswidgets.
- Må IKKE foretage utilsigtede remote publication kommandoer (`git push`).

## 🧪 Multi-Layer Verification Contracts
- [ ] 100% test pass rate på alle konfigurerede lag.
- [ ] 0 linter- og typechecker-advarsler.
