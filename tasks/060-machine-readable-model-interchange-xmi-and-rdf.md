---
type: Task Package
title: "Task 060: Machine-Readable Model Interchange: XMI & RDF/Turtle"
description: "Maskinlæsbar serialisering til XMI 2.x for UML-udveksling og RDF Turtle (SKOS/OWL) for integration med det fællesoffentlige Modelkatalog"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, xmi, rdf, turtle, skos, owl, interoperability, model-catalogue]
---

# Task 060: Machine-Readable Model Interchange: XMI & RDF/Turtle

**Status**: `DONE`
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
- [x] **AC1 - UML 2.5 / XMI 2.1 Eksport til Enterprise Architect**: Implementere serialisering af `InformationModel` og `ModelMetadata` til valid OMG UML 2.1/2.5 XMI (`.xmi`), der kan importeres direkte i Sparx Enterprise Architect med klasser, attributter, standard primitive datatyper, multipliciteter, generaliseringer, associationer, kompositioner, stereotyper og tagged values.
- [x] **AC2 - W3C SKOS RDF/Turtle Eksport af Begrebsliste**: Implementere serialisering af begreber (`Concept`), metadata og semantiske relationer (generaliseringer som `skos:broader`/`narrower`, associationer som `skos:related`) til valid W3C RDF Turtle (`.skos.ttl`) struktureret som et `skos:ConceptScheme` med `skos:prefLabel`, `skos:altLabel`, `skos:definition`, `skos:scopeNote`, og kildeangivelser.
- [x] **AC3 - W3C SHACL / OWL RDF/Turtle Eksport af Informationsmodel**: Implementere serialisering af klasser og relationer til W3C SHACL Shapes og OWL Ontology (`.shacl.ttl`), hvor hver informationsklasse modelleres som `owl:Class` og `sh:NodeShape` med tilhørende `sh:property` shapes for attributter, korrekte `xsd:` datatyper, `sh:minCount`/`sh:maxCount` multiplicitetsrestriktioner samt `sh:in` begrænsninger for enumerations.
- [x] **AC4 - UI Eksport Integration**: Udvide topbar-menuen "Eksporter" med en dedikeret sektion for maskinlæsbare formater: `Enterprise Architect (XMI 2.1)...`, `Begrebsliste (W3C SKOS Turtle)...` og `Informationsmodel (W3C SHACL/OWL Turtle)...` forbundet med native `rfd` fildialoger.
- [x] **AC5 - Validations Test Suite**: Etablere automatiseret testsuite i `tests/acceptance.rs`, der headless beviser syntaktisk validitet, korrekt escaping og fuld semantisk overensstemmelse for samtlige tre maskinlæsbare formater.

## 🚫 Must NOT
- Må IKKE introducere eksterne C/C++ biblioteker eller tunge bindings (brug ren, deterministisk Rust XML og Turtle formatering).
- Må IKKE bryde Zero-Daemon invarianten ved at starte baggrunds-triplestores eller eksterne sockets.
- Må IKKE producere udokumenterede ikke-standardiserede XML- eller RDF-dialekter, der fejler ved validering i standardværktøjer som Sparx EA eller W3C validators.
- Må IKKE tabe begrebsrelationer, attributmultipliciteter eller tagged values under serialisering.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke til opfyldelse af FDA Regel 05 og Modelkatalog-interoperabilitet.
- 2026-09-30: Implementeret xmi.rs, skos.rs, shacl.rs og integreret i UI samt verifieret via testsuite.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
