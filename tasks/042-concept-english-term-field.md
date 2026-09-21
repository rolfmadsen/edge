---
type: Task Package
title: "Task 042: Tilføjelse af Felt for Engelsk Term på Begreber"
description: "Udvidelse af Begrebsmodellen med feltet Engelsk term for flersproget understøttelse i model, tabelvisning og egenskabsinspektør"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [concepts, english-term, multilingual, fda, concept-table, inspector]
---

# Task 042: Tilføjelse af Felt for Engelsk Term på Begreber

**Status**: `ACTIVE`  
**Intent**: 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/features/concepts/mod.rs`, `src/ui/concept_table.rs`, `src/ui/concept_editor.rs`, `src/ui/inspector_panel.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
I henhold til FDA-retningslinjer og international interoperabilitet skal begreber i en fællesoffentlig begrebsmodel kunne have en engelsk ækvivalent term:

1. **Domænemodel**:
   - `Concept` udvides med et valgfrit felt: `english_term: Option<String>`.
   - Bagudkompatibilitet sikres via `#[serde(default, skip_serializing_if = "Option::is_none")]`, så eksisterende gemte modeller kan indlæses fejlfrit.
   - Tilføjelse af metoder: `pub fn english_term(&self) -> Option<&str>` og `pub fn set_english_term(&mut self, term: Option<String>)`.
2. **Begrebsliste (Fane 1)**:
   - Visning af "Engelsk term" i begrebsoversigten (`concept_table.rs`) enten som separat kolonne eller som sekundær pille/etiket under/ved siden af "Foretrukken term".
   - Det globale søgefelt i begrebslisten udvides til også at matche på den engelske term.
3. **Egenskaber & Redigering (Inspector)**:
   - Dedikeret inputfelt til "Engelsk term" i Egenskabs-panelet (`concept_editor.rs` og `inspector_panel.rs`), placeret logisk lige efter eller ved siden af "Foretrukken term *".
   - Integreret med tastaturnavigation, undo/autosave og mutation-broadcasting ved live-samarbejde (`collab`).

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Modeludvidelse med engelsk term**: `Concept` indeholder feltet `english_term: Option<String>` med tilhørende getter, setter og konstruktør-støtte.
- [ ] **AC2 - Bagudkompatibel serialisering**: Modeller uden feltet deserialiserer til `english_term: None` uden advarsler eller fejl.
- [ ] **AC3 - Visning i Begrebslisten**: Den engelske term vises overskueligt i begrebstabellen for begreber, hvor den er udfyldt.
- [ ] **AC4 - Søgbarhed**: Indtastning af en engelsk term i søgefeltet filtrerer begrebslisten korrekt.
- [ ] **AC5 - Redigering i Egenskaber**: Brugeren kan indtaste og opdatere den engelske term direkte i Egenskabs-panelet, og ændringer persisteres automatisk.
- [ ] **AC6 - Verifikation via Accepttest**: `test_task_042_concept_english_term_field` beviser at feltet kan oprettes, redigeres, gemmes, søges og læses.

---

## 🚫 Must NOT
- Må IKKE gøre feltet `english_term` obligatorisk (skal forblive `Option<String>`).
- Må IKKE bryde JSON-formatet for eksisterende FDA-modelprojekter.
- Må IKKE overskrive eller forveksle engelsk term med "Godkendt term" eller "Frarådet term".

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_042`
- `cargo clippy --all-targets`
- `cargo fmt --check`
