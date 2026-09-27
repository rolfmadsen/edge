---
type: Task Package
title: "Task 045: Semantisk 3-Vejs Model Merge Motor (merge_models)"
description: "Model-bevidst 3-vejs fusionsalgoritme for FDA begreber, klasser, relationer og diagrammer med automatisk feltfletning og struktureret konfliktopsporing"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T16:16:00Z" }
tags: [merge, semantic-merge, 3-way, git, conflict-resolution, model-engine]
---

# Task 045: Semantisk 3-Vejs Model Merge Motor (merge_models)

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-27`  
**Scope**: `src/features/model/merge.rs`, `src/features/model/mod.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Implementere en domæne- og modelspecifik 3-vejs merge-motor (`merge_models`), der fletter lokale ændringer (`ours`) med serverens ændringer (`theirs`) med udgangspunkt i en fælles forfader-commit (`base`):

1. **Automatisk Entitets- og Feltfusion**:
   - Uafhængige entiteter (fx Bruger A redigerede Begreb 1, Bruger B redigerede Begreb 2) sammensmeltes automatisk uden advarsler.
   - Uafhængige felter på samme entitet (fx Bruger A ændrede definition, Bruger B ændrede kilde på samme begreb) kombineres automatisk.
2. **Sletning vs. Redigering**:
   - Hvis Bruger A har slettet et element, mens Bruger B har redigeret det, bevares elementet med en klar advarselsmeddelelse, så der aldrig opstår brudte relationer eller panics.
3. **Struktureret Konfliktopsporing**:
   - Hvis begge parter har ændret det samme tekstfelt til forskellige værdier, registreres en præcis `ModelConflict` (entitet, feltnavn, base-værdi, our-værdi, their-værdi).
   - Konflikten forbliver i hukommelsen til præsentation i Kants visuelle dialog. Der skrives **aldrig** rå Git-konfliktmarkører (`<<<<<<< HEAD`) til filerne.
4. **Fail-Closed Integritet**:
   - Den resulterende flettede model valideres jf. FDA Modelreglerne, og efterlader aldrig modellen i en ugyldig tilstand.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Automatisk fusion af uafhængige entiteter**: Nye eller ændrede begreber og klasser fra henholdsvis `ours` og `theirs` inkluderes begge i det flettede resultat.
- [ ] **AC2 - Granulær feltfusion på samme entitet**: Ændringer på forskellige felter i samme begreb eller klasse sammensmeltes uden konflikt (fx `ours` ændrer `definition`, `theirs` ændrer `source`).
- [ ] **AC3 - Detektion af modstridende feltændringer**: Samtidige ændringer af samme felt til forskellige værdier detekteres og returneres som en struktureret `ModelConflict` (med base, ours og theirs værdier).
- [ ] **AC4 - Sikker håndtering af Sletning vs. Redigering**: Slettede entiteter, der er blevet redigeret af modparten, håndteres uden panics eller korrupte relationer.
- [ ] **AC5 - 3-Vejs Fletning af Informationsklasser og Attributter**: Attributter flettes på ID-niveau, så parallelle tilføjelser af nye attributter til samme klasse begge bevares.
- [ ] **AC6 - Idempotens & Fuld Integritet**: Fletning af identiske modeller (`merge(base, ours, ours)`) resulterer i 0 konflikter og identisk model.
- [ ] **AC7 - Verifikation via Accepttest**: `test_task_045_semantic_three_way_model_merge` i `tests/acceptance.rs` beviser feltfusion, sletningshåndtering, konfliktopsamling og validering.

---

## 🚫 Must NOT
- Må IKKE skrive rå Git-konfliktmarkører (`<<<<<<< HEAD`) til modellen.
- Må IKKE panikke ved krydsende relationer eller slettede noder.
- Må IKKE producere en model, der fejler FDA-validering.

---

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke Task 045 jf. tracer-bullet planen.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_045`
- `cargo clippy --all-targets`
- `cargo fmt --check`
