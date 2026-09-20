---
type: Task Package
title: "Task 026: E2EE Klientside Krypto & WebSocket Netværkskanal"
description: "Implementering af ChaCha20-Poly1305 kryptering/dekryptering i src/features/collab/crypto.rs, sessionsbillet (token) serialisering/parsing og tokio-tungstenite netværkskanal til Iced"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:55:00Z" }
tags: [collaboration, crypto, chacha20poly1305, e2ee, websocket, tokio-tungstenite]
---

# Task 026: E2EE Klientside Krypto & WebSocket Netværkskanal

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Implementere et klientside kryptografisk modul i `src/features/collab/crypto.rs` baseret på `chacha20poly1305` (AEAD):
   - Generering af kryptografisk sikre 256-bit symmetriske nøgler.
   - Generering af alfanumeriske `RoomId` identifikatorer (f.eks. `PEER-4821`).
   - Funktioner til symmetrisk kryptering og dekryptering af vilkårlige payloads (`&[u8]`).
2. Implementere en **Sessionsbillet (Session Token)**:
   - Serialisering og deserialisering af en kompakt, url-sikker sessionsstreng, som værten kan kopiere og gæsten kan indsætte.
   - Billetten indkapsler: `relay_url`, `room_id` og `key`.
   - Gæstens parsing skal automatisk validere formatet og afvise ugyldige eller korrupte tokens.
3. Etablere en Iced-kompatibel WebSocket-klientkanal i `src/features/collab/network.rs`:
   - Anvender `tokio-tungstenite` til asynkron WSS-forbindelse.
   - Forbindelses-livscyklus: `Connecting`, `Connected`, `Reconnecting`, `Disconnected`.
   - Kanal med tovejs-flow: En `UnboundedSender` for udgående krypterede frames og en Iced-kompatibel stream/kanal for indgående modtagne frames.
4. Gennemføre omfattende property-based tests med `proptest`, der beviser, at vilkårlig data kan krypteres og dekrypteres tabsløst, og at dekryptering med forkert nøgle eller manipuleret ciphertext medfører øjeblikkelig afvisning (AEAD auth-tag validering).

## 📋 Acceptance Criteria
- [x] `chacha20poly1305` og `tokio-tungstenite` tilføjes til `edge` dependencies.
- [x] `src/features/collab/crypto.rs` implementerer `CollabKey::generate()`, `encrypt(...)` og `decrypt(...)`.
- [x] `SessionTicket` struct kan serialiseres til/fra formatet `edge:v1:<base64-payload>` indeholdende relay URL, rum og nøgle.
- [x] Enhedstests beviser:
  - En vilkårlig byte-sekvens krypteres og dekrypteres fejlfrit med samme nøgle.
  - Dekryptering med en anden nøgle fejler med `CryptoError::AuthenticationFailed`.
  - Manipulerede ciphertexts fejler altid (integritetsbeskyttelse).
- [x] `src/features/collab/network.rs` etablerer WebSocket-forbindelse til den specificerede relay-URL og det tilhørende rum.
- [x] Netværkskanalen understøtter automatisk reconnection med exponential backoff ved kortvarige netværksudfald.
- [x] `cargo test` og `cargo clippy -- -D warnings` passerer 100%.

## 🚫 Must NOT
- Må IKKE sende ukrypteret data over netværket (alt indhold skal passere gennem `encrypt()` før afsendelse).
- Må IKKE sende nøglen til relay-serveren (hverken i HTTP-query, headers eller URL path).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som opgave 2 i E2EE Live Collaboration serien jf. ADR 008.

## 🧪 Verifikation
- `cargo test test_crypto`
- `cargo test test_session_ticket`
- `cargo test --workspace`
