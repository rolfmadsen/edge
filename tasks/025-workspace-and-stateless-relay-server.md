---
type: Task Package
title: "Task 025: Cargo Workspace & Stateless In-Memory Relay Server (edge-relay)"
description: "Etablering af Cargo Workspace, udvikling af den ultralette, blinde Axum WebSocket relay i crates/edge-relay med rum-routing, last-snapshot buffer, Dockerfile, docker-compose.yml og Koyeb PaaS specifikation"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:55:00Z" }
tags: [collaboration, relay, axum, websocket, workspace, docker, koyeb]
---

# Task 025: Cargo Workspace & Stateless In-Memory Relay Server (edge-relay)

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Konvertere `edge` roden til et rent Cargo Workspace med to medlemmer: `.` (Iced desktop klienten) og `crates/edge-relay` (den uafhængige Axum WebSocket relay).
2. Implementere `crates/edge-relay` som en ultralet, in-memory, blind byte-router for WebSocket-forbindelser:
   - Rum-routing baseret på URL-parameter: `GET /ws?room=<ROOM_ID>`.
   - In-memory `Room`-tilstand med `broadcast::Sender<bytes::Bytes>`, `last_snapshot: Option<bytes::Bytes>` og deltager-tæller.
   - Health check endpoint: `GET /health` -> `200 OK`.
3. Sikre containerisering og udrulningsparathed:
   - `crates/edge-relay/Dockerfile` (multi-stage Alpine/scratch minimal container < 15MB).
   - `docker-compose.yml` til nem lokal kørsel eller intern organisation udrulning bag Nginx/Traefik.
   - `koyeb.yaml` og README med 1-klik "Deploy to Koyeb" knap.
4. Etablere automatiserede integrationstests, der beviser, at to WebSocket-klienter i samme rum kan udveksle binære frames uden forsinkelse, mens klienter i andre rum er isolerede.

## 📋 Acceptance Criteria
- [ ] Rodens `Cargo.toml` deklarerer `[workspace]` med members `.` og `crates/edge-relay`.
- [ ] `crates/edge-relay` har eget minimalt `Cargo.toml` med afhængigheder: `axum`, `tokio`, `tower-http`, `tracing`, `tracing-subscriber`, `bytes`, `futures-util`.
- [ ] Relayen kompilerer uafhængigt uden afhængigheder af Iced eller domænemodeller.
- [ ] WebSocket endpoint `/ws?room=<id>` opretter eller tilslutter klienten til et in-memory rum.
- [ ] Første klient i rummet (typisk værten) kan uploade et `last_snapshot` af krypterede bytes; nye klienter, der kobler på rummet, modtager automatisk dette `last_snapshot` som første binære frame.
- [ ] Binære frames modtaget fra én klient broadcastes øjeblikkeligt til alle andre klienter i samme rum via Tokio broadcast-kanal.
- [ ] Når sidste klient frakobler et rum, frigives rummet fra RAM efter en kort timeout (60 sekunder).
- [ ] `GET /health` returnerer `200 OK` med tekst `"OK"`.
- [ ] Miljøvariable understøttes: `PORT` (default: 8080), `HOST` (default: "0.0.0.0"), `RUST_LOG` (default: "info").
- [ ] `docker-compose.yml` og `koyeb.yaml` er oprettet og syntaktisk gyldige.
- [ ] Fuld integrationstestsuite i `crates/edge-relay/tests/` verificerer handshake, isolation mellem rum og snapshot-levering til nytilkomne deltagere.
- [ ] `cargo test --workspace` og `cargo clippy --workspace -- -D warnings` passerer 100%.

## 🚫 Must NOT
- Må IKKE gemme data på disk eller introducere ekstern database i relayen (100% in-memory / zero persistence).
- Må IKKE importere Iced eller Edge-domænemodeller i relay-kassen (relayen skal forblive blind over for payload-indhold).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som første opgave i E2EE Live Collaboration serien jf. ADR 008.

## 🧪 Verifikation
- `cargo test -p edge-relay`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
