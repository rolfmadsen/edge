---
type: Task Package
title: "Task 010: Informationsmodel - Klasser, Attributter og Begrebssporing"
description: "Integration af Informationsmodel med klasser, attributter, FDA primitive datatyper, multipliciteter og M:N begrebssporing i ModelProject og UI Fane 4"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T18:09:00Z" }
tags: [task-lifecycle, intent, information-model, uml, classes, attributes, primitive-types, multiplicity, fda]
---

# Task 010: Informationsmodel - Klasser, Attributter og Begrebssporing

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Etablere Trin 3 i FDA progressionen i `edge`: En fuldgyldig Informationsmodel med UML-klasser, attributter, autoritative FDA standard primitive datatyper og multipliciteter.
Hver klasse og hver attribut skal kunne eksistere selvstændigt eller have en eksplicit sporbarhedsrelation (M:N) til et eller flere begreber fra begrebslisten jf. FDA Modelreglernes principper for sporbarhed.
Informationsmodellen skal integreres i `ModelProject`, persisteres til disk, og gøres fuldt redigerbar i en moderne master-detail editor i Fane 4 i Iced brugerfladen.

## 📋 Acceptance Criteria
- [x] **Kerne-datastrukturer og Begrebssporing (`src/features/information_model/`)**:
  - `InformationClass` har `id: Uuid`, `name: String`, `description: Option<String>`, `concept_ids: Vec<Uuid>`, samt `attributes: Vec<Attribute>`.
  - `Attribute` har `id: Uuid`, `name: String` (valideret med lowerCamelCase advarsel/tjek jf. FDA §6.3), `data_type: PrimitiveType`, `multiplicity: Multiplicity`, og `concept_ids: Vec<Uuid>`.
  - `InformationModel` etableres som container med metoder til CRUD på klasser og attributter, samt opslag af klasser knyttet til et specifikt begreb.
- [x] **ModelProject & Disk-Persistens (`src/features/model/`)**:
  - `ModelProject` indeholder `information_model: InformationModel` med `#[serde(default)]` for at bevare fuld bagudkompatibilitet med eksisterende filer.
  - Fuld disk-persistens via `storage.rs` verificeret med round-trip serialisering og deserialisering.
- [x] **Fane 4 UI: Master-Detail Editor (`src/ui/app.rs`)**:
  - Venstre kolonne: Liste over klasser, søgning/filtrering, `+ Ny Klasse` knap, og mulighed for hurtigt at oprette en klasse fra et begreb.
  - Højre kolonne: Detaljevisning for valgt klasse med redigering af navn, beskrivelse, tilknyttede begreber (multi-select / badge-vælger), samt tabel over klassens attributter med tilføj, rediger og slet.
- [x] **ADR 006**:
  - Oprettelse af `docs/adr/006-information-model-and-concept-traceability.md`, der dokumenterer `M:N` begrebssporing for klasser og attributter samt FDA-typeafgrænsning.
- [x] **Fuld Verifikation & Nul Regressionsfejl**:
  - Enhedstests for datamodellen, metoder og persistens.
  - 100% grøn test pass-rate (`cargo test --workspace`) og 0 clippy advarsler (`cargo clippy -- -D warnings`).

## 🚫 Must NOT
- Må IKKE fjerne, mutere eller forstyrre eksisterende begreber eller canvas-grafen, når klasser/attributter oprettes eller slettes (`Zero Mutation Side-Effects`).
- Må IKKE bryde deserialisering af eksisterende `model.edge.json` filer (`#[serde(default)]` invariant).
- Må IKKE tillade uautoriserede primitive datatyper uden for de 8 definerede FDA-typer.
- Må IKKE foretage remote publication (`git push`).

## 📝 Revisions
- 2026-09-19: Oprettet task 010 efter sparring og godkendelse af bruger.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
