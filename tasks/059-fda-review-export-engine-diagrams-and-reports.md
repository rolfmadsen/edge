---
type: Task Package
title: "Task 059: FDA Review Export Engine: Diagrams & Reports"
description: "Eksportmotor til officiel FDA review-indlevering: Vektor-SVG og PNG diagrammer, CSV/Excel begrebsliste i Bilag D/E format og samlet modelrapport"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, export, svg, png, csv, excel, review-package, report]
---

# Task 059: FDA Review Export Engine: Diagrams & Reports

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Gøre det muligt at eksportere modelprojekter som officielle indleveringspakker til FDA Modelteknisk Review jf. Kapitel 7 og Kapitel 8.2.1:
1. **Diagram-eksport (Vektor & Billede)**:
   - Eksport af det aktive lærred (Begrebsmodel eller Informationsmodel) til standardiseret **vektor-SVG** i høj kvalitet.
   - Eksport til højopløselig **PNG** (f.eks. til præsentationer og dokumenter).
2. **Begrebsliste-eksport (Tabelform jf. Bilag D & E)**:
   - Eksport af begrebslisten til **CSV** og **Excel-kompatibel HTML/XML tabel** med samtlige 12 standardkolonner:
     1. Foretrukken dansk term
     2. Accepteret dansk term
     3. Frarådet dansk term
     4. Definition
     5. Eksempel
     6. Kommentar
     7. Anvendelsesnote
     8. Juridisk kilde (ELI)
     9. Kilde
     10. Tilhører emneområde (Ja / Nej / ModelRef URI)
     11. Identifikator (HTTP-URI)
     12. Afledt af (HTTP-URI)
3. **Modelrapport & Afleveringsdokumentation**:
   - Generering af en komplet Markdown / HTML rapport med projektets metadata (Tabel D), forretningsformål, begrebsdefinitioner og diagrammer.

## 📋 Acceptance Criteria
- [ ] **AC1 - Standardiseret Vektor-SVG Eksport**: Eksport af aktivt diagramlærred (Begrebsmodel og Informationsmodel) til standardiseret, velformet SVG med FDA styling (sand/blå/grøn/gul baggrunde, stereotyper, attributter, ortogonale linjer, generaliseringstrekanter, kompositionsdiamanter, associationspile og multipliciteter). ViewBox afpasses dynamisk efter elementernes ydergrænser med passende margin.
- [ ] **AC2 - RFC-4180 CSV-eksport af Begrebsliste**: Eksport af samtlige begreber i tabellen som RFC-4180 kompatibel CSV med UTF-8 BOM (`\u{FEFF}`) og præcis de 12 FDA standardkolonner fra Bilag D & E:
  1. `Foretrukken term`
  2. `Accepteret term`
  3. `Frarådet term`
  4. `Definition`
  5. `Eksempel`
  6. `Kommentar`
  7. `Anvendelsesnote`
  8. `Juridisk kilde`
  9. `Kilde`
  10. `Tilhører emneområde`
  11. `Identifikator`
  12. `Afledt af`
  Korrekt escaping af citationstegn, linjeskift og kommaer samt bevarelse af æ, ø, å.
- [ ] **AC3 - Samlet Modelrapport (Markdown & HTML)**: Generering af officiel indleveringsrapport i både Markdown (`.md`) og selvstændig stylet HTML (`.html`) indeholdende:
  - Dokumenttitel og modelnavn
  - Indholdsfortegnelse (TOC)
  - Tabel D modelmetadata (navn, URI, ansvarlig myndighed, status, godkendelse, version, sprog, dato, etc.)
  - Lovgrundlag & juridiske kilder
  - Begrebskatalog (definitioner per genus et differentiam, kilder, noter)
  - Informationsmodel-oversigt (klasser, attributter, udfaldsrum, multipliciteter, relationer).
- [ ] **AC4 - Eksportmenu i Brugerfladen**: Menu og knapper i UI (`Eksporter` dropdown i topbaren samt handlinger i respektive visninger) for:
  - `Eksportér SVG-diagram...`
  - `Eksportér begrebsliste (CSV)...`
  - `Eksportér afleveringsrapport (Markdown / HTML)...`
  Integreret med `rfd` native fildialoger og statusfeedback.
- [ ] **AC5 - Headless Snapshot- og Accepttests**: Komplet testsuite i `tests/acceptance.rs` der verificerer:
  - Deterministisk generering af SVG med korrekte tags og farvekoder for både begrebs- og informationsmodel.
  - RFC-4180 validering af CSV-output med UTF-8 BOM og særtegn.
  - Validering af rapportindhold (metadata, TOC, begreber, klasser).
  - TEA message-flow for eksport-handlinger.

## 🚫 Must NOT
- Må IKKE afhænge af eksterne cloud-konvertere eller eksterne services (100% lokal generering i Rust).
- Må IKKE tabe specialtegn (æ, ø, å, Unicode) under eksport.
- Må IKKE bryde RFC-4180 standarden for CSV (escaping med dobbelte anførselstegn ved komma/linjeskift).
- Må IKKE fejle hvis lærredet er tomt (skal generere gyldig SVG med tom/default viewBox).

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke til opfyldelse af FDA Kapitel 8.2.1 indleveringskrav.
- 2026-09-30: Skærpet specifikation for SVG, RFC-4180 CSV, Markdown/HTML rapport og UI eksportmenu.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
