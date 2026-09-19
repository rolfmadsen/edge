---
type: Knowledge Bundle Index
title: "edge Context & Domain Glossary"
description: "Kernebegreber, arkitekturgrænser og definitioner for edge"
status: stable
generated: { by: process:xgauntlet-init, at: "2026-09-06T18:00:00Z" }
tags: [glossary, domain-model, ubiquitous-language, okf]
---

# edge Context & Domain Glossary

This document defines the core ubiquitous language for `edge` using Aristotle's formula (*definitio per genus et differentiam*). It captures domain concepts without implementation noise.

---

## 📖 Core Concepts

**Task**:
An executable unit of engineering work, that has bounded acceptance criteria and verifiable completion evidence.
_Avoid_: Ticket, issue, story, workitem.

**Layer**:
A verification step, that executes a specific analysis or testing command within a bounded timeout.
_Avoid_: Stage, phase, check-item.

**Gauntlet**:
A sequential verification pipeline, that executes verification layers with fail-closed semantics and halts on the first mandatory failure.
_Avoid_: Test runner, CI script, harness.

**Canonical Workspace Manifest**:
A deterministic SHA-256 digest, that captures normalized Git-tree/blob OID hashes and raw file contents across in-scope workspace paths.
_Avoid_: Git commit, workspace hash, checksum.

**Verification Report**:
An unsigned data record, that binds verification layer outcomes, diagnostic findings, and task contracts to the workspace manifest digests.
_Avoid_: Proof report, receipt, certification.

---

## 🏛️ FDA Domain Concepts (Fællesoffentlig Digital Arkitektur)

**Model (FDA Model)**:
En struktureret repræsentation af et forretnings- eller dataområde, der overholder de fællesoffentlige modelregler for dokumentation, versionering og identifikation.
_Avoid_: Diagramfil, tegning, skitse.

**Begreb (Concept)**:
En mental enhed eller abstraktion, der sammenfatter fælles karakteristika for en mængde af fænomener i virkeligheden, og som identificeres ved en stabil HTTP-URI.
_Avoid_: Dataelement, entitet (når der menes begreb).

**Term**:
En sproglig betegnelse i naturligt sprog for et givet begreb inden for et domæne.
_Avoid_: Variabelnavn, feltnavn.

**Foretrukken term**:
Den sproglige betegnelse, som en myndighed eller et fagligt fællesskab officielt har fastlagt som den primære repræsentant for et begreb.
_Avoid_: Navn, overskrift.

**Struktureret definition**:
En tekstuel forklaring af et begrebs betydning, udformet efter Aristoteles' formel med angivelse af et overordnet begreb (*genus proximum*) og en eller flere adskillende egenskaber (*differentia specifica*).
_Avoid_: Cirkulær definition, opremsende definition, negativ definition.

**Begrebsliste**:
En tabelformet repræsentation af en begrebsmodel, der opstiller begreber med de 12 standardiserede metadatafelter jf. FDA Modelreglerne Bilag D og E.
_Avoid_: Ordliste, begrebskatalog uden standardfelter.

**Begrebsmodel**:
En terminologisk model, der visualiserer begreber og deres indbyrdes semantiske relationer (generaliseringer, generaliseringssæt og associationer) uafhængigt af datastrukturer.
_Avoid_: Datamodel, entitetsdiagram.

**Informationsmodel**:
En UML-baseret model, der beriger begreber med forretningsregler, klassestrukturer, attributter med standard primitive datatyper og multipliciteter.
_Avoid_: Fysisk databasemodel, skema.

