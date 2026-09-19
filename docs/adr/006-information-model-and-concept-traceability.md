---
type: Architectural Decision Record
title: 'ADR 006: Informationsmodel, Begrebssporing og FDA Primitive Datatyper'
status: accepted
tags: [architecture, adr, information-model, uml, classes, attributes, traceability, fda]
---

# 6. Informationsmodel, Begrebssporing og FDA Primitive Datatyper

**Status**: `accepted`  
**Date**: `2026-09-19`  

## Context
Ifølge Digitaliseringsstyrelsens regler for begrebs- og datamodellering (FDA Modelreglerne v2.1) repræsenterer Informationsmodellen Trin 3 i den metodiske progression efter Begrebslisten (Trin 1) og Begrebsmodellen (Trin 2).

Informationsmodellen operationaliserer de forretningsmæssige begreber til konkrete datastrukturer (UML-klasser) med attributter, primitive standard datatyper og multipliciteter.

Der opstår her et centralt arkitektonisk spørgsmål om koblingen mellem begreber og informationsmodellen:
1. Skal en klasse i informationsmodellen altid svare 1:1 til et begreb, eller kan den oprettes selvstændigt?
2. Kan en klasse relatere sig til flere begreber (f.eks. en klasse der samler eller afgrænser flere begreber)?
3. Kan en attribut i en klasse have en direkte sporbarhedsrelation til et begreb (f.eks. attributten `cprNummer` knyttet til begrebet *CPR-nummer*)?
4. Hvilke datatyper og multiplicitetsformater er autoritative?

## Decision
1. **Afkoblet Domænemodel med Eksplicit Sporbarhed (M:N Traceability)**:
   - En `InformationClass` kan oprettes selvstændigt uden kildebegreb (`concept_ids` er tom).
   - En `InformationClass` kan knyttes til ét eller flere begreber (`concept_ids: Vec<Uuid>`).
   - Hver `InformationClass` tildeles sit eget stabile `id: Uuid`, et navn (`name: String`), en valgfri beskrivelse (`description: Option<String>`), og en liste af attributter (`attributes: Vec<Attribute>`).

2. **Attribut-niveau Begrebssporing**:
   - Hver `Attribute` har sit eget unikke `id: Uuid` og kan ligeledes referere til 0, 1 eller flere begreber (`concept_ids: Vec<Uuid>`).
   - Attributnavne følger FDA Modelreglernes §6.3 konvention (lowerCamelCase).

3. **Autoritative FDA Primitive Datatyper**:
   - Kun de 8 standardiserede FDA primitive datatyper tillades jf. FDA Modelreglernes standard:
     - `CharacterString`
     - `Integer`
     - `Decimal`
     - `Boolean`
     - `Date`
     - `DateTime`
     - `Time`
     - `URI`

4. **Multiplicitets-semantik**:
   - `Multiplicity` understøtter standard multipliciteter: `1` (`exactly_one`), `0..1` (`zero_or_one`), `0..*` (`zero_or_more`), `1..*` (`one_or_more`), samt arbitrære `lower..upper` grænser.

5. **Container & Bagudkompatibilitet**:
   - `InformationModel` etableres i `src/features/information_model/` og integreres i `ModelProject`.
   - Serialisering i `model.edge.json` anvender `#[serde(default)]`, så eksisterende modelprojekter uden informationsmodel indlæses uden fejl.

## Consequences
- **Positive**:
  - Fuld overensstemmelse med FDA Modelreglernes krav om metodisk progression og sporbarhed (traceability) mellem terminologi og datastruktur.
  - Høj fleksibilitet: Udviklere/modellerere kan frit modellere tekniske datastrukturer, koble klasser direkte til begreber, eller koble enkelte attributter til specifikke begreber.
  - Robust persistens uden brud på eksisterende projektfiler.
- **Negative / Opmærksomhedspunkter**:
  - Når et begreb slettes i begrebslisten, skal informationsmodellen håndtere eller advare om forældede `concept_id` referencer (eller rydde dem op via synkronisering), uden at slette selve klassen eller attributten.
