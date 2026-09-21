---
type: Task Package
title: "Task 039: Canvas Tekstafskæring og Tastaturfokuseret Oprettelse i Egenskaber"
description: "Tekstafkortning/cutoff med ellipsis for lange attributter på klasser samt tastaturfokuseret oprettelse af begreber og klasser direkte i Egenskaber"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-21T19:20:00Z" }
tags: [canvas, class, text-cutoff, ellipsis, inspector, focus, ergonomics]
---

# Task 039: Canvas Tekstafskæring og Tastaturfokuseret Oprettelse i Egenskaber

**Status**: `DONE`  
**Intent**: 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/ui/diagram_canvas.rs`, `src/ui/information_model_view.rs`, `src/ui/concept_model_view.rs`, `src/ui/app.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
1. **Tekstafkortning (Cutoff / Ellipsis) på informationsmodellens klasser**:
   - Klassernes bredde bevares fast på `DEFAULT_CLASS_NODE_WIDTH = 220.0` (ingen udvidelse af boksens bredde).
   - Tekst på attributter (`+ <navn> : <Type> [<mult>] 🔗`) og klassenavn der overskrider nodens indvendige bredde (ca. 192 px til rådighed efter padding) afkortes pænt med `...` (ellipsis).
   - Teksten renderes aldrig ud over klassens højre ramme.
   - Den fulde tekst kan altid inspiceres og redigeres i højre Egenskabs-panel (og/eller ved hover).
2. **Ensrettet, tastaturfokuseret oprettelse direkte i Egenskaber**:
   - **Begrebsmodellen (Fane 2)**:
     - `+ Opret begreb` i værktøjslinjen og dobbeltklik på lærredet opretter et begreb direkte på canvas og vælger det.
     - Egenskabs-panelet åbnes straks i redigeringstilstand for det nye begreb, og inputfeltet "Foretrukken term *" modtager automatisk tastaturfokus (`operation::focus("preferred_term_input")`).
     - Den forstyrrende svævende popup-dialog udfases.
     - Brugeren kan efterfølgende skifte til Begrebslisten (Fane 1) for at udfylde uddybende detaljer (kilder, definition, etc.).
   - **Informationsmodellen (Fane 3)**:
     - `+ Opret klasse` i værktøjslinjen og dobbeltklik på lærredet tildeler fokus direkte til klassenavnefeltet i Egenskaber (`operation::focus("info_class_name_input")`).

---

## 📋 Acceptance Criteria
- [x] **AC1 - Klassetekst overskrider ikke bredden**: Attributter og klassenavne på informationsmodellens lærred afkortes med ellipsis `...`, hvis deres bredde overskrider nodens indvendige plads (220.0 px minus margin).
- [x] **AC2 - Tastaturfokus ved ny klasse**: `CreateInformationClassAtCenter` og `CreateInformationClassAt` fokuserer automatisk inputfeltet `"info_class_name_input"` i Egenskaber.
- [x] **AC3 - Ensrettet oprettelse på Begrebsmodel**: `CreateConceptAtCenter` og dobbeltklik på Begrebsmodellen opretter begrebet direkte på lærredet, vælger det og sætter tastaturfokus i `"preferred_term_input"` i Egenskaber.
- [x] **AC4 - Bevarelse af 220px klasse-geometri**: Klassenoder bevarer deres faste bredde (220.0 px) og standard højdeberegning.
- [x] **AC5 - Verifikation via Accepttest**: `test_task_039_canvas_class_text_cutoff_and_inspector_focus` beviser afskæring og fokus-adfærd.

---

## 🚫 Must NOT
- Må IKKE øge klassens bredde over 220.0 px.
- Må IKKE klippe tekst hårdt uden pæn ellipsis (`...`).
- Må IKKE forhindre redigering af det fulde attributnavn i Egenskaber.

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_039`
- `cargo clippy --all-targets`
- `cargo fmt --check`
