---
type: Task Package
title: "Task 061: Naming Convention Linter (FDA §19)"
description: "Aktiv linter der håndhæver FDA Regel 19 navnekonventioner: UpperCamelCase på klasser og lowerCamelCase på attributter og associationsender"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T23:09:00Z" }
tags: [fda, linter, naming-conventions, §19, UpperCamelCase, lowerCamelCase]
---

# Task 061: Naming Convention Linter (FDA §19)

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`
**Scope**: `src/features/information_model/linter.rs`, `src/features/information_model/mod.rs`, `src/ui/information_model_view.rs`, `tests/acceptance.rs`

## 🎯 Formål
Håndhæve FDA Modelregel 19 (*Brug standardiserede konventioner for angivelse af navne*) med aktiv realtidslinting i informationsmodellen:

1. **UpperCamelCase for klasser og strukturerede datatyper**:
   - Klassenavne skal starte med stort bogstav og bruge CamelCase (f.eks. `EthjuletCykel`, ikke `ethjulet_cykel`).
   - Enumerationsnavne skal følge UpperCamelCase.
   - Strukturerede datatyper skal følge UpperCamelCase.

2. **lowerCamelCase for attributter og associationsender**:
   - Attributnavne skal starte med lille bogstav og bruge CamelCase (f.eks. `stelnummer`, `maxPassagerer`).
   - Associationsende-navne (edge labels i informationsmodellen) skal bruge lowerCamelCase.

3. **Naturligt sprog i begrebsmodellen**:
   - Begrebsnavne skal bruge naturligt sprog med lille begyndelsesbogstav (allerede korrekt).
   - Associationsnavne i begrebsmodellen skal bruge naturligt sprog (allerede korrekt).

4. **UI Integration**:
   - Inline advarsler i informationsmodel-inspektøren ved navneovertrædelser.
   - Tooltip med FDA-reference og forklaring.
   - Ikke-blokerende: advarsler forhindrer ikke gemning.

## 📋 Acceptance Criteria
- [ ] **AC1 - UpperCamelCase Linter for Klasser**: En `NamingLinter::check_class_name(name) -> Option<NamingIssue>` returnerer advarsel hvis klassenavnet ikke er UpperCamelCase.
- [ ] **AC2 - lowerCamelCase Linter for Attributter**: `NamingLinter::check_attribute_name(name) -> Option<NamingIssue>` returnerer advarsel hvis attributnavnet ikke er lowerCamelCase.
- [ ] **AC3 - lowerCamelCase Linter for Associations-Labels**: Edge labels i informationsmodellen valideres mod lowerCamelCase.
- [ ] **AC4 - UI Advarsler i Inspektøren**: Visuel feedback (ikon + tooltip) ved klassenavne og attributnavne der bryder konventionen.
- [ ] **AC5 - Enhedstest**: Automatiserede tests for UpperCamelCase- og lowerCamelCase-detektering med positive og negative eksempler.

## 🚫 Must NOT
- Må IKKE blokere gemning eller eksport ved navnefejl (kun vejledende advarsler).
- Må IKKE påvirke begrebsmodellens navnekonventioner (naturligt sprog er korrekt dér).
- Må IKKE ændre eksisterende navne automatisk.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke efter FDA compliance review af Kant v0.4.0.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
