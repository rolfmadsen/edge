---
type: Task Package
title: "Task 060: Machine-Readable Model Interchange: XMI & RDF/Turtle"
description: "Maskinlæsbar serialisering til XMI 2.x for UML-udveksling og RDF Turtle (SKOS/OWL) for integration med det fællesoffentlige Modelkatalog"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, xmi, rdf, turtle, skos, owl, interoperability, model-catalogue]
---

# Task 060: Machine-Readable Model Interchange: XMI & RDF/Turtle

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Opfylde FDA Modelregel 05 (*Gør modellen tilgængelig i maskinlæsbart format*) og sikre interoperabilitet med det fællesoffentlige økosystem:
1. **XMI 2.x Serialisering (UML Standard Interchange)**:
   - Eksportere informations- og begrebsmodellen som gyldig OMG XMI (XML Metadata Interchange).
   - Tillade problemfri import i Enterprise Architect og andre professionelle modelleringsværktøjer.
   - Indeholde pakker med stereotyperne `«ConceptModel»` og `«InformationModel»` samt tagged values for metadata jf. Kapitel 5.2.2.
2. **RDF / Turtle Serialisering (Linked Data)**:
   - Serialisere begreber som `skos:Concept` i et `skos:ConceptScheme` med:
     - `skos:prefLabel` (@da), `skos:altLabel`, `skos:hiddenLabel`
     - `skos:definition` (@da)
     - `skos:broader` / `skos:narrower` (generaliseringer)
     - `skos:related` (associationer)
     - `rdfs:isDefinedBy` (model URI)
     - `dct:source`, `dct:modified`, `owl:versionInfo`
   - Gøre modellen direkte klar til optagelse og fremsøgning i det fællesoffentlige Modelkatalog på `data.gov.dk` jf. Kapitel 8.3.

## 📋 Acceptance Criteria
- [ ] **AC1 - XMI 2.x Generator**: Implementere serialisering af `ModelProject` til valid OMG UML 2.x / XMI format.
- [ ] **AC2 - RDF/Turtle SKOS Generator**: Implementere serialisering af begreber og relationer til valid W3C RDF Turtle syntaks (`.ttl`).
- [ ] **AC3 - Namespace & Prefix Header**: Turtle output indeholder korrekte standardpræfikser (`skos:`, `dct:`, `rdfs:`, `owl:`, `xsd:`) samt modellens eget namespace.
- [ ] **AC4 - UI Eksport Handling**: Mulighed for at vælge `XMI (.xmi)` og `RDF Turtle (.ttl)` fra applikationens eksportdialog.
- [ ] **AC5 - Validations Test**: Headless tests validerer, at det genererede XML og Turtle parses fejlfrit og afspejler modellens indhold.

## 🚫 Must NOT
- Må IKKE introducere tunge C-runtime eller ustabile C++ biblioteker (brug ren Rust XML/Turtle formatering).
- Må IKKE tabe begrebsrelationer eller tagged values under serialisering.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke til opfyldelse af FDA Regel 05 og Modelkatalog-interoperabilitet.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
