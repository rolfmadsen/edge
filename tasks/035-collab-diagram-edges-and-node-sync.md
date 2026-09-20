# Task 035: Synkronisering af relationer, klasser og diagram-noder i realtime kollaboration

**Status**: `DONE`  
**Intent**: 🐛 `BUG FIX`  
**Dato**: `2026-09-20`  
**Scope**: `src/features/collab/protocol.rs`, `src/ui/app.rs`, `tests/acceptance.rs`  

---

## 🎯 Formål
Retter synkroniseringsfejl opdaget under to-maskiners live-test mellem Vært og Gæst over `kant-relay`:
1. **Begrebsmodel kanter/relationer**: Drag-to-connect (`GraphEdgeCreated`), modal relationer (`GraphCreateRelation`), redigering af art (`GraphUpdateEdgeKind`), etiketter (`GraphUpdateEdgeLabel`), retning (`GraphToggleEdgeDirected`), vending (`GraphReverseEdge`) og sletning (`GraphDeleteSelected`, `GraphDeleteRelation`) udsendes og synkroniseres pålideligt til gæstemaskinen.
2. **Informationsmodel klasser & attributter**: Tilføjelse, sletning og redigering af informationsklasser og deres attributter synkroniseres, og nodestørrelser opdateres automatisk på alle tilsluttede klienter.
3. **Informationsmodel kanter & relationer**: Forbindelser og relationer mellem klasser (`ClassRelationAdded`, `ClassRelationDeleted`) synkroniseres, og ortogonale ruter genberegnes på modtageren.
4. **Flytning af diagram-noder**: Flytning af noder i informationsmodellen synkroniseres ved at referere klassens UUID i stedet for tilfældige lokale diagram-node ID'er.
5. **Projektbeskyttelse mod overskrivning**: Værten afviser indkommende snapshots fra relay-serverens cache, så værtens aktive model aldrig utilsigtet nulstilles.

---

## 📋 Acceptance Criteria
- [x] **AC1 - Begrebsmodel relationer synkroniseres**: `GraphEdgeCreated` og andre relationshandlinger udsender `ModelMutation::RelationAdded`, `RelationUpdated` eller `RelationDeleted { from, to }`. Modtageren indsætter relationen og ruter kanterne.
- [x] **AC2 - Informationsmodel relationer synkroniseres**: Oprettelse og sletning af relationer mellem klasser transmitterer `ClassRelationAdded` og `ClassRelationDeleted` og ruter kanterne på modtageren.
- [x] **AC3 - Informationsmodel nodeflytning synkroniseres**: `NodeMoved` matcher både begrebsnoder og klassenoder via deres domæne-UUID.
- [x] **AC4 - Host Snapshot Guard**: `apply_snapshot` ignorerer snapshots hvis den lokale rolle er `Host`.
- [x] **AC5 - Verifikation**: Fuld accepttest-suite inklusiv E2E kollaboration over efemerisk relay passerer fejlfrit.

---

## 🚫 Must NOT
- Værtens projekt må ALDRIG overskrives af indkommende snapshots.
- Relationen mellem to noder må ikke slette uvedkommende relationer tilknyttet noderne.
- Klienter må ikke antage identiske lokale node-ID'er på tværs af maskiner for klassediagrammer.

---

## 🧪 Verifikation
- `cargo test --test acceptance test_task034_collab_edge_and_diagram_sync_lifecycle`
- `cargo test --test acceptance test_task029_e2e_collab_sync_and_presence`
- `cargo test --workspace`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
