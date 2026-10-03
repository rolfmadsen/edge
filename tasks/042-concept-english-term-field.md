---
type: Task Package
title: "Task 042: Understøttelse af Engelsk Begrebslag (DA+EN) jf. FDA Skabelon"
description: "Udvidelse af Begrebsmodellen med fuld DA+EN flersprogethed i henhold til den officielle FDA Excel-skabelon (Begrebsliste DA+EN) med visning i tabel, inspektør og editor"
status: active
generated: { by: process:antigravity-task-init, at: "2026-10-03T15:42:00Z" }
tags: [concepts, english-term, multilingual, fda, da-en, concept-table, inspector, merge]
---

# Task 042: Understøttelse af Engelsk Begrebslag (DA+EN) jf. FDA Skabelon

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-21`  
**Genaktiveret & Rescopet**: `2026-10-03` (Genoptaget med fuld DA+EN paritet jf. officiel FDA Excel-skabelon)  
**Scope**: `src/features/concepts/mod.rs`, `src/features/model/merge.rs`, `src/ui/concept_editor.rs`, `src/ui/concept_table.rs`, `src/ui/concept_model_view.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
Understøtte tosprogede (DA+EN) begrebsmodeller i overensstemmelse med Digitaliseringsstyrelsens officielle FDA Begrebslisteskabelon (`docs/Begrebsliste_i_tabelformat_skabelon.xlsx`, arket `Begrebsliste DA+EN`).

Ifølge FDA vejledningen kræver internationalt genbrug, EU-interoperabilitet og standardisering engelske termer og definitioner. Skabelonarket `Begrebsliste DA+EN` fastlægger præcis 7 parallelle engelske metadatafelter:
1. `prefLabel (en)`: Foretrukken term (en)
2. `altLabel (en)`: Accepteret term (en)
3. `hiddenLabel (en)`: Frarådet term (en)
4. `definition (en)`: Definition (en)
5. `example (en)`: Eksempel (en)
6. `comment (en)`: Kommentar (en)
7. `applicationNote (en)`: Anvendelsesnote (en)

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Domænemodel & Serde**: `ConceptEnglishFields` defineres med de 7 FDA DA+EN felter. `Concept` beriges med et valgfrit `english: Option<ConceptEnglishFields>`. Serialisering er 100% bagudkompatibel via `#[serde(default, skip_serializing_if = "Option::is_none")]`.
- [ ] **AC2 - Merge Engine Integration**: 3-vejs model merge engine i `src/features/model/merge.rs` fusionerer de engelske felter konfliktfrit på feltniveau.
- [ ] **AC3 - Concept Editor Formular**: `ConceptEditorState` og UI i `src/ui/concept_editor.rs` udvides med redigering af de engelske felter (foretrukken term, definition, eksempler og noter).
- [ ] **AC4 - Begrebstabel Visning**: `src/ui/concept_table.rs` viser den engelske foretrukne term (fx kursivt med `🇬🇧` ikon) under den danske term for hurtigt flersproget overblik.
- [ ] **AC5 - Egenskabsinspektør**: Canvas-inspektøren i `src/ui/concept_model_view.rs` viser og tillader hurtig redigering af engelsk term og definition for det markerede begreb.
- [ ] **AC6 - Verifikation via Accepttest**: `test_task_042_concept_english_fields_support` i `tests/acceptance.rs` bekræfter komplet livscyklus: oprettelse, JSON persistens, merge og UI-formatering.

---

## 🚫 Must NOT
- Må IKKE bryde eksisterende gemte modeller (alle eksisterende JSON-filer uden engelske felter skal deserialisere fejlfrit).
- Må IKKE gøre engelsk term eller definition obligatorisk (kun de danske felter er obligatoriske jf. FDA Modelreglerne).
- Må IKKE kompromittere offline-first eller introducere eksterne oversættelses-API'er.

---

## 📝 Revisions
- 2026-09-21: Oprettet som simplificeret ad-hoc felt.
- 2026-10-01: Midlertidigt afventende efter monolingual FDA vejledningsgennemgang.
- 2026-10-03: Genaktiveret og udvidet til fuld DA+EN paritet efter identifikation af det officielle `Begrebsliste DA+EN` ark i Digitaliseringsstyrelsens Excel-skabelon.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_042`
- `cargo clippy --all-targets`
- `cargo fmt --check`
