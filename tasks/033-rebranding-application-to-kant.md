# Task 033: Rebranding af applikation fra Edge til Kant

**Status**: `ACTIVE`  
**Intent**: 🔄 `REFACTOR`  
**Dato**: `2026-09-20`  
**Scope**: `Cargo.toml`, `src/main.rs`, `src/ui/`, `src/features/model/storage.rs`, `assets/`, `.github/workflows/`, `koyeb.yaml`, `README.md`, `tests/`  

---

## 🎯 Formål
Omdøbe applikationen fra "Edge" til "Kant" for at eliminere forveksling med Microsoft Edge browseren.
Omdøbningen omfatter kasser/crates (`kant`, `kant-relay`), binære navne, UI-tekster, standard filendelse (`*.kant.json`), distributionsfiler, desktop-ikoner og CI/CD workflows, mens interne grafteoretiske datastrukturer (`Node`/`Edge`) bevares på idiomatisk engelsk, og der sikres transparent bagudkompatibilitet for eksisterende `*.edge.json` model-filer.

---

## 📋 Acceptance Criteria
- [ ] **1. Crate- og binærnavn til Kant**: Rod-crate omdøbes til `name = "kant"` i `Cargo.toml`. Relay-kassen omdøbes til `kant-relay`. Binæren hedder `kant` (`kant.exe`).
- [ ] **2. UI & App-titel til Kant**: Skrivebordsapplikationens vinduestitel opdateres til `"Kant - Begrebs- og Informationsmodellering med FDA"`, og header logo-tekst opdateres til `"Kant"`.
- [ ] **3. Filendelser & Bagudkompatibilitet**: Standard gemmefil og lagringssti opdateres til `model.kant.json`. Native filvælgere og filscannere accepterer og lister både `*.kant.json` og `*.edge.json`.
- [ ] **4. OS Assets & Packaging**: `assets/kant.desktop` og `assets/icons/kant.svg` er oprettet og refereret i `Cargo.toml` (`[package.metadata.deb]`). GitHub Actions `release.yml` bygger og uploader `kant-linux-x86_64`, `kant-macos-aarch64` og `kant-windows-x86_64`.
- [ ] **5. Testsuite & Clang/Clippy Green**: Alle tests i `tests/acceptance.rs` og `tests/proptests.rs` kompilerer med `use kant::...` og passerer 100%.

---

## 🚫 Must NOT
- Må IKKE omdøbe interne grafteoretiske datastrukturer (`ClassDiagramEdge`, `DiagramEdge`, `EdgeRouter`, `edges: Vec<...>`) til et blandet sprogmiks.
- Må IKKE bryde evnen til at indlæse eksisterende `*.edge.json`-filer.
- Må IKKE efterlade ubrugte `edge`-referencer i release-pipeline eller desktop-definitioner.

---

## 📝 Revisions
- `2026-09-20`: Oprettet jf. sparring og godkendt implementeringsplan (ADR 009).

---

## 🧪 Verifikation
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy -- -D warnings`
