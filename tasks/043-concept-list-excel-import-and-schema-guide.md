---
type: Task Package
title: "Task 043: Import af Begrebsliste via Excel med Skabelon og Visuel Skemaguide"
description: "Hurtig masse-oprettelse af begreber via Excel (.xlsx) import med integreret skabelongenerering, visuel skemaguide og validering af obligatoriske felter"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [import, excel, xlsx, schema-guide, concepts, validation, ergonomics]
---

# Task 043: Import af Begrebsliste via Excel med Skabelon og Visuel Skemaguide

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/features/concepts/mod.rs`, `src/features/concepts/excel_import.rs`, `src/ui/concept_table.rs`, `src/ui/app.rs`, `Cargo.toml`, `tests/acceptance.rs`

---

## 🎯 Formål
Gøre det hurtigt og fejlfrit for domæneeksperter og forretningsanalytikere at oprette store mængder begreber på én gang ved at importere dem direkte fra et Excel-regneark (`.xlsx`):

1. **Fastlæggelse af Felter & Valideringsregler**:
   - **Obligatoriske felter (kræves for at oprette et gyldigt FDA-begreb)**:
     - `Foretrukken term` / `Navn` * (ikke-tom tekststreng, trimmet)
     - `Definition` * (ikke-tom tekststreng, trimmet)
   - **Valgfrie felter**:
     - `Engelsk term` (jf. Task 042)
     - `Tilhører domæne` / `Indgår i domæne` ("Ja", "Nej", eller model-URI. Standard: "Ja")
     - `Godkendt term`
     - `Frarådet term`
     - `Eksempel`
     - `Note` / `Kommentar`
     - `Anvendelsesnote`
     - `Kilde`
     - `Retskilde`
     - `Identifikator` / `ID` (valgfri; genereres automatisk som UUID hvis udeladt)
2. **Hjælp til at forstå schemaet (User Ergonomics)**:
   - **"Hent Excel Skabelon"**: En knap i Begrebslistens topbar, der lader brugeren gemme en færdigskabelon (`begreber-skabelon.xlsx`) med korrekte kolonnenavne, tydelig markering af obligatoriske felter med stjerne (*), og 2 realistiske eksempel-rækker.
   - **Visuel Skemaguide Modal ("Se Excel Format & Krav")**: En integreret dialog i appen, der præsenterer skemaet i en ren tabel: feltnavn, status (Obligatorisk / Valgfri), beskrivelse og eksempelværdier.
3. **Import Flow med Forhåndsvisning og Validering**:
   - Brugeren klikker "Importér Excel" og vælger en fil via native fildialog (`rfd`).
   - Excel-arket parses sikkert i baggrunden (via `calamine`). Kolonner matches fleksibelt (både danske og engelske overskrifter accepteres uafhængigt af store/små bogstaver).
   - En forhåndsvisningsmodal opsummerer resultatet:
     - F.eks. "*24 begreber fundet: 22 gyldige og klar til import, 2 afvist pga. manglende navn eller definition (række 4 og 11)*".
   - Brugeren bekræfter importen, hvorefter de gyldige begreber føjes til projektets begrebsliste, autosave aktiveres, og listen opdateres med det samme.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Robust Excel Parsing**: Parseren kan indlæse `.xlsx` filer (og fallback standard `.csv`) og identificere kolonner uafhængigt af rækkefølge og store/små bogstaver.
- [ ] **AC2 - Validering af Obligatoriske Felter**: Rækker uden både en ikke-tom `Foretrukken term` og `Definition` afvises med en klar årsagsforklaring (rækkenummer og manglende felt).
- [ ] **AC3 - Håndtering af Valgfrie Felter**: Udfyldte valgfrie felter (engelsk term, domæne, eksempler, noter, kilder) mappes korrekt over i `Concept`.
- [ ] **AC4 - Hent Skabelon-funktion**: Brugeren kan med et klik eksportere/gemme en gyldig `.xlsx` skabelonfil med forbillede-data.
- [ ] **AC5 - Visuel Skemaguide**: En modal i brugergrænsefladen forklarer kolonneskemaet og reglerne pædagogisk.
- [ ] **AC6 - Forhåndsvisnings- og bekræftelsesdialog**: Importen udføres ikke blindt; brugeren ser antallet af gyldige/ugyldige rækker før endelig bekræftelse.
- [ ] **AC7 - Verifikation via Accepttest**: `test_task_043_concept_list_excel_import_and_schema_guide` beviser at skabelon genereres, Excel-data parses og valideres korrekt, og begreber tilføjes til modellen.

---

## 🚫 Must NOT
- Må IKKE crashe eller panikke på korrupte filer, ukendte arknavne eller tomme rækker.
- Må IKKE importere begreber, der mangler foretrukken term eller definition.
- Må IKKE overskrive eksisterende begreber medmindre brugeren eksplicit vælger "Overskriv ved navne-sammenfald".
- Må IKKE introducere tunge C-runtime eller eksterne usikre dependencies (`calamine` anvender 100% pure Rust).

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_043`
- `cargo clippy --all-targets`
- `cargo fmt --check`
