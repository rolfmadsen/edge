# Task 036: Domæne-vælger (Ja/Nej) i begrebseditor & oprydning i lærredspalet

**Status**: `DONE`  
**Intent**: 🔄 `ENHANCEMENT`  
**Dato**: `2026-09-20`  
**Scope**: `src/ui/concept_editor.rs`, `src/ui/concept_model_view.rs`, `src/ui/app.rs`, `tests/acceptance.rs`  

---

## 🎯 Formål
Forbedrer brugergrænsefladen for oprettelse og redigering af begreber samt renser lærredets venstre sidepanel og topheader:
1. **Domæne / Kontekst (Tilhører emneområde §26)**:
   - Erstatte det uklare fritekstfelt med to segmenterede knapper: `[ Lokalt begreb (Ja) ]` og `[ Indlånt begreb (Nej) ]`.
   - Hvis "Indlånt begreb (Nej)" vælges, fremvises et valgfrit underfelt til `Kildemodel URI` med hjælpetekst.
   - Forbedre hjælpeteksten under `Identifikator (HTTP-URI)` (Bilag D), så det fremgår tydeligt, at dette er begrebets eget unikke ID, uafhængigt af en eventuel kildemodel-URI.
2. **Oprydning i Begrebsmodel-paletten**:
   - Fjerne den overflødige `+ Nyt begreb` knap øverst til højre i venstre panel på Begrebsmodel-fanen for at skabe et rent og ensartet udtryk svarende til Informationsmodellen.
3. **Oprydning i top-headeren**:
   - Fjerne det overflødige `FDA v2.1` badge ved siden af "Kant" i brand-sektionen i headeren, da modelreglerne allerede fremgår med klikbart link i footeren.

---

## 📋 Acceptance Criteria
- [x] **AC1 - Segmenteret domænevælger**: `ConceptEditorState` har `belongs_to_domain: BelongsToDomain` og `model_uri: String`. Knapperne `Lokalt begreb (Ja)` og `Indlånt begreb (Nej)` opdaterer tilstanden via `Message::SetConceptDomain`.
- [x] **AC2 - Betinget Kildemodel URI**: Når `Indlånt begreb (Nej)` er valgt, vises tekstfeltet til `Kildemodel URI`. Ved gemning med udfyldt URI oprettes begrebet med `BelongsToDomain::ModelRef(uri)`. Hvis URI er tom, oprettes det med `BelongsToDomain::No`.
- [x] **AC3 - Bagudkompatibilitet**: `editor.update_field(ConceptFormField::BelongsToDomain, ...)` fungerer fortsat fejlfrit for ældre kald og tests.
- [x] **AC4 - Palet-header oprydning**: Knappen `+ Nyt begreb` er fjernet fra venstre palet i `concept_model_view.rs`.
- [x] **AC5 - Header-badge oprydning**: `FDA v2.1` badget er fjernet fra headerens brand-sektion for at give et renere udtryk.
- [x] **AC6 - Verifikation**: Alle tests i suiten passerer uden regressionsfejl.

---

## 🚫 Must NOT
- Eksisterende begreber med `BelongsToDomain::ModelRef(...)` må ALDRIG miste deres URI ved åbning i editoren.
- `Message::StartNewConcept` fra Begrebsliste-hovedtabellen må IKKE fjernes eller brydes.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task036_concept_editor_domain_toggle_and_model_ref`
- `cargo test --workspace`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
