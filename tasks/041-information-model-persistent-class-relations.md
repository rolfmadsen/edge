---
type: Task Package
title: "Task 041: Bevarelse af Klasserelationer og Associationer ved Fjernelse fra Canvas"
description: "Sikring af at relationer mellem klasser i informationsmodellen bevares semantisk og genoprettes automatisk på lærredet, hvis en klasse fjernes og senere genindsættes"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-21T21:30:00Z" }
tags: [information-model, relations, associations, canvas, persistence, semantics]
---

# Task 041: Bevarelse af Klasserelationer og Associationer ved Fjernelse fra Canvas

**Status**: `ACTIVE`  
**Intent**: 🔄 `ENHANCEMENT`  
**Oprettet**: `2026-09-21`  
**Scope**: `src/features/information_model/mod.rs`, `src/features/model/mod.rs`, `src/ui/app.rs`, `src/ui/information_model_view.rs`, `tests/acceptance.rs`

---

## 🎯 Formål
I henhold til UML og FDA-retningslinjer for informationsmodellering er relationer (associationer, generaliseringer og kompositioner) semantiske strukturer mellem klasser (`InformationClass`), ikke blot flygtige geometriske linjer på et specifikt diagram:

1. **Problem**:
   - I øjeblikket slettes alle tilstødende kanter (`edges.retain(|e| e.from() != node_id && e.to() != node_id)`) permanent fra modellen, når brugeren klikker "Fjern fra diagram" for en klasse.
   - Hvis brugeren efterfølgende tilføjer klassen til lærredet igen fra paletten, er samtlige relationer og opsatte multipliciteter gået tabt.
2. **Løsning (Persistente Klasserelationer)**:
   - Skil **visuel diagrampræsentation** (hvilke klasser er placeret på lærredet som `ClassDiagramNode`) fra **semantiske modelrelationer** (hvilke relationer eksisterer mellem klasser i informationsmodellen).
   - Når en klasse fjernes fra lærredet (`RemoveClassFromDiagram`), fjernes dens diagramnode fra lærredet, men selve relationen bevares (enten semantisk i `InformationModel` eller som hvilende/klassebaserede kanter).
   - Når en klasse genindsættes på lærredet, gendannes/synliggøres kanterne automatisk mellem de klasser, der nu er på lærredet, komplet med relationstype, multipliciteter, retningspile og labels.
   - En relation slettes KUN permanent, hvis brugeren eksplicit vælger at slette selve relationen (fx via slet-knap i relationsoversigt/inspektør eller slet-tast på valgt relation), eller hvis selve klassen slettes permanent fra modellen (`remove_class`).

---

## 📋 Acceptance Criteria
- [ ] **AC1 - Relationer bevares ved skjul/fjern fra lærred**: Når `RemoveClassFromDiagram(node_id)` udløses, bevares relationens data (type, multipliciteter, retningspil og label) i modellen.
- [ ] **AC2 - Automatisk genopståen på Canvas**: Når en tidligere fjernet klasse genindsættes på lærredet (via paletten "Tilføj til diagram"), synliggøres relationer automatisk for alle modstående klasser, der også befinder sig på lærredet.
- [ ] **AC3 - Eksplicit sletning af relation**: Brugeren kan fortsat slette en relation permanent, når det ønskes (uden at skulle fjerne klassen fra modellen).
- [ ] **AC4 - Sletning af klasse kaskaderer**: Hvis en klasse slettes permanent fra projektet (`DeleteInformationClass`), opryddes dens tilhørende relationer permanent.
- [ ] **AC5 - Bagudkompatibilitet**: Eksisterende `.kant.json` projektfiler indlæses uden fejl eller tab af data.
- [ ] **AC6 - Verifikation via Accepttest**: `test_task_041_persistent_class_relations_across_canvas_removal` beviser at relationer overlever fjernelse og genindsættelse på lærredet.

---

## 🚫 Must NOT
- Må IKKE slette relationer permanent, når en klasse blot fjernes fra diagrammet.
- Må IKKE bryde JSON-serialisering eller model-skemaet for eksisterende FDA-projekter.
- Må IKKE efterlade forældreløse relationer (dangling edges), hvis en klasse slettes permanent fra modellen.

---

## 📝 Revisions
- 2026-09-21: Oprettet opgavepakke efter sparring med brugeren.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_041`
- `cargo clippy --all-targets`
- `cargo fmt --check`
