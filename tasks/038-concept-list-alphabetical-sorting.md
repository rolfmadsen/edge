---
type: Task Package
title: "Task 038: Alfabetisk Sortering af Begrebslisten og Paletten"
description: "Automatisk alfabetisk sortering af begreber i Begrebslisten (Fane 1), model-repository og Begrebsmodellens palet (Fane 2)"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T19:20:00Z" }
tags: [concepts, sorting, list, palette, ergonomics]
---

# Task 038: Alfabetisk Sortering af Begrebslisten og Paletten

**Status**: `ACTIVE`  
**Intent**: 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/features/model/mod.rs`, `src/ui/app.rs`, `src/ui/concept_model_view.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Sikre at begreber automatisk præsenteres og vedligeholdes i alfabetisk rækkefølge baseret på `preferred_term`:
1. **Model & Projekt**:
   - `Project::add_concept` og `Project::sort_concepts_alphabetically`: Sikre at samlingen af begreber i projektet holdes eller sorteres alfabetisk (case-insensitive / dansk collation).
2. **Begrebslisten (Fane 1)**:
   - `filtered_concepts(&self)` i `App` returnerer altid begreberne alfabetisk sorteret efter `preferred_term.to_lowercase()`, både med og uden aktivt søgefilter.
   - Når et nyt begreb oprettes, indsættes det automatisk på sin rette alfabetiske placering i tabellen frem for i bunden.
3. **Begrebsmodellens Venstre Palet (Fane 2)**:
   - Listen over begreber i venstre palet i `concept_model_view` vises stringent alfabetiseret.
4. **Drop-downs & Vælgere**:
   - Vælgerlister (f.eks. ved oprettelse af klasse fra begreb eller knytning til attribut) fremstår alfabetisk ordnede.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Automatisk sortering i `Project`**: `Project::sort_concepts_alphabetically()` sorterer alle begreber alfabetisk efter `preferred_term` case-insensitivt. `add_concept` bevarer eller genskaber alfabetisk sortering.
- [ ] **AC2 - Alfabetisk Begrebsliste (Fane 1)**: `app.filtered_concepts()` returnerer referencer sorteret alfabetisk. Ved oprettelse af begreb med term f.eks. "Aalborg" eller "Båd" placeres det korrekt i forhold til eksisterende begreber.
- [ ] **AC3 - Alfabetisk Palet i Begrebsmodel (Fane 2)**: Begreberne i venstre repository-browser i `concept_model_view` er sorteret alfabetisk fra top til bund.
- [ ] **AC4 - Bevarelse af Modelintegritet & Collab**: Sorteringen påvirker ikke UUID-identifikatorer, diagramknuder, relationer eller collab-synkronisering.

---

## 🚫 Must NOT
- Må IKKE ændre på begrebernes unikke ID (`Uuid`) eller bryde referencer i grafknuder (`concept_graph`) eller informationsmodel (`class.concept_ids()`).
- Må IKKE fejle på danske specialtegn (æ, ø, å).

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_038`
- `cargo clippy --all-targets`
- `cargo fmt --check`
