---
type: Task Package
title: "Task 043: Excel Import, Upsert, Skabelon & Dataeksport for Begrebsliste"
description: "Masse-håndtering af begreber via Excel (.xlsx) med import (sikker upsert på ID/navn), eksport af tom FDA-skabelon, de facto dataeksport af projektets begrebsliste og visuel skemaguide"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [import, export, excel, xlsx, upsert, schema-guide, concepts, validation, fda, ergonomics]
---

# Task 043: Excel Import, Upsert, Skabelon & Dataeksport for Begrebsliste

**Status**: `ACTIVE`  
**Intent**: 🚀 `NEW FEATURE`  
**Oprettet**: `2026-09-21`  
**Opdateret**: `2026-10-03` (Udvidet med de facto Excel-dataeksport, eksport af tom FDA-skabelon samt intelligent upsert)  
**Scope**: `src/features/concepts/mod.rs`, `src/features/concepts/excel.rs`, `src/ui/concept_table.rs`, `src/ui/app.rs`, `Cargo.toml`, `tests/acceptance.rs`

---

## 🎯 Formål
Gøre det hurtigt, fleksibelt og fejlfrit for forretningsanalytikere og domæneeksperter at arbejde med FDA-begreber i Excel:

1. **Eksport af Tom FDA-skabelon**:
   - Brugeren kan med ét klik ("Hent tom FDA-skabelon") gemme den officielle skabelon (`Begrebsliste_i_tabelformat_skabelon.xlsx`), perfekt når en ny model startes op.
2. **De Facto Excel-Dataeksport**:
   - Eksport af hele projektets begrebsliste til et rigtigt `.xlsx`-regneark, der opfylder Digitaliseringsstyrelsens layout:
     - Arket `Forretningsmetadata`: Modellens navn, URI, beskrivelse, sprog, status m.v.
     - Arket `Begrebsliste DA`: Udfyldt med modellens danske begreber.
     - Arket `Begrebsliste DA+EN`: Udfyldt med både danske og engelske felter jf. Task 042.
3. **Import Flow med Intelligent Upsert & Forhåndsvisning**:
   - Brugeren vælger en `.xlsx`-fil via fildialog (`rfd`).
   - Parseren indlæser rækker uafhængigt af kolonneplacering og store/små bogstaver.
   - **Upsert-matching**:
     - Hvis en rækkes `Identifikator` (URI/ID) matcher et eksisterende begreb, eller hvis `Foretrukken term` (trimmet, case-insensitiv) matcher et eksisterende begreb: **opdateres** det eksisterende begreb. Dens interne `Uuid` bevares, så placeringer på diagrammer og relationer ikke brydes!
     - Hvis intet match findes: **indsættes** det som et nyt begreb.
   - **Forhåndsvisningsdialog**:
     - Viser overblik: F.eks. *"24 begreber fundet i Excel: 18 nye indsættes, 4 eksisterende opdateres (upsert), 2 afvist pga. manglende navn eller definition"*.
     - Brugeren bekræfter før importen endeligt skrives til modellen og gemmes.
4. **Visuel Skemaguide Modal**:
   - En ren hjælpedialog i appen ("Se Excel Format & Krav"), der viser samtlige FDA felter, obligatorisk/valgfri status og formkrav.

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Robust Excel Parsing**: Parseren kan indlæse `.xlsx` filer (både `Begrebsliste DA` og tosproget `Begrebsliste DA+EN`, samt fallback standard `.csv`) og identificere kolonner uafhængigt af rækkefølge og store/små bogstaver.
- [ ] **AC2 - Validering af Obligatoriske Felter**: Rækker uden både en ikke-tom `Foretrukken term` og `Definition` afvises med klar årsagsforklaring (rækkenummer og manglende felt).
- [ ] **AC3 - Intelligent Upsert**: Import opdaterer eksisterende begreber (matchet på `identifier` eller `preferred_term`) uden at ændre deres `Uuid`, og opretter nye begreber for ukendte termer.
- [ ] **AC4 - Eksport af Tom FDA-skabelon**: Brugeren kan med ét klik eksportere den originale officielle `Begrebsliste_i_tabelformat_skabelon.xlsx` fil.
- [ ] **AC5 - De Facto Dataeksport til Excel**: Modellen kan eksporteres til en gyldig `.xlsx` fil med udfyldte metadata og begreber i `Begrebsliste DA` og `Begrebsliste DA+EN`.
- [ ] **AC6 - Visuel Skemaguide**: En modal i brugergrænsefladen forklarer kolonneskemaet og reglerne pædagogisk.
- [ ] **AC7 - Forhåndsvisnings- og bekræftelsesdialog**: Brugeren præsenteres for antal nye, antal opdaterede (upsert) og eventuelle afviste rækker før bekræftelse.
- [ ] **AC8 - Verifikation via Accepttest**: `test_task_043_concept_list_excel_import_upsert_and_export` beviser at skabelon eksporteres, data eksporteres til `.xlsx`, Excel-data parses og upsertes korrekt, og begreber opdateres i modellen.

---

## 🚫 Must NOT
- Må IKKE ændre eksisterende begrebers `Uuid` under upsert (det ville ødelægge diagram-noder og relationer).
- Må IKKE crashe eller panikke på korrupte filer, ukendte arknavne eller tomme rækker.
- Må IKKE importere begreber, der mangler foretrukken term eller definition.
- Må IKKE introducere tunge C-runtime eller eksterne usikre dependencies (anvend 100% pure Rust: `calamine` til læsning og `rust_xlsxwriter` til skrivning).

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke.
- 2026-10-03: Udvidet med tosprogethed (DA+EN jf. Task 042), de facto Excel dataeksport, eksport af tom officiel FDA-skabelon og intelligent upsert-matching.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_043`
- `cargo clippy --all-targets`
- `cargo fmt --check`
