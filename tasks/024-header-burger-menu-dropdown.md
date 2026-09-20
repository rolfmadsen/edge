---
type: Task Package
title: "Task 024: Header Burger-menu & Modelomslag Onboarding"
description: "Placering af fil- og projekthandlinger under en klassisk burger-menu i øverste venstre hjørne samt automatisk visning af Modelomslag ved oprettelse af nyt projekt"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T11:30:00Z" }
tags: [ui, ergonomics, navigation, burger-menu, header, modal]
---

# Task 024: Header Burger-menu & Modelomslag Onboarding

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Rense top-baren for overskydende knapper ved at introducere en stilren burger-menu (`☰`) til venstre ved brandet ("Edge v0.1").
2. Samle alle fil- og modelhandlinger i en overskuelig dropdown-menu:
   - `+ Nyt projekt`
   - `📁 Åbn projekt...`
   - `💾 Gem projekt`
   - `💾 Gem som...`
   - `📋 Modelomslag & Metadata`
   - `📖 FDA Modelregler v2.1 ↗`
3. Sikre intuitiv dismissal: klik på `☰` toggler, klik uden for menuen lukker, og valg af et punkt lukker menuen og udfører handlingen.
4. Onboarding: Når et nyt projekt oprettes via `+ Nyt projekt`, vises Modelomslaget automatisk, så brugeren straks kan udfylde titel, domæneområde og ansvarlig organisation.

## 📋 Acceptance Criteria
- [ ] Burger-menu knap (`☰`) er placeret i venstre side af header-baren ved logoet.
- [ ] De 4 tidligere knapper (+ Nyt, Åbn, Gem, Gem som) og den separate "📋 Modelomslag"-knap er fjernet fra header-baren for et rent og fokuseret udtryk.
- [ ] Fane-vælgeren (Begrebsliste, Begrebsmodel, Informationsmodel) forbliver centreret i header-baren.
- [ ] Klik på `☰` åbner en flyout/dropdown-menu med tydelige ikoner og tekster for alle handlinger.
- [ ] Klik på et menupunkt udfører handlingen og lukker menuen.
- [ ] Klik uden for dropdown-menuen lukker menuen uden utilsigtede bivirkninger.
- [ ] Oprettelse af nyt projekt (`Message::NewProject`) åbner automatisk Modelomslag & Metadata modalen.
- [ ] `cargo test` og `cargo clippy -- -D warnings` passerer 100% uden fejl eller advarsler.

## 🚫 Must NOT
- Må IKKE fjerne eller bryde nogen eksisterende filgemme- eller åbne-funktioner.
- Må IKKE tillade overlappende modaler med burger-menuen åben.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet efter sparring om ergonomi og header-forenkling.

## 🧪 Verifikation
- `cargo test test_burger_menu`
- `cargo test`
- `cargo clippy -- -D warnings`
