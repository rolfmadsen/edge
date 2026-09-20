---
type: Task Package
title: "Task 027: Collab Protokol, Host/Guest Tilstande & Mutation Bridge"
description: "Definering af CollabPayload og ModelMutation, brobygning til Iced update-loopet, modtagelse af synkrone begivenheder samt disklås og deaktivering af autosave for Guest-klienten"
status: in_progress
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:55:00Z" }
tags: [collaboration, protocol, mutations, iced, autosave, disk-lock]
---

# Task 027: Collab Protokol, Host/Guest Tilstande & Mutation Bridge

**Status**: `IN_PROGRESS`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Definere hændelsesprotokollen i `src/features/collab/protocol.rs`:
   - `CollabPayload::Snapshot(ModelProject)` til initial synkronisering ved opkobling.
   - `CollabPayload::Mutation(ModelMutation)` for inkrementelle ændringer:
     - `ConceptAdded(Concept)`, `ConceptUpdated(Concept)`, `ConceptDeleted(Uuid)`
     - `InformationClassAdded(InformationClass)`, `InformationClassUpdated(...)`, `InformationClassDeleted(...)`
     - `RelationAdded(Relation)`, `RelationDeleted(Uuid)`
     - `NodeMoved { id: Uuid, x: f32, y: f32 }` (med throttling ved drag)
2. Etablere en tovejs bro i Iceds `update()`-funktion:
   - **Udgående:** Når en lokal bruger tilføjer/redigerer begreber eller flytter noder, dispatches en `ModelMutation`, som krypteres og sendes ud over WebSocket.
   - **Indgående:** Når en remote `ModelMutation` modtages og dekrypteres, integreres den direkte i appens aktive `ModelProject` i RAM uden at gen-udsende hændelsen (forebyggelse af ekko/loops).
3. Håndtere roller: `CollabRole::Host` vs. `CollabRole::Guest`:
   - **Værten (Host):** Bevarer sit normale autosave jf. ADR 003 (hver godkendt ændring gemmes atomisk til værtens lokale `model.edge.json`).
   - **Gæsten (Guest):** Autosave deaktiveres eksplicit! Gæstens lokale filrækkevidde låses, så gæstens egne lokale filer aldrig overskrives af data fra sessionen.
   - Gæsten forsynes i stedet med muligheden for manuelt at eksportere via "Gem som kopi...".

## 📋 Acceptance Criteria
- [ ] `CollabPayload` og `ModelMutation` er defineret med `serde::{Serialize, Deserialize}`.
- [ ] `src/ui/app.rs` udvides med `CollabState` (None, Host, Guest).
- [ ] Lokale handlinger i Begrebslisten, Begrebsmodellen og Informationsmodellen udsender tilhørende `ModelMutation`, når en session er aktiv.
- [ ] Canvas drag af noder throttles (maks 15 Hz) eller sendes ved `MouseReleased` for at undgå netværksmætning.
- [ ] Indgående hændelser muterer `ModelProject` i RAM og opdaterer visningen for alle faner (Begrebsliste, Begrebsmodel, Informationsmodel).
- [ ] Automatiserede tests beviser, at hvis `CollabState == Guest`, foretages der **aldrig** skrivning til `model.edge.json` ved modtagelse af mutationer.
- [ ] Værten kan uploade et fuldt snapshot ved opstart, som gæsten indlæser som erstatning for sit RAM-projekt ved tilslutning.
- [ ] `cargo test` og `cargo clippy -- -D warnings` passerer 100%.

## 🚫 Must NOT
- Må IKKE overskrive gæstens lokale `model.edge.json` fil under nogen omstændigheder.
- Må IKKE gen-broadcaste modtagne indgående mutationer tilbage til netværket (ingen loopbacks).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som opgave 3 i E2EE Live Collaboration serien jf. ADR 008.

## 🧪 Verifikation
- `cargo test test_mutation_bridge`
- `cargo test test_guest_autosave_suppressed`
- `cargo test --workspace`
