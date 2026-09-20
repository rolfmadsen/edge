---
type: Task Package
title: "Task 019: Palette UX: Header-Handlinger og Direkte Søge-Nærhed"
description: "Reorganisering af venstre palet i Begrebsmodel og Informationsmodel, så opret-knapper flyttes op i headerlinjen og søgefeltet støder direkte op til den filtrerede liste"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, ux, palette, search, layout, visual-hierarchy]
---

# Task 019: Palette UX: Header-Handlinger og Direkte Søge-Nærhed

**Status**: `DONE`
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
- [x] I Begrebsmodellens palet er `+ Nyt begreb` integreret i headeren, og søgefeltet er placeret direkte over begrebslisten.
- [x] I Informationsmodellens palet er `+ Ny` og `+ Fra begreb...` placeret over søgefeltet, så søgefeltet støder direkte op til klasselisten.
- [x] Søgning filtrerer listerne uændret, men den kognitive belastning og layout-spring er elimineret.
- [x] Både tastaturfokus og musenavigation fungerer gnidningsfrit.
- [x] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE fjerne funktionalitet vedrørende oprettelse af begreber eller klasser (inklusive oprettelse fra eksisterende begreb).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.
- 2026-09-20: Gennemført og verificeret med TDD gauntlet. Header-handlinger konsolideret og direkte søge-nærhed etableret på tværs af Begrebs- og Informationsmodel.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`

