# Task 034: Bundling af relationer efter type og retning (Port-Bus)

**Status**: `DONE`  
**Intent**: 🐛 `BUG FIX` / 🔄 `ENHANCEMENT`  
**Dato**: `2026-09-20`  
**Scope**: `src/ui/edge_router.rs`, `tests/acceptance.rs`  

---

## 🎯 Formål
I henhold til ADR 005 og FDA Modelreglerne (kapitel 5 & 7, bl.a. Fig 7.1) skal relationer af samme type på samme side af en node samles i et fælles anker/stamme ("bundling"):
- Flere indgående relationer af samme type til samme nodeside (f.eks. generaliseringer der rammer undersiden af en superklasse) skal have identisk port-offset og dermed dele et fælles pilehoved og vertikal/horisontal stamme.
- Flere udgående relationer af samme type fra samme nodeside skal ligeledes have identisk kildeslot-offset og udgå fra samme punkt.
- Hvis der optræder relationer af forskellige typer eller forskellige retninger på samme side (f.eks. indgående generaliseringer kombineret med en udgående komposition), skal hver gruppe `(RelationKind, is_source)` have sit eget adskilte slot. Disse grupper skal sorteres spatielt (spatial sorting) baseret på modstående noders koordinater, så linjeforløbene forbliver parallelle og uden unødige krydsninger.

---

## 📋 Acceptance Criteria
- [x] **1. Bundling af indgående relationer af samme type**: Flere indgående relationer af samme type på samme side (f.eks. to generaliseringer mod en superklasse) tildeles præcist samme port-slot (`to_slot_offset`), uanset om der også findes andre relationstyper på samme side.
- [x] **2. Bundling af udgående relationer af samme type**: Flere udgående relationer af samme type fra samme side tildeles præcist samme kildeslot (`from_slot_offset`).
- [x] **3. Spatiel separation af distinkte grupper**: Grupper med forskellig `(RelationKind, is_source)` på samme side tildeles symmetriske, adskilte slot-offsets (`SLOT_SPACING = 24.0px`), sorteret rumligt efter modstående noders gennemsnitlige position, så parallelle relationer ikke krydser unødigt.
- [x] **4. Bevarelse af eksisterende routing-regler**: Eksisterende tests for ortogonal routing, hysterese og ikke-krydsende linjer forbliver grønne.

---

## 🚫 Must NOT
- Relationer af forskellig type (eller modsat retning) må ALDRIG dele samme slot-offset.
- En tilføjelse af en relation af en anden type må ALDRIG ophæve bundlingen af eksisterende relationer af samme type på den pågældende side.
- Spatiel sortering må ikke introducere unødige linjekrydsninger.

---

## 🧪 Verifikation
- `cargo test test_relation_bundling_by_type_and_direction`
- `cargo test test_edges_do_not_cross_unnecessarily_when_sorted_vertically`
- `cargo test`
