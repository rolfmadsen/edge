---
type: Task Package
title: "Task 024: Desktop Menulinje & Sidebar Toggle (Cosmic Files Mønster)"
description: "Implementering af klassisk integreret desktop-menulinje (Filer & Hjælp), [◨] sidebar-toggle til at klappe venstre palet sammen for lærredsfokus, samt Modelomslag onboarding ved Nyt Projekt inspireret af Cosmic Files"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:38:00Z" }
tags: [ui, ergonomics, navigation, desktop-menu, sidebar-toggle, cosmic-style, modal]
---

# Task 024: Desktop Menulinje & Sidebar Toggle (Cosmic Files Mønster)

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Erstatte de overfyldte header-knapper (+ Nyt, Åbn, Gem, Gem som, Modelomslag) med en elegant, integreret desktop-menulinje (`Filer` og `Hjælp`) i overensstemmelse med System76's Iced-baserede Cosmic Files design.
2. Implementere en `[◨]` Sidebar Toggle knap i header-baren (og `Ctrl+B` genvej), der lader brugeren klappe venstre repository-palet sammen for at opnå maksimalt fokus og fuld bredde til model-canvaset.
3. Onboarding: Når et nyt projekt oprettes via `Filer -> ➕ Nyt projekt`, vises Modelomslaget automatisk, så brugeren straks kan udfylde titel, domæneområde og ansvarlig organisation.
4. Sikre desktop dismissal: Klik uden for en åben menu eller tryk på Escape lukker menuen.

## 📋 Acceptance Criteria
- [ ] Header-baren har en `[◨]` panel-toggle knap yderst til venstre.
- [ ] Integrerede desktop menupunkter `Filer` og `Hjælp` er placeret ved siden af logoet.
- [ ] De 4 tidligere fritstående knapper (+ Nyt, Åbn, Gem, Gem som) og den separate "📋 Modelomslag"-knap er fjernet fra header-baren.
- [ ] Fane-vælgeren (Begrebsliste, Begrebsmodel, Informationsmodel) forbliver centreret i header-baren.
- [ ] Klik på `Filer` åbner dropdown med: `➕ Nyt projekt`, `📁 Åbn projekt...`, `💾 Gem`, `💾 Gem som...` samt `📋 Modelomslag & Metadata...`.
- [ ] Klik på `Hjælp` åbner dropdown med: `📖 FDA Modelregler v2.1 ↗` samt `ℹ️ Om Edge FDA Modeller...`.
- [ ] Klik på `[◨]` (eller `Ctrl+B`) klapper venstre-paletten sammen/ud i både Begrebsmodel og Informationsmodel, så diagrammet får fuld bredde.
- [ ] Oprettelse af nyt projekt (`Message::NewProject`) åbner automatisk Modelomslag & Metadata modalen.
- [ ] `cargo test` og `cargo clippy -- -D warnings` passerer 100% uden fejl eller advarsler.

## 🚫 Must NOT
- Må IKKE fjerne eller bryde nogen eksisterende filgemme- eller åbne-funktioner.
- Må IKKE tillade overlappende modaler med en åben menu.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet og raffineret fra mobil burger-menu til autentisk Cosmic Files desktop-menulinje + sidebar toggle.

## 🧪 Verifikation
- `cargo test test_task024`
- `cargo test`
- `cargo clippy -- -D warnings`
