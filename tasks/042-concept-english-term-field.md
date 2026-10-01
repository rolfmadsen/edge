---
type: Task Package
title: "Task 042: Tilføjelse af Felt for Engelsk Term på Begreber"
description: "Udvidelse af Begrebsmodellen med feltet Engelsk term for flersproget understøttelse i model, tabelvisning og egenskabsinspektør"
status: cancelled
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [concepts, english-term, multilingual, fda, concept-table, inspector]
---

# Task 042: Tilføjelse af Felt for Engelsk Term på Begreber

**Status**: `CANCELLED`  
**Intent**: 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-21`  
**Afsluttet**: `2026-10-01 (Annulleret)`  
**Scope**: `src/features/concepts/mod.rs`, `src/ui/concept_table.rs`, `src/ui/concept_editor.rs`, `src/ui/inspector_panel.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål & Årsag til Annullering
Opgaven er **annulleret** efter arkitektonisk afklaring og genbesøg af den officielle FDA Begrebs- og Informationsmodelleringsvejledning ([docs/fda_modelleringsvejledning.md:1086-1115](file:///home/rolfmadsen/Github/edge/docs/fda_modelleringsvejledning.md#L1086-L1115)):
1. **FDA Standard Felter (Bilag D & E)**: FDA definerer udelukkende termfelterne `Foretrukken dansk term`, `Accepteret dansk term` og `Frarådet dansk term`. Der specificeres intet felt for engelsk term i FDA begrebslisteskabelonen.
2. **Flersprogethed**: Hvis flersprogethed i fremtiden ønskes understøttet, bør det implementeres via fuld internationalisering (SKOS sprogkoder `@da`, `@en` for samtlige labels og definitioner) fremfor et enkelt ad-hoc felt.
3. **Fokus**: Udviklingen fokuseres i stedet på officielle FDA-krav, herunder Task 063 (Kontrollerede Udfaldsrum: UI & Canvas Integration for Enumerationer jf. FDA Kapitel 5.2/5.5).

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke.
- 2026-10-01: Annulleret (CANCELLED) for at opretholde 1:1 overensstemmelse med den officielle FDA Begrebslisteskabelon.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_042`
- `cargo clippy --all-targets`
- `cargo fmt --check`
