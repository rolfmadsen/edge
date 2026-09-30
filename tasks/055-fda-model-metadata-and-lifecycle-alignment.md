---
type: Task Package
title: "Task 055: FDA Modelmetadata & Lifecycle Alignment"
description: "Harmonisering af modelmetadata med FDA Tabel D og E, adskillelse af modelStatus og approvalStatus samt bagudkompatibel migration"
status: done
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, metadata, lifecycle, approval, tabel-d, model-project]
---

# Task 055: FDA Modelmetadata & Lifecycle Alignment

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Bringe projektets overordnede metadata i 100% overensstemmelse med De fællesoffentlige regler for begrebs- og datamodellering (FDA v2.1.0, Kapitel 6, Tabel D og E):
1. **Adskillelse af Modelstatus og Godkendelsesstatus**:
   - `model_status`: `Development` (Under udvikling), `Completed` (Endelig), `Deprecated` (Forældet), `Withdrawn` (Trukket tilbage) jf. Regel 12.
   - `approval_status`: `AwaitingApproval` (Afventer godkendelse), `Approved` (Godkendt), `ApprovedWithRemarks` (Godkendt med bemærkninger), `NotRelevant` (Ikke relevant) jf. Regel 11.
2. **Supplerende FDA Metadatafelter (Tabel D)**:
   - `is_approved_by`: `Option<String>` (Forum for godkendelse).
   - `model_scope`: `ModelScope` (`Core` / Kernemodel vs. `ApplicationProfile` / Anvendelsesmodel) jf. Regel 25.
   - `language`: `String` (Modellens primære sprog, f.eks. "da" jf. RFC5646).
   - `date_modified`: `String` (Seneste opdateringsdato i ISO-8601 format YYYY-MM-DD).
   - `version_notes`: `Option<String>` (Ændringshistorik).
   - `source`: `Option<String>` (Reference til standarder eller kilder på modelniveau).
   - `was_derived_from`: `Option<String>` (Afledt af model-URI) jf. Regel 14.
   - `legal_sources`: `Vec<String>` (Lovgrundlag med understøttelse af multiple ELI-referencer) jf. Regel 13.
3. **Namespace URI Validering & Vejledning**:
   - Validering af namespace URI mod FDA-mønstret `https://data.gov.dk/{type}/{scope}/{reference}` jf. Kapitel 4.
4. **UI & Bagudkompatibilitet**:
   - Opdatere metadata-redigeringsmodalen i Kant til at fremvise og redigere alle felter struktureret.
   - Fuld bagudkompatibel Serde-deserialisering for eksisterende `model.kant.json` og `.kant/metadata.json` filer.

## 📋 Acceptance Criteria
- [x] **AC1 - Domænemodel & Tabel D Felter**: `ModelMetadata` indeholder samtlige obligatoriske og anbefalede felter fra FDA Tabel D.
- [x] **AC2 - Status Enums Adskilt**: `ModelStatus` (livscyklus) og `ApprovalStatus` (forretningsgodkendelse) er adskilte typer med korrekte FDA-betegnelser og display strings.
- [x] **AC3 - Bagudkompatibel Migration**: Indlæsning af ældre projektfiler med legacy `status: "Draft" | "Candidate" | "Approved"` deserialiseres uden fejl til gyldige `model_status` og `approval_status`.
- [x] **AC4 - UI Metadata Modal**: UI modalen (`EditMetadata`) lader brugeren vælge modelStatus, approvalStatus, modelScope, language og redigere godkendende forum samt kilder.
- [x] **AC5 - Verificeret Serde & Headless Tests**: Unit- og acceptancetests beviser roundtrip serialisering i både enkeltfil og dekomponeret format.

## 🚫 Must NOT
- Zero-Daemon invariant: Må IKKE introducere baggrundsprocesser.
- Må IKKE bryde eksisterende `.kant` kataloger eller tabe brugerdata under deserialisering.
- Må IKKE bryde headless tests (`cargo test --workspace`).

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke efter gennemgang af fda_modelleringsvejledning.md Kapitel 6.
- 2026-09-30: Fuldført implementering, bagudkompatibel Serde-migrering, fuld Tabel D modal UI og acceptancetests. Markeret DONE.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
- `cargo fmt --check`
