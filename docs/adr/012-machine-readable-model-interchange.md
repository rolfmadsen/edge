---
type: Architectural Decision Record
title: 'ADR 012: Maskinlæsbar Modeludveksling via OMG UML 2.5 / XMI 2.1 og W3C SKOS/SHACL RDF'
status: accepted
tags: [architecture, adr, fda, xmi, rdf, turtle, skos, shacl, owl, sparx-ea, model-catalogue]
---

# 12. Maskinlæsbar Modeludveksling via OMG UML 2.5 / XMI 2.1 og W3C SKOS/SHACL RDF

**Status**: `accepted`  
**Date**: `2026-09-30`  

## Context
FDA Modelregel 05 fastsætter krav om, at forretnings- og datamodeller skal stilles til rådighed i åbne, maskinlæsbare formater, så modeller kan udveksles på tværs af myndigheder og værktøjer samt indgå i det fællesoffentlige Modelkatalog på `data.gov.dk`:
1. **UML-værktøjsinteroperabilitet**: Mange offentlige institutioner anvender Sparx Systems Enterprise Architect (EA) eller lignende UML-værktøjer til virksomhedsarkitektur. For at understøtte tovejs eller nedstrøms brug i EA skal modeller kunne eksporteres i en standardiseret XMI-dialekt, som EA kan importere uden tab af attributter, typer, multipliciteter, generaliseringer og relationer.
2. **Semantisk web & Modelkataloget (SKOS)**: Begrebslisten og begrebsmodellen skal kunne registreres som et kontrolleret vokabular i Modelkataloget jf. W3C Simple Knowledge Organization System (SKOS) standarden med hierarkiske (`broader`/`narrower`) og associative (`related`) relationer.
3. **Validering & Data Shapes (SHACL/OWL)**: Informationsmodellen definerer klasser, egenskaber og forretningsrestriktioner. For at understøtte maskinel datavalidering i datadistributionsplatforme og API-gateways er W3C SHACL (Shapes Constraint Language) og OWL (Web Ontology Language) den officielle EU- og fællesoffentlige standard.
4. **Zero-Daemon & Portabilitet**: Kant skal bevare sin lette, deterministiske arkitektur uden tunge C++ biblioteker (som libxml2 eller Raptor) og uden at køre lokale triplestores eller SPARQL-daemons.

## Decision
1. **OMG UML 2.1 / XMI 2.1 som primær XMI-dialekt til Sparx EA**:
   - Vi implementerer en ren Rust XML-generator, der danner standardiseret OMG UML 2.1 / XMI 2.1 (`.xmi`).
   - XMI 2.1 er den mest stabile og bredt understøttede udvekslingsstandard i Sparx Enterprise Architect (EA importerer OMG UML 2.1 uden de proprietære tab, som ofte ses ved version 2.5/2.5.1 XMI-dialekter).
   - Informationsmodellen eksporteres som en `uml:Package` under en `uml:Model` med:
     - `uml:Class` for hver klasse med `isAbstract` og dokumentation.
     - `uml:Property` for hver attribut med datatyper (PrimitiveTypes, enumerations og strukturerede typer) samt `lowerValue` og `upperValue`.
     - `uml:Generalization` for generaliseringshierarkier.
     - `uml:Association` for associationer og kompositioner (`aggregation="composite"`).
     - Stereotyper `«InformationModel»` og `«ConceptModel»` samt FDA metadata som tagged values.

2. **W3C SKOS i RDF/Turtle (`.skos.ttl`) for Begrebslisten**:
   - Begrebslisten serialiseres som et `skos:ConceptScheme` med standardpræfikser (`skos:`, `dct:`, `rdfs:`, `owl:`, `xsd:`).
   - Hvert begreb modelleres som `skos:Concept` med:
     - Stabil URI (fra `identifier` eller modellens namespace).
     - `skos:prefLabel "..."@da`, `skos:altLabel "..."@da`, `skos:hiddenLabel "..."@da`.
     - `skos:definition "..."@da`, `skos:scopeNote`, `skos:example`, `rdfs:comment`.
     - `skos:broader` / `skos:narrower` afledt af begrebsmodellens generaliseringer.
     - `skos:related` afledt af begrebsmodellens associationer.
     - `dct:source` for juridiske kilder og litteraturkilder.

3. **W3C SHACL Shapes & OWL Ontology (`.shacl.ttl`) for Informationsmodellen**:
   - Hver informationsklasse modelleres dualt som `owl:Class` og `sh:NodeShape` med `sh:targetClass`.
   - Attributter modelleres som `sh:property` med `sh:path`, `sh:name`, `sh:datatype` (W3C XSD datatyper) og `sh:minCount` / `sh:maxCount` multipliciteter.
   - Enumerations modelleres med `sh:in ( "værdi1" "værdi2" )` eller `owl:oneOf`.
   - Klasse-relationer modelleres med `sh:property` og `sh:class <TargetClassUri>`.

4. **Deterministisk, Zero-Dependency Rust Serialisering**:
   - Al generering sker med deterministisk ordning af elementer og tripler for at sikre reproducerbarhed og forhindre støj i versionstyring.
   - Ingen eksterne C-runtimes eller eksterne netværksservices under eksport (opfylder Zero-Daemon og Zero Ambient Authority).

## Consequences
- **Positive konsekvenser**:
  - Fuld tovejs værktøjsinteroperabilitet med Sparx Enterprise Architect og andre UML-værktøjer.
  - Direkte kompatibilitet med Digitaliseringsstyrelsens Modelkatalog på `data.gov.dk` via SKOS RDF/Turtle.
  - Maskinvaliderbare datakontrakter via W3C SHACL.
  - Hurtig, robust headless eksport uden tunge eksterne afhængigheder.
- **Trade-offs**:
  - Sparx EA har visse proprietære diagramlayout-tags, som ikke er en del af den rene OMG XMI 2.1 standard; elementer placeres derfor i pakketræet i EA og kan automatisk arrangeres via EAs interne diagramlayout-motor.
