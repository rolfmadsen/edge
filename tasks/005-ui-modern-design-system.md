---
type: Task Package
title: "Task 005: Moderne Desktop Designsystem & UI-Løft i Iced 0.14"
description: "Implementering af komplet design-tokensystem, segmented tab-navigation, stack-baserede modale overlays og FDA-semantiske komponenter"
status: open
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T12:28:00Z" }
tags: [task-lifecycle, intent, ui, design-system, iced, theming, modal-stack, fda-model]
---

# Task 005: Moderne Desktop Designsystem & UI-Løft i Iced 0.14

**Status**: `SPEC`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Løfte det visuelle og ergonomiske kvalitetsniveau i Edge fra en rå prototype til en elegant, moderne desktop-applikation (Option 1) baseret på ren Iced 0.14. Dette opnås ved at indføre et sammenhængende design-token- og stilsystem i `theme.rs`, en integreret Segmented HeaderBar, overlejrede modale dialoger via `stack!`, samt kort- og badge-baseret layout for begrebslisten, editoren og graf-canvaset jf. FDA Modelreglerne.

## 📋 Acceptance Criteria
- [ ] `src/ui/theme.rs` udvides med komplet design-tokensystem (Surfaces, Neutrals, Badges, Borders, Elevation/Shadows) og typesikre Iced 0.14 widget styles (`card_container`, `pill_container`, `segmented_button_active`, `segmented_button_inactive`, `button_primary`, `button_secondary`, `button_danger`, `button_subtle`, `modern_text_input`, `modal_backdrop`, `modal_card`).
- [ ] `src/ui/app.rs` moderniseres med en sammenhængende Desktop HeaderBar (Brand badge, Segmented Pill Tab Navigation, og hurtig-handlinger med autosave-indikator).
- [ ] Overlejrede modale dialoger via `iced::widget::stack!` for fildialoger ("Åbn / Gem som...") og relation-oprettelse ("+ Opret Relation"), med semi-transparent dæmpet baggrund (`backdrop`), så applikationen ikke layout-skifter.
- [ ] `src/ui/concept_table.rs` og `src/ui/concept_editor.rs` opgraderes til kort-baseret visuelt hierarki med FDA Sand/Blå status-badges, tydelig typografi og polerede formular-sektioner.
- [ ] `src/ui/graph_canvas.rs` og graf-inspektøren i `app.rs` fremstår med forbedret dybde (subtile kanter, struktureret properties sheet).
- [ ] 100% test pass rate på alle tests (`cargo test --tests`) og clippy med 0 advarsler (`cargo clippy -- -D warnings`).

## 🚫 Must NOT
- Må IKKE introducere eksterne tunge biblioteker eller bryde ADR 001 (forbliv på ren Iced 0.14).
- Må IKKE bryde eksisterende public contracts på `App`, `ModelProject`, `Concept`, eller tastaturgenveje.
- Må IKKE foretage remote publication handlinger (`git push`).
- Må IKKE foretage destruktive resets (`git reset --hard` eller `git clean -f`).

## 📝 Revisions
- 2026-09-19: Task oprettet for Moderne Desktop Designsystem & UI-Løft.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
