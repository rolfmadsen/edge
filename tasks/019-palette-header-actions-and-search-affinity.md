---
type: Task Package
title: "Task 019: Palette UX: Header-Handlinger og Direkte Søge-Nærhed"
description: "Reorganisering af venstre palet i Begrebsmodel og Informationsmodel, så opret-knapper flyttes op i headerlinjen og søgefeltet støder direkte op til den filtrerede liste"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, ux, palette, search, layout, visual-hierarchy]
---

# Task 019: Palette UX: Header-Handlinger og Direkte Søge-Nærhed

**Status**: `ACTIVE`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Løse den visuelle og logiske afbrydelse i venstre palet, hvor `+ Nyt begreb` og `+ Ny` / `+ Fra begreb...` i dag ligger placeret imellem søgefeltet og elementlisten.
2. I Begrebsmodel (Graf):
   - Flytte `+ Nyt begreb` op i palettens øverste header ved siden af overskriften og tælleren (f.eks. som en kompakt primær knap eller et `+` ikon).
   - Placere `🔍 Søg begreber...` umiddelbart over listen uden mellemliggende knapper, så søgefeltet og den filtrerede liste udgør en samlet visuel enhed.
3. I Informationsmodel:
   - Flytte `+ Ny` og `+ Fra begreb...` op i palettens headersektion.
   - Placere `🔍 Søg klasser...` direkte over klasselisten.
4. Sikre ensartet visuelt hierarki og spacing på tværs af begge modeller.

## 📋 Acceptance Criteria
- [ ] I Begrebsmodellens palet er `+ Nyt begreb` integreret i headeren, og søgefeltet er placeret direkte over begrebslisten.
- [ ] I Informationsmodellens palet er `+ Ny` og `+ Fra begreb...` placeret over søgefeltet, så søgefeltet støder direkte op til klasselisten.
- [ ] Søgning filtrerer listerne uændret, men den kognitive belastning og layout-spring er elimineret.
- [ ] Både tastaturfokus og musenavigation fungerer gnidningsfrit.
- [ ] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE fjerne funktionalitet vedrørende oprettelse af begreber eller klasser (inklusive oprettelse fra eksisterende begreb).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`
