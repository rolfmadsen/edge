# Task 031: FDA Informationsklasse Egenskaber (Abstrakt, Indlånt/Lokal) og Rendering

**Status**: ACTIVE  
**Intent**: 🚀 NEW FEATURE  

## 🎯 Formål
Gøre det muligt at angive FDA- og UML-egenskaber for informationsklasser i Informationsmodellen:
1. Understøtte **Abstrakte klasser** (`is_abstract: bool`) i datamodellen, inspectoren og på canvas via kursiv og `{abstract}` markering.
2. Understøtte **Fremmede/Indlånte klasser** (`is_local: bool` og valgfri `origin_model`) jf. FDA Modelleringsvejledning Kapitel 2, 5 og 7.
3. Kable visualiseringen på canvas til klassens faktiske egenskaber:
   - Sandfarvet (`#FEFAF7`) for lokale klasser.
   - Blå (`#87CDEB`) for indlånte/fremmede klasser.
4. Tilføje intuitive kontroller i højre inspector og oversigtsindikatorer i venstre palet.

## 📋 Acceptance Criteria
- [ ] **AC1 (Datamodel)**: `InformationClass` har felterne `is_abstract: bool` (default `false`), `is_local: bool` (default `true`) og `origin_model: Option<String>` (default `None`) med fuld Serde-bagudkompatibilitet (`#[serde(default)]`).
- [ ] **AC2 (Begrebsarv)**: `InformationClass::from_concept(concept)` initialiserer automatisk `is_local` og eventuel `origin_model` ud fra begrebets `BelongsToDomain`.
- [ ] **AC3 (Inspector-kontroller)**: Når en informationsklasse er valgt i inspectoren, kan brugeren slå `Abstrakt` til/fra, skifte mellem `Lokal` og `Indlånt/Fremmed`, samt angive kildemodel/URI for indlånte klasser.
- [ ] **AC4 (Canvas Farvekodning)**: Canvas-rendering benytter `FDA_SAND` (`#FEFAF7`) for lokale klasser og `FDA_BORROWED_BLUE` (`#87CDEB`) for indlånte klasser, herunder konsistente skillelinjer og kantfarver.
- [ ] **AC5 (Canvas Abstrakt Notation)**: Abstrakte klasser visualiseres i overensstemmelse med UML 2.5 / FDA med kursiveret klassenavn og `{abstract}` egenskabsmærke.
- [ ] **AC6 (Palet-indikatorer)**: Venstre klassepalet reflekterer klassens status visuelt (kursivt navn for abstrakte og blå markør/badge for indlånte klasser).
- [ ] **AC7 (Regression & Verifikation)**: Omfattende accepttest i `tests/acceptance.rs` verificerer hele flowet, og hele testsuiten passerer (`cargo test --workspace` og `cargo clippy`).

## 🚫 Must NOT
- Må IKKE bryde bagudkompatibilitet ved deserialisering af eksisterende `model.edge.json` projekter.
- Må IKKE ændre geometri, knækpunkter eller portberegning for de rettede relationer fra Task 030.
- Må IKKE tillade uigennemskuelige farver, der bryder FDA farvekonventionerne i Kap. 7.2.

## 📝 Revisions
- 2026-09-20: Oprettet efter brugeranmodning og FDA-regelafklaring.

## 🧪 Verifikation
- `cargo test --test acceptance test_task_031`
- `cargo test --workspace`
- `cargo clippy --all-targets -- -D warnings`
