---
type: Task Package
title: "Task 016: COSMIC-Inspireret Glas & Dybde Designsystem i theme.rs"
description: "Opgradering af Edges temasystem med COSMIC-inspireret æstetik: glasoverflader, bløde radier, dybe diffuse skygger og pille-formede kontroller"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T00:29:00Z" }
tags: [ui, theme, cosmic, design-system, glassmorphism, elevation, styling]
---

# Task 016: COSMIC-Inspireret Glas & Dybde Designsystem i theme.rs

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Opgradere [src/ui/theme.rs](file:///home/rolfmadsen/Github/edge/src/ui/theme.rs) med en poleret COSMIC-inspireret æstetik:
   - Semi-transparente "glassmorphic" overflader på paneler og kort med bløde kontraster.
   - Forøgede, organiske hjørneradier (12px - 16px) på overflader og 8px - 10px på knapper.
   - Subtile, diffuse dybdeskygger (elevation) der får flydende sidepaneler (Klasser, Begreber, Relationer) og dialoger til at svæve elegant over lærredet.
   - Pille-formede faneblade og forbedrede hover/active tilstande med blød farvet glød.
2. Bevare 100% krydsplatformsstøtte til Linux, macOS og Windows uden afhængighed af eksterne C/Wayland-specifikke biblioteker.
3. Bevare alle eksisterende modelregler, FDA-farver og test-invarianter.

## 📋 Acceptance Criteria
- [ ] `card_container_style` opgraderet med semi-transparent baggrund, 12px radier og blød dybdeskygge.
- [ ] `pill_container_style` og `segmented_tab_button` opdateret med ægte pille-form (16px radier) og hævet aktiv tilstand.
- [ ] `primary_button_style` og `secondary_button_style` forfinet med 8px radier og hover-glød / elevation.
- [ ] `modern_input_style` og `modal_card_style` forfinet med matchende COSMIC radier og dybde.
- [ ] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE ændre FDA farvekoder i begrebs- eller informationsmodellen.
- Må IKKE introducere platformsspecifikke crates eller bryde krydskompilering.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-20: Oprettet efter brugerønske om et mere poleret, COSMIC-lignende design i ren Iced.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`
