---
type: Task Package
title: "Task 029: Inbound Collab Subscription, Transport Framing & Relay Hardening"
description: "Udbedring af indgående netværkskanal i Iced App via subscription, eksplicit transport-framing mod nonce-kollision, kryptografisk replay-beskyttelse og DoS-hærdning af relay"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T15:36:00Z" }
tags: [collaboration, e2ee, iced, subscription, framing, security, hardening]
---

# Task 029: Inbound Collab Subscription, Transport Framing & Relay Hardening

**Status**: `DONE`
**Intent**: `🐛 BUG FIX` / `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. **Løse den blokerende indgående netværks-pipeline i GUI (`src/ui/app.rs`):**
   - Etablere en Iced `Subscription`, som lytter på `CollabChannel` og modtager netværkshændelser i den kørende applikation.
   - Dekryptere modtagne pakker med sessionsnøglen og dispatche til `Message::CollabApplySnapshot` og `Message::CollabApplyMutation`.
2. **Eksplicit Transport Framing mod Nonce-kollision (`protocol.rs` & `edge-relay`):**
   - Indføre `FrameType` (Snapshot = `0x01`, Mutation = `0x02`, Presence = `0x03`, HostLeft = `0x04`).
   - Fjerne vilkårlig inspektion af krypterede råbytes i relayen; relayen cacher udelukkende frames med `0x01` præfiks og router blindt.
3. **Kryptografisk Replay-beskyttelse (`protocol.rs`):**
   - Indkapsle payloads i `CollabEnvelope` med sekvensnummer (`seq: u64`) og tidsstempel (`timestamp: u64`).
   - Klienter afviser frames med `seq <= last_received_seq`.
4. **Relay Hardening mod DoS/OOM (`crates/edge-relay`):**
   - Maksimal payloadgrænse (5 MB).
   - Rumbegrænsning (maks. 1.000 aktive rum i RAM).
   - Udsendelse af live presence frame (`0x03`) ved tilslutning/frakobling.
5. **Vært Afbrydelse (Graceful Exit):**
   - Værten udsender `HostLeft` frame (`0x04`) før frakobling, hvilket udløser `notify_host_ended_session()` hos alle gæster.

## 📋 Acceptance Criteria
- [x] `FrameType` og `CollabEnvelope` er implementeret med serde og enheds-tests.
- [x] `CollabChannel` udsender frames med eksplicit `FrameType` præfiks.
- [x] `edge-relay` håndhæver maks payload-størrelse og udsender presence opdateringer (`0x03`).
- [x] `App::subscription` indeholder en aktiv lytter på kollaborationskanalen, som modtager snapshots, mutationer, presence og host-exit.
- [x] Modtagne mutationer opdaterer modelsandheden i RAM uden at gen-broadcaste til netværket.
- [x] Gæster modtager omgående opdateret deltagerantal i headeren.
- [x] Hvis værten afbryder sessionen, modtager gæster `GuestEndedNoticeModalState` med mulighed for "Gem som kopi...".
- [x] Automatiseret E2E accepttest beviser reel tovejs synkronisering mellem to forbundne `App` instanser.
- [x] `cargo clippy --workspace -- -D warnings` og `cargo test --workspace` passerer 100%.

## 🚫 Must NOT
- Må IKKE tillade rå kryptering uden replay-beskyttelse.
- Må IKKE tillade overskrivning af gæstens lokale diskfiler.
- Må IKKE lække kryptonøgler i relay-log eller URL'er.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet efter arkitektonisk gennemgang og opdagelse af de to kritiske P0-defekter.

## 🧪 Verifikation
- `cargo test --package edge -- features::collab`
- `cargo test -p edge-relay`
- `cargo test test_task029_e2e_collab_sync_and_presence`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
