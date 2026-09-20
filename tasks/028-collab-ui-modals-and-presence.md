---
type: Task Package
title: "Task 028: Samarbejds-UI, Start/Join Modaler & Live Deltagerstatus"
description: "Integration af Samarbejde i desktop-headeren, Cosmic Glass modaler for Start Session (Host med servervalg) og Join Session (Guest med 1-klik token paste) samt live tilstedeværelses- og forbindelsesindikator"
status: in_progress
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:55:00Z" }
tags: [collaboration, ui, iced, cosmic-glass, modals, presets, presence]
---

# Task 028: Samarbejds-UI, Start/Join Modaler & Live Deltagerstatus

**Status**: `IN_PROGRESS`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Tilføje en dedikeret `🌐 Samarbejde` menu eller knap i Edges desktop header-bar (ved siden af `Filer` og `Hjælp`).
2. Implementere **"Start Live Session" (Vært)** modal:
   - Valg af Relay Server med faste presets:
     1. `Koyeb Cloud (Standard - Frankfurt)`: `wss://edge-relay.koyeb.app/ws`
     2. `Intern Organisation`: Konfigurerbar URL (kan gemmes som standard)
     3. `Lokal Docker`: `ws://localhost:8080/ws`
     4. `Brugerdefineret URL...`
   - Generering og fremvisning af Sessionskode (med 1-klik "Kopiér sessionskode" knap).
   - Valg om at huske servervalg til fremtidige sessioner.
3. Implementere **"Deltag i Live Session" (Gæst)** modal:
   - Ét enkelt inputfelt: "Indsæt sessionskode eller link".
   - Automatisk validering ved paste (grøn afkrydsning ved gyldig token).
   - Knap til "Forbind".
4. Tilføje en **Live Deltagerindikator**:
   - Viser aktuel status i header-baren / statuslinjen:
     - 🟢 `Live: Vært (N deltagere)` / 🟢 `Live: Gæst (Forbundet til Vært)`
     - 🟡 `Genforbinder...`
     - ⚪ `Offline`
   - Knap til at forlade sessionen eller afslutte sessionen med advarsel til gæster.

## 📋 Acceptance Criteria
- [ ] Header-baren indeholder et `🌐 Samarbejde` menupunkt med valgmulighederne: `Start session (Vært)...`, `Deltag i session (Gæst)...` samt `Afbryd session` (når aktiv).
- [ ] Værtsdialogen lader brugeren vælge mellem server-presets eller indtaste en custom URL.
- [ ] Værtsdialogen genererer en gyldig sessionskode og tilbyder en "Kopiér kode" knap med visuel "Kopieret! ✓" feedback.
- [ ] Gæstedialogen validerer den indsatte kode øjeblikkeligt og viser klar fejlmeddelelse, hvis formatet er ugyldigt.
- [ ] Statusbaren eller headeren viser en diskret pille med grøn indikator og antal deltagere under en aktiv session.
- [ ] Hvis værten afslutter sessionen, præsenteres gæsten for en dialog med mulighed for at "Gemme som kopi..." før lærredet ryddes eller sessionen lukkes.
- [ ] Modalerne følger Cosmic Glass designsystemet (ensartet typografi, transparens, mørkt tema, focus rings og tastaturnavigation via Esc/Enter).
- [ ] `cargo test` og `cargo clippy -- -D warnings` passerer 100%.

## 🚫 Must NOT
- Må IKKE tillade opstart af simultane sessioner i samme klientvindue.
- Må IKKE efterlade dialoger åbne ved vellykket opkobling.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som opgave 4 i E2EE Live Collaboration serien jf. ADR 008.

## 🧪 Verifikation
- `cargo test test_collab_ui`
- `cargo test`
- `cargo clippy -- -D warnings`
