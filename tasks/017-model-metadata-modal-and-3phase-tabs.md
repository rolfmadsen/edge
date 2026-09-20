---
type: Task Package
title: "Task 017: Model Omslag & Metadata Redigeringsmodal og 3-Faset Navigation"
description: "Fjernelse af Omslag & Metadata som selvstændig fane til fordel for en 3-faset arbejdsgang, samt implementering af en komplet redigeringsmodal til modelomslagets metadata"
status: pending
generated: { by: process:antigravity-task-init, at: "2026-09-20T09:55:00Z" }
tags: [ui, metadata, model-cover, tabs, fda-phases, modal]
---

# Task 017: Model Omslag & Metadata Redigeringsmodal og 3-Faset Navigation

**Status**: `PENDING`
**Intent**: `🔄 ENHANCEMENT`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Reorganisere hovednavigationen så den afspejler FDA Modelreglernes 3 faser uden en forstyrrende metadata-fane:
   - `1. Begrebsliste (Bilag D & E)`
   - `2. Begrebsmodel (Graf)`
   - `3. Informationsmodel`
2. Etablere en dedikeret knap i top-headeren (`📋 Modelomslag` / `⚙️ Modelmetadata`) ved siden af modeltitlen, som åbner en modal dialog til redigering af modelprojektets omslag og metadata.
3. Gøre alle metadatafelter redigerbare i modalen:
   - Modelnavn (tekstfelt)
   - Beskrivelse (flere linjers tekstfelt)
   - Status (`ModelStatus`: Draft, Candidate, Approved via dropdown/picklist)
   - Emneområde (§26) (tekstfelt)
   - Ansvarlig organisation/myndighed (tekstfelt)
   - Model-URI (tekstfelt)
   - Version (tekstfelt)
4. Sikre at ændringer gemmes direkte i `ModelMetadata` og trigger markering af ikke-gemte ændringer (`SaveStatus::Unsaved`).

## 📋 Acceptance Criteria
- [ ] Top-fanebaren indeholder præcis 3 faner: Begrebsliste, Begrebsmodel og Informationsmodel.
- [ ] `Tab::Metadata` er fjernet fra fanelinjen, og default aktiv fane ved opstart eller nyt projekt er `Tab::ConceptList`.
- [ ] En knap i headeren åbner `ModelMetadataModal`.
- [ ] Modalen indeholder formularfelter for samtlige metadatafelter (navn, beskrivelse, status, emneområde, ansvarlig myndighed, URI, version).
- [ ] Gem-knap i modalen opdaterer `project.metadata` og sætter applikationen i unsaved-status.
- [ ] Annuller/Luk knapper og Escape-tast lukker modalen uden at gemme utilsigtede ændringer.
- [ ] Enhedstests validerer korrekt opdatering og serialisering af de redigerede metadatafelter.
- [ ] 100% test pass rate på `cargo test --workspace` og clippy uden advarsler.

## 🚫 Must NOT
- Må IKKE fjerne eller forringe felter defineret i `ModelMetadata`.
- Må IKKE bryde bagudkompatibilitet for eksisterende `model.edge.json` filer.
- Må IKKE foretage remote git push.

## 📝 Revisions
- 2026-09-20: Oprettet som led i vertikal opsplitning af sparringspunkterne.

## 🧪 Verifikation
- `cargo test test_metadata`
- `cargo clippy -- -D warnings`
- Visuel test af modalen og 3-faset navigation.
