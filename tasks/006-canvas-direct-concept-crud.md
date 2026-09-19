---
type: Task Package
title: "Task 006: Direkte Begrebsoprettelse & Node-redigering på Canvas"
description: "Dobbeltklik på tomt canvas til lynoprettelse af begreb under markøren samt hurtigredigering af noder uden fane-skift"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T15:15:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, concept-crud, fda-model]
---

# Task 006: Direkte Begrebsoprettelse & Node-redigering på Canvas

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Gøre det muligt for brugeren at modellere begreber direkte i diagrammet under Fane 3 ("3. Begrebsmodel (Graf)") uden at skulle forlade canvaset for at gå til Begrebslisten. Dobbeltklik på et tomt område af lærredet åbner en fokuseret lynoprettelses-dialog (overlejret under eller ved markøren) for foretrukken term og definition jf. FDA Modelreglerne, hvorefter begrebet automatisk oprettes og noden placeres præcis på klikpositionen. Desuden skal dobbeltklik på en eksisterende node aktivere hurtigredigering af begrebets data.

## 📋 Acceptance Criteria
- [x] `GraphCanvas` registrerer dobbeltklik-hændelser på lærredet (`canvas::event::Event`) og skelner mellem klik på en eksisterende node og klik på en tom baggrund.
- [x] Dobbeltklik på en tom baggrund åbner en dedikeret lynoprettelses-modal ("Nyt Begreb på Lærred") med inputfelter for Foretrukken term (autofokuseret), Definition (Aristoteles' formel) og valg af lokal/indlånt tilknytning.
- [x] Ved bekræftelse (`Enter` eller "Opret") oprettes begrebet i `project.concepts()` med fuld `ConceptValidator`-validering, og en tilhørende grafnode placeres på det præcise klik-koordinat `(x, y)` og markeres straks.
- [x] Dobbeltklik på en eksisterende grafnode åbner en fokuseret inline-redigering af nodens foretrukne term og definition direkte på canvaset (eller i sidepanelet uden faneskift).
- [x] Autosave udløses automatisk efter oprettelse eller redigering, så persistensfilen altid er synkroniseret.
- [x] 100% test pass rate på unit-, accept- og proptests samt clippy med 0 advarsler.

## 🚫 Must NOT
- Må IKKE tillade oprettelse af begreber uden påkrævede FDA-felter (ugyldig/tom definition eller tom term).
- Må IKKE bryde eksisterende enkeltklik-adfærd (enkeltklik vælger/fravælger og trækker noder).
- Må IKKE navigere væk fra Begrebsmodel-fanen under lynoprettelse eller redigering.
- Må IKKE foretage remote publication handlinger (`git push`).

## 📝 Revisions
- 2026-09-19: Task oprettet for direkte begrebsoprettelse og node-redigering på canvas.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `xgauntlet check-spec -t 006-canvas-direct-concept-crud`
- `xgauntlet verify --task 006-canvas-direct-concept-crud`
