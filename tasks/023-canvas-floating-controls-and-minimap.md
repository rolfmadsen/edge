---
type: Task Package
title: "Task 023: Canvas Ergonomi: Flydende Zoom/Pan Kontroller og Miniaturekort (Minimap)"
description: "Implementering af et svævende kontrolpanel med interaktivt miniaturekort (minimap) og hurtige zoom/pan kontroller inspireret af xArchi"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, canvas, ergonomics, minimap, zoom, pan, xarchi-inspiration]
---

# Task 023: Canvas Ergonomi: Flydende Zoom/Pan Kontroller og Miniaturekort (Minimap)

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Implementere et moderne, svævende ergonomi-kontrolpanel på canvaset inspireret af xArchi-mønstret (placeret i nederste højre hjørne over diagrammet):
   - **Miniaturekort (Minimap)**: En kompakt visning af hele diagrammet, der viser noderne som små rektangler og et rektangel for den aktuelle viewport-ramme.
   - **Zoom Kontroller**:
     - `+` (Zoom ind med fast faktor, f.eks. +15%).
     - `–` (Zoom ud med fast faktor, f.eks. -15%).
     - `⊡` (Fit to view / Nulstil zoom til 100% og centrer noderne).
     - Tekstuel visning af det aktuelle zoomniveau (f.eks. `100%`, `75%`, `120%`).
2. Sikre at klik og træk i minimappet flytter viewportens panorering direkte.
3. Virker identisk og reaktivt på tværs af både Begrebsmodel (Graf) og Informationsmodel.

## 📋 Acceptance Criteria
- [ ] Svævende kontrolpanel vises nederst til højre på diagram-canvaset med semi-transparent COSMIC glas-styling.
- [ ] Minimap renderer diagrammets noder og den aktuelle viewport-ramme skaleret ned i realtid.
- [ ] Knapperne `+`, `–` og `⊡` (reset) justerer viewportens zoom og panorering forudsigeligt.
- [ ] Det aktuelle zoomniveau vises i procent.
- [ ] Interaktion med minimap-kontrollerne blokerer ikke for normal knude- eller kant-interaktion på selve lærredet.
- [ ] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE gøre canvas renderingen langsom eller introducere mærkbare framedrops.
- Må IKKE forhindre panorering med musetræk eller mellemrumstast.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo test test_viewport`
- `cargo clippy -- -D warnings`
- Manuel afprøvning af minimap og zoom/pan kontroller i begge modelleringstrin.
