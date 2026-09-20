# Task 032: Konsistent Midterknæk og Parallel Kanal-Routing for Relationer

**Status**: `ACTIVE`  
**Intent**: 🐛 `BUG FIX` / 🔄 `ENHANCEMENT`  
**Dato**: `2026-09-20`  
**Scope**: `src/ui/edge_router.rs`, `tests/acceptance.rs`  

---

## 🎯 Formål
I FDA begrebs- og informationsmodeller (graf-canvas) skal ortogonale relationer mellem noder knække præcist i midten imellem nodernes ydre kant-afgrænsninger, uanset relationstype (Generalisering, Komposition, Association). Samtidig må relationer af forskellig type aldrig overlappe på parallelle strækninger (collinear overlap), men skal forsynes med parallelle kanaler (`CHANNEL_OFFSET = 14.0px`) og krydses med linjebroer (`BridgeHop`), jf. ADR 005.

I den hidtidige implementering var knækkets position forrykket med ±7 pixels på grund af pilehoveders (`ARROW_HEAD_LENGTH = 14.0`) og diamanters (`DIAMOND_LENGTH = 14.0`) længder, fordi knækket blev udregnet på de afkortede linjepunkter (`line_start_pt`/`line_end_pt`) fremfor de faktiske kant-porte (`start_pt`/`end_pt`).

---

## 📋 Acceptance Criteria
- [ ] **1. Midterknæk uafhængigt af symbolstørrelse**: Enkeltstående relationer (`Generalization`, `Composition`, `Association`) mellem to noder har deres ortogonale knæk præcist i midten mellem nodernes yderkanter (`(start_pt.y + end_pt.y) / 2.0` hhv. `(start_pt.x + end_pt.x) / 2.0`).
- [ ] **2. Ingen linje gennem symboler**: Linjens yderpunkter afkortes fortsat korrekt til pilehovedets base for generalisering og diamantens bagkant for komposition, uden at det forskyder knækkets koordinater.
- [ ] **3. Parallel kanalseparation (Anti-overlap)**: Flere relationer af forskellig type på samme side tildeles deterministiske, parallelle baner med `CHANNEL_OFFSET = 14.0px`, så horisontale/vertikale linjestykker ikke smelter sammen.
- [ ] **4. Linjebroer ved skæringer**: Vinkelrette krydsninger mellem adskilte kanaler og forbindelseslinjer detekteres og markeres med linjebuer (`BridgeHop`) via `detect_bridges`.

---

## 🚫 Must NOT
- Symbollængder (`ARROW_HEAD_LENGTH`, `DIAMOND_LENGTH`, `HALF_ARROW_LENGTH`) må ALDRIG forskyde knækpunkter (`mid_y` / `mid_x`).
- Relationer af forskellig type må ALDRIG overlappe på samme linjesegment.
- Forbindelseslinjer må IKKE tegnes igennem pilehoveder eller diamanters indre.

---

## 🧪 Verifikation
- `cargo test test_task_032_consistent_relation_midpoint_and_channel_routing`
- `cargo test`
