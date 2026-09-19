---
type: Task Package
title: "Task 014: Rettede Associationer (Halv pil), Retningsvending & Filter-forbedringer"
description: "Halv pil på associationer som default med mulighed for toggle, retningsvending af relationer i panelet, filtrering af eksisterende klassenavne i dropdown og fjernelse af forældet Synk-knap"
status: active
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T21:55:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, relations, directed-association, half-arrow, reversal, filtering]
---

# Task 014: Rettede Associationer (Halv pil), Retningsvending & Filter-forbedringer

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
1. Implementere en halv pil (diagonal streg/barb) helt ude ved målnodens kant på associationer som default for at indikere navigabilitet/retning, med mulighed for at slå den fra (`directed: bool`).
2. Tilføje en "⇄ Vend retning" knap i relations-panelet (både for begrebsmodellen og informationsmodellen), som vender relationen og dens porte om.
3. Frasortere begreber i "+ Fra begreb..." dropdownen under Informationsmodellens Klasser-palet, hvis der allerede eksisterer en klasse med samme navn.
4. Fjerne den forældede og overflødige "🔄 Synk" knap i Begreber-panelet.

## 📋 Acceptance Criteria
- [x] `DiagramEdge` og `ClassDiagramEdge` udvides med `directed: Option<bool>` (default `true` for Association; serialiseret med serde og fuld bagudkompatibilitet).
- [x] `EdgeRouter` genererer en halv pil (en diagonal streg/barb) helt ude ved målnodens kant for rettede associationer.
- [x] `DiagramCanvas` renderer den halve pil ved målnodens kant med korrekt vinkel og farve.
- [x] Relations-panelet indeholder en checkbox eller toggle-knap til at slå retningspilen til/fra på associationer.
- [x] Relations-panelet indeholder en "⇄ Vend retning" knap, der bytter `from` og `to` samt spejlvender `source_port` og `target_port`, og bevarer markeringen.
- [x] "+ Fra begreb..." dropdownen i Informationsmodellens venstre palet udelader begreber, der matcher et eksisterende klassenavn (case-insensitivt).
- [x] Den forældede "🔄 Synk" knap er fjernet fra Begreber-paletten i `concept_model_view.rs`.
- [x] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE ændre visning af Generalisering (hvid trekant) eller Komposition (sort diamant).
- Må IKKE ødelægge eksisterende `model.edge.json` filer ved indlæsning.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Oprettet efter brugerønske om halv pil på associationer, retningsvending, dropdown-filtrering og oprydning i Synk-knappen.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 014-directed-association-half-arrow-and-reversal`
- `xgauntlet verify --task 014-directed-association-half-arrow-and-reversal`
