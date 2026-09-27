---
type: Task Package
title: "Task 051: Model Repository Isolation and Git Provider Agnosticism"
description: "Isoler hvert FDA-modelprojekt i sin egen dedikerede mappe og Git-lager, beskyt mod softwarekataloger, gør Git-integrationen provider-agnostisk (GitLab/Gitea/GitHub), sikr ren nulstilling ved kloning samt automatisk navngivning baseret på repository-navn."
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-27T21:34:00Z" }
tags: [git, isolation, model-repository, provider-agnostic, coarchi, clone]
---

# Task 051: Model Repository Isolation and Git Provider Agnosticism

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-27`

## 🎯 Formål
Sikre at hvert modelprojekt i Kant er en isoleret enhed med sit eget dedikerede Git-lager, beskytte mod uønsket brug af overordnede software- eller systemkataloger som Git-rod, fjerne hardcoded GitHub-binding til fordel for generisk Git-understøttelse (GitLab, Gitea, Forgejo, intern Git), sikre at 'Klon model fra Git' åbner i et fuldstændig rent, nulstillet projekt uden lækage fra tidligere åbnede modeller, samt udlede projektets modelnavn direkte fra repository-navnet ved kloning.

## 📋 Acceptance Criteria
- [ ] AC1 (Model Directory Isolation): `repo_dir()` og GitService må ALDRIG falde tilbage til `"."` eller bruge et overordnet softwareprojekt (mapper med `Cargo.toml`, `package.json` osv. eller systemmapper som `~` og `/`). Hvis en model er ny/ugemt, er Git inaktiv eller kræver eksplicit modelmappe.
- [ ] AC2 (Nyt Projekt = Ren Tavle): "Nyt projekt" starter altid med ren tavle og uinitialiseret Git-status uden historik eller slettede elementer fra tidligere modeller.
- [ ] AC3 (Git-Hosting Agnosticisme): `build_authenticated_url` understøtter GitLab (`oauth2:<TOKEN>`), Gitea/Forgejo (`<TOKEN>`), GitHub (`x-access-token:<TOKEN>`), samt eksplicitte credentials (`<BRUGER>:<TOKEN>`). Alle UI-tekster og placeholders er neutrale.
- [ ] AC4 (Ren Kloning og Modelnavn): "Klon model fra Git" nulstiller hele applikationstilstanden (editor, noder, relationer) og indlæser den klonede model rent. Hvis fjernlageret er tomt, initialiseres en ren FDA-model automatisk. Projektets navn (`metadata.name`) sættes automatisk til repository-navnet, hvis det ikke allerede er defineret.

## 🚫 Must NOT
- Must NOT tillade at en model genbruger eller ændrer Git-lageret for Kant-applikationens eget kildekodelager (`edge`).
- Must NOT fjerne eller miste brugerens lokale modelarbejde.
- Must NOT hardcode GitHub som den eneste understøttede Git-platform.
- Must NOT merge eller lække data fra et tidligere åbent projekt over i et nyklonet projekt.

## 📝 Revisions
- 2026-09-27: Oprettet opgavepakke baseret på sparring med brugeren.

## 🧪 Verifikation
- Unit tests i `src/features/git/service.rs` for provider-agnostisk URL-opbygning, udledning af repo-navn fra URL, og sikkerhedsguardrails for modelmapper.
- Acceptance tests i `tests/` for "Nyt projekt" og "Klon model fra Git" tilstands-nulstilling.
- `cargo test --workspace` gauntlet.
