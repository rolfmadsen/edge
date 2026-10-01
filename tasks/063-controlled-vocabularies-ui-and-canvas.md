---
type: Task Package
title: "Task 063: Kontrollerede Udfaldsrum: UI & Canvas Integration for Enumerationer"
description: "Komplet UI- og lærredsunderstøttelse af enumerationer i informationsmodellen med grønne FDA-kasser, attribut-referencer og stiplede dependency-relationer"
status: done
generated: { by: process:antigravity-task-init, at: "2026-10-01T20:17:00Z" }
tags: [fda, information-model, enumeration, controlled-vocabularies, canvas, inspector, dependencies]
---

# Task 063: Kontrollerede Udfaldsrum: UI & Canvas Integration for Enumerationer

**Status**: `DONE`  
**Intent**: `🚀 NEW FEATURE`  
**Oprettet**: `2026-10-01`  
**Scope**: `src/features/information_model/`, `src/features/model/`, `src/ui/`, `tests/acceptance.rs`

---

## 🎯 Formål
Færdiggøre implementeringen af **Model A** (den officielle FDA-metode jf. Task 058 og Modelreglerne kapitel 5.2 & 5.5) ved at integrere kontrollerede udfaldsrum (`InformationEnumeration`) direkte i brugergrænsefladen og på UML-lærredet:

1. **Oprettelse & Redigering i Informationsmodel Studiet**:
   - Palette-/studiovisning i venstre side med sektion for kontrollerede udfaldsrum.
   - Oprettelse af enumerationer med `UpperCamelCase` navn, valgfri definition og værdier i `lowerCamelCase` jf. FDA Tabel B.
   - Redigering af enumerationsværdier i egenskabsinspektøren.

2. **Lærred (Canvas) Visualisering**:
   - Enumerationer kan placeres på diagrammet som diagramnoder.
   - Tegnes via `render_uml_enumeration_node` i officiel FDA grøn (`ThemeColors::FDA_ENUM_GREEN`, `#E8FDE3`), med keyword `«enumeration»`, navn og værdiliste.
   - Fuld støtte for markering, flytning, grid-snap og sletning/fjernelse fra diagrammet.

3. **Attribut-tilknytning & Stiplede Dependency-pile**:
   - Attribut-inspektørens typevælger udvides, så en klasses attribut kan vælge en oprettet enumeration som `InformationDataType::Enumeration`.
   - Når en klasse har en attribut med en enumeration som datatype, og begge elementer er placeret på lærredet, manifesteres en stiplet `Dependency`-relation med åbent pilehoved og label `«use»` jf. FDA Modelreglerne Kapitel 5.5.

---

## 📋 Acceptance Criteria
- [x] **AC1 - UI til Oprettelse og Værdiredigering**: Brugeren kan oprette en `InformationEnumeration` og tilføje/fjerne værdier (med validering til `lowerCamelCase` jf. FDA Tabel B) fra Informationsmodel-visningen.
- [x] **AC2 - Grønne Enumeration-kasser på Lærred**: Enumerationer kan tilføjes til lærredet og tegnes i `ThemeColors::FDA_ENUM_GREEN` (`#E8FDE3`) med `«enumeration»`, centreret overskrift, skillelinje og værdier.
- [x] **AC3 - Attribut Dropdown med Enumerationer**: Attribut-inspektøren tilbyder både primitive typer og modellens oprettede enumerationer.
- [x] **AC4 - Automatisk Dependency-relation på Canvas**: Ved valg af en enumeration for en klasses attribut synkroniseres en stiplet `Dependency`-relation til enumerationen på diagrammet jf. FDA Kapitel 5.5.
- [x] **AC5 - Persistens & Samarbejde**: Enumerationsnoder og -relationer persisteres deterministisk i `.kant.json` og dekomponeret lagring samt synkroniseres ved collab.
- [x] **AC6 - Verifikation via Accepttest**: `test_task_063_controlled_vocabulary_enumeration_ui_and_canvas` verificerer oprettelse, canvas-placering, attribut-kobling og dependency-rendering.

---

## 🚫 Must NOT
- Må IKKE tillade almindelige associationer eller kompositioner til/fra enumerationer (kun `Dependency` jf. FDA).
- Må IKKE tillade ugyldige enumerationsværdier med mellemrum eller forkert case (SKAL følge `lowerCamelCase`).
- Må IKKE bryde eksisterende modeller eller JSON-serialisering med primitive typer.

---

## 📝 Revisions
- 2026-10-01: Oprettet opgavepakke efter arkitekturafklaring af FDA Model A (kontrollerede udfaldsrum).

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task_063`
- `cargo clippy --all-targets`
- `cargo fmt --check`
