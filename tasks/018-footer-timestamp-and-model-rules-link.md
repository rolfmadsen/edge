---
type: Task Package
title: "Task 018: Footer Ergonomi: Læsbart Tidsstempel og Klikbart FDA Modelregler Link"
description: "Opgradering af applikationens statuslinje med menneskeligt læsbart tidsstempel for seneste gemning samt klikbart link til FDA Modelreglerne i browseren"
status: pending
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, footer, timestamp, ergonomics, external-link]
---

# Task 018: Footer Ergonomi: Læsbart Tidsstempel og Klikbart FDA Modelregler Link

**Status**: `PENDING`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Gøre "FDA Modelregler v2.1" i footeren til et klikbart link / knap, som åbner den officielle vejledning (`https://arkitektur.digst.dk/modelregler`) i brugerens standardbrowser.
2. Forbedre visningen af gemme-status i footeren, så der vises et menneskeligt læsbart tidsstempel for, hvornår modellen sidst blev gemt (f.eks. `💾 Sidst gemt kl. 09:52 • model.edge.json`).
3. Gøre det let at afkode filstatus hurtigt uden at fylde statuslinjen med lange, uoverskuelige filstier (fuld sti gøres tilgængelig som tooltip eller komprimeret sti).

## 📋 Acceptance Criteria
- [ ] Klik på `FDA Modelregler v2.1` i footeren åbner `https://arkitektur.digst.dk/modelregler` via systembrowser.
- [ ] `SaveStatus::Saved` udvides eller suppleres med et tidsstempel for gemningstidspunktet.
- [ ] Footer-teksten formateres som `💾 Sidst gemt kl. HH:MM:SS • <filnavn>` når en model er gemt.
- [ ] Ved ikke-gemte ændringer vises fortsat tydelig indikation (`⚠️ Ikke gemte ændringer` eller `⚠️ Nyt projekt`).
- [ ] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE fejle uhåndteret hvis åbning af systembrowser ikke lykkes (fejl skal logges eller ignoreres stumt).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`
