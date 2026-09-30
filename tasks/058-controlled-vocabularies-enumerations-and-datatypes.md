---
type: Task Package
title: "Task 058: Controlled Vocabularies: Enumerations & Structured Datatypes"
description: "Understøttelse af kontrollerede udfaldsrum med grønne enumerationer og gule strukturerede datatyper samt reference fra attributter"
status: active
generated: { by: process:antigravity-task-init, at: "2026-09-30T21:46:00Z" }
tags: [fda, enumeration, datatypes, controlled-vocabularies, information-model, canvas]
---

# Task 058: Controlled Vocabularies: Enumerations & Structured Datatypes

**Status**: `ACTIVE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-30`

## 🎯 Formål
Implementere kontrollerede udfaldsrum i Informationsmodellen i overensstemmelse med FDA Modelreglerne (Kapitel 5.2, 5.5, Regler 15, 27 og 28):
1. **Enumerationer (Grønne Kasser `#E8FDE3`)**:
   - Struktur for `InformationEnumeration` indeholdende `id: Uuid`, `name: String`, `values: Vec<String>` og metadata.
   - Visuel boks på lærredet med keyword `«enumeration»`, grøn baggrund (`ThemeColors::FDA_ENUM_GREEN`), og værdier i `lowerCamelCase` jf. Tabel B.
2. **Strukturerede Datatyper (Gule Kasser `#FBF9C6`)**:
   - Struktur for `StructuredDataType` indeholdende egne under-attributter.
   - Visuel boks på lærredet med keyword `«dataType»`, gul baggrund (`ThemeColors::FDA_STRUCTURED_YELLOW`).
3. **Attribut-kobling & Dependency-relationer**:
   - Udvide `Attribute.data_type` fra ren `PrimitiveType` til `InformationDataType { Primitive(PrimitiveType), Enumeration(Uuid), Structured(Uuid) }`.
   - Vise stiplet dependency-pil på lærredet mellem klasse/attribut og den pågældende enumeration eller strukturerede datatype jf. Kapitel 5.5.
4. **Valg af Primitivt Datatypesystem (Regel 27)**:
   - Mulighed for at konfigurere modellen til enten ISO/TC 211 eller XSD/RDFS primitive datatyper (og forhindre sammenblanding).

## 📋 Acceptance Criteria
- [ ] **AC1 - Datamodeller for Enumeration & StructuredType**: `InformationModel` udvides med opbevaring af enumerationer og strukturerede datatyper.
- [ ] **AC2 - Attribut Reference**: Attributter kan vælge mellem primitive typer, oprettede enumerationer og strukturerede datatyper som udfaldsrum.
- [ ] **AC3 - Canvas Rendering med Korrekte Farver**:
   - Enumerationer tegnes i `ThemeColors::FDA_ENUM_GREEN` med `«enumeration»`.
   - Strukturerede datatyper tegnes i `ThemeColors::FDA_STRUCTURED_YELLOW` med `«dataType»`.
- [ ] **AC4 - Dependency Linjer**: Visuel rendering af stiplede pile med åbent pilehoved mellem klasser/attributter og deres refererede typer.
- [ ] **AC5 - Persistens & Merge**: Enumerationer og strukturerede typer serialiseres deterministisk i både `.kant.json` og dekomponeret `.kant/` format.

## 🚫 Must NOT
- Må IKKE bryde eksisterende attributter med primitive typer.
- Må IKKE tillade cirkulære afhængigheder i strukturerede datatyper.

## 📝 Revisions
- 2026-09-30: Oprettet opgavepakke efter analyse af FDA Kapitel 5.5.

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets`
