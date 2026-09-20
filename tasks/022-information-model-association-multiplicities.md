---
type: Task Package
title: "Task 022: Informationsmodel: Multipliciteter på UML Associationer"
description: "Implementering af kilde- og mål-multipliciteter på relationer i informationsmodellen jf. FDA Modelreglerne v2.1 §6"
status: pending
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [domain, ui, information-model, uml, associations, multiplicity, fda-modelregler]
---

# Task 022: Informationsmodel: Multipliciteter på UML Associationer

**Status**: `PENDING`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Implementere kilde- og mål-multipliciteter på UML-relationer (associationer) i overensstemmelse med FDA Modelreglerne v2.1 §6.
2. Udvide `ClassDiagramEdge` i `src/features/information_model/`:
   - `source_multiplicity: Option<Multiplicity>`
   - `target_multiplicity: Option<Multiplicity>`
3. Udvide dialogen for oprettelse/redigering af relationer samt Egenskaber-panelet for valgte relationer med vælgere for multiplicitet (med standard FDA-presets: `1`, `0..1`, `0..*`, `1..*`).
4. Rendre multipliciteterne visuelt på lærredet i umiddelbar nærhed af kantens start- og slutpunkter ved portene.

## 📋 Acceptance Criteria
- [ ] `ClassDiagramEdge` har felter til `source_multiplicity` og `target_multiplicity` med serde-kompatibilitet (bagudkompatibel med default `None`).
- [ ] Oprettelsesdialogen for relationer giver mulighed for at angive multiplicitet for både kilde og mål.
- [ ] Når en kant er valgt på lærredet, viser højre panel (Egenskaber) kontroller til at ændre multipliciteterne.
- [ ] Diagram canvas renderer multiplicitetsteksterne (f.eks. `1` og `0..*`) læsbart ved kilde- og målportene.
- [ ] 100% test pass rate på unit-, model- og diagramtests samt clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE bryde eksisterende kant-routing (`EdgeRouter`) eller port-hysterese.
- Må IKKE gøre multiplicitet obligatorisk for generaliseringer (arv har ikke multiplicitet).
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning.

## 🧪 Verifikation
- `cargo test test_class_diagram_edge`
- `cargo clippy -- -D warnings`
