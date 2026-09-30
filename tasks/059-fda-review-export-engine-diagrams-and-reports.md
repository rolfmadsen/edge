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
- [ ] **AC1 - SVG Canvas Eksport**: Diagramlærredet kan eksporteres som gyldig SVG-fil med korrekte FDA-farver, tekstplacering og forbindelseslinjer.
- [ ] **AC2 - Begrebsliste CSV/Excel Eksport**: Begrebslisten kan eksporteres som RFC-4180 kompatibel CSV (med UTF-8 BOM til Excel) indeholdende alle 12 FDA standardkolonner.
- [ ] **AC3 - Modelrapport Generering**: En knap i UI genererer en samlet afleveringsrapport med metadata, lovgrundlag og indholdsfortegnelse.
- [ ] **AC4 - UI Eksportmenu**: Topbaren/burger-menuen forsynes med en "Eksporter"-undermenu (`SVG-diagram`, `Begrebsliste (CSV)`, `Afleveringsrapport (Markdown)`).
- [ ] **AC5 - Headless Snapshot Tests**: Testsuite verificerer deterministisk SVG- og CSV-output fra en dummy-model.

## 🚫 Must NOT
- Må IKKE afhænge af eksterne cloud-konvertere (alt genereres 100% lokalt i Rust).
- Må IKKE tabe specialtegn (æ, ø, å, Unicode) under eksport.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke til opfyldelse af FDA Kapitel 8.2.1 indleveringskrav.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
