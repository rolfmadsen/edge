# Composite Coding Standards: edge

Dette dokument fastlægger de tværgående kodestandarder og håndværksmæssige principper for `edge` (primær stack: `rust`).

---

## 🏛️ Transversale Arkitektur- & Kvalitetsinvarianter

1. **Package-by-Feature (Screaming Architecture)**:
   - Al kode organiseres i autonome feature-moduler, der indkapsler forretningslogik, modeller og lokale tests.
   - Undgå flade kataloger med tekniske lag (`models/`, `views/`, `controllers/`).

2. **Test-Driven Development (Red-Green-Refactor)**:
   - Skriv altid fejlede accept- eller enhedstests først (RED).
   - Implementér den minimale kode, der gør testen grøn (GREEN).
   - Refaktorér med bevaret adfærd under fuld testdækning (REFACTOR).

3. **Nul Compiler- & Linter-Advarsler**:
   - Kodebasen skal til enhver tid kompilere og lintes med 0 advarsler under `-D warnings` / tilsvarende flags.

4. **Fejlhåndtering & Fail-Closed**:
   - Håndter alle fejl eksplicit via type-sikre resultater.
   - Slug aldrig exceptions eller fejl uden struktureret rapportering.
