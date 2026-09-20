---
type: Task Package
title: "Task 021: Informationsmodel: Attribut-til-Begreb Lineage Vælger"
description: "Etablering af visuel kobling og lineage i UI fra Informationsmodellens klasseattributter til forretningsbegreber jf. ADR 006"
status: pending
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, information-model, attributes, lineage, traceability, concepts, adr-006]
---

# Task 021: Informationsmodel: Attribut-til-Begreb Lineage Vælger

**Status**: `PENDING`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Operationalisere sporbarheden (lineage) fra tekniske attributter i informationsmodellen til modellens forretningsbegreber (som defineret i `Attribute.concept_ids` jf. ADR 006).
2. I Informationsmodellens Egenskaber-panel (under redigering af en klasses attributter):
   - Tilføje en valgfri vælger (`pick_list` eller dropdown): "Relateret begreb".
   - Vælgeren lister samtlige begreber fra Begrebslisten (med mulighed for "Intet begreb / Teknisk felt").
3. Give en diskret visuel indikation (badge / ikon) i attributlisten på inspectoren (og eventuelt på UML-kassen), når en attribut har en direkte reference til et forretningsbegreb.
4. Sikre at sletning eller omdøbning af begreber håndteres robust uden at ødelægge informationsmodellen.

## 📋 Acceptance Criteria
- [ ] Attribut-editoren i Informationsmodellens inspector indeholder en dropdown til at vælge tilknyttet begreb.
- [ ] Valg af begreb persisteres i `Attribute.concept_ids` og gemmes i projektfilen.
- [ ] Hvis et begreb vælges, vises begrebets navn eller et lineage-ikon ud for attributten i inspectoren.
- [ ] Enhedstests bekræfter at `Attribute` bevarer `concept_ids` gennem serialisering og deserialisering.
- [ ] 100% test pass rate på `cargo test` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE tvinge alle attributter til at have et begreb (flere attributter er rent tekniske såsom ID'er, tidsstempler eller tekniske flag).
- Må IKKE slette attributter hvis det relaterede begreb slettes.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo test test_attribute_lineage`
- `cargo clippy -- -D warnings`
