---
type: Task Package
title: "Task 011: UML Klassediagram Canvas og Canvas Studio Paradigme"
description: "Transformation af Fane 4 til et interaktivt UML Klassediagram Canvas med 3-sektions klassekasser, ortogonal routing, og 3-delt Studio-layout (Venstre: Palet, Center: Canvas, Højre: Context Inspector)"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T18:34:00Z" }
tags: [task-lifecycle, intent, information-model, uml, canvas, studio-layout, class-graph, drag-and-drop]
---

# Task 011: UML Klassediagram Canvas og Canvas Studio Paradigme

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Løfte Informationsmodellen (Fane 4) fra den statiske formularflade til et professionelt, interaktivt **UML Klassediagram Canvas** baseret på det ensartede **Canvas Studio Paradigme**:
1. **Venstre Palet (Repository Browser)**: Slank oversigt over alle projektets klasser, søgning, placerings-status på canvas (`[✓]` / `[+]`), samt hurtig oprettelse af nye klasser eller oprettelse direkte ud fra begreber.
2. **Center Canvas (UML Klassediagram)**: Hovedarbejdsområde med pan, zoom, grid, snap, ortogonal Manhattan-routing, og 3-sektions UML klassekasser (`«Klasse»` / klassenavn, delelinje, og attributlinjer `+ name : Type [multiplicitet]`) med automatisk dynamisk højde.
3. **Højre Inspector (Context Panel)**: Kontekstafhængig sidebar, som ved valg af en klasse muliggør hurtig redigering af navn, beskrivelse, tilknyttede begreber samt inline administration af klassens attributter (tilføj, rediger, slet) i realtid med øjeblikkelig opdatering af UML-kassen på canvas.
4. **Persistens**: Diagrammets layout (noder, koordinater og relationer) persisteres i `model.edge.json` via en dedikeret `information_graph: ClassGraph` med fuld bagudkompatibilitet.

## 📋 Acceptance Criteria
- [x] **Kerne-grafmodel for Informationsmodellen (`ClassGraph`)**:
  - `ClassDiagramNode` med `id: NodeId`, `class_id: Uuid`, `x: f32`, `y: f32`, `width: f32`, `height: f32`.
  - Dynamisk beregning af højde baseret på antallet af attributter (min 80px, f.eks. + 20px pr. attribut).
  - `ClassDiagramEdge` med `from: NodeId`, `to: NodeId`, `kind: RelationKind`, `label: Option<String>`.
  - `ClassGraph` container med metoder til at tilføje, fjerne, flytte noder og håndtere relationer.
- [x] **ModelProject & Disk-Persistens**:
  - `ModelProject` indeholder `#[serde(default)] information_graph: ClassGraph`.
  - Sikrer fail-closed integritet: Sletning af en klasse kaskadesletter tilhørende node og relationer.
- [x] **UML Canvas Rendering (`src/ui/information_canvas.rs`)**:
  - 3-sektions UML klassekasser jf. FDA Modelreglerne (FDA Sand `#FEFAF7`, FDA Blå for indlånte).
  - Centreret stereotype `«Klasse»` og klassenavn med fed skrift.
  - Horisontal skillelinje.
  - Venstrestillede attributter formateret som `+ name : Type [multiplicitet]`.
  - Ortogonal Manhattan routing med pile for Associationer og Generaliseringer.
- [x] **Canvas Studio UI i Fane 4 (`src/ui/information_model_view.rs`)**:
  - 3-delt opbygning: Venstre Palet (~240px), Center Canvas (Fill), Højre Inspector (~300px).
  - Venstre palet viser hvilke klasser der er på canvas (`[✓]`) vs ikke på canvas (`[+]`).
  - Højre inspector viser og redigerer den valgte klasses navn, begrebstilknytning og inline attributliste.
- [x] **ADR 007**:
  - Oprettelse af `docs/adr/007-canvas-studio-paradigm-and-uml-class-canvas.md`.
- [x] **Fuld Verifikation & Nul Regressionsfejl**:
  - Grønne enhedstests og acceptancetests i `tests/acceptance.rs`.
  - 100% grøn test pass-rate (`cargo test --workspace`) og 0 clippy advarsler (`cargo clippy -- -D warnings`).

## 🚫 Must NOT
- Må IKKE fjerne eller forstyrre Begrebsmodellens eksisterende canvas (`ConceptGraph`) eller begrebslisten.
- Må IKKE bryde deserialisering af eksisterende `model.edge.json` filer (`#[serde(default)]` invariant).
- Må IKKE introducere `cosmic-text` panic (aldrig bruge `.size(0)`).
- Må IKKE foretage remote publication (`git push`) eller destruktive git resets.

## 📝 Revisions
- 2026-09-19: Oprettet task 011 efter bruger-sparring og godkendelse af implementeringsplan.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
