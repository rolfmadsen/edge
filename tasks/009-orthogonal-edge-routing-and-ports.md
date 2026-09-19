---
type: Task Package
title: "Task 009: Ortogonal Edge-Routing, Præcise Pilehoveder og Port-Bus"
description: "90 graders ortogonale linjer, synlige pilehoveder forankret på nodekant, nærheds-portskift, parallelle baner og linjebroer"
status: done
generated: { by: process:xgauntlet-task-init, at: "2026-09-19T17:47:00Z" }
tags: [task-lifecycle, intent, ui, canvas, graph, edges, routing, orthogonal, fda]
---

# Task 009: Ortogonal Edge-Routing, Præcise Pilehoveder og Port-Bus

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-19`

## 🎯 Formål
Forbedre og professionalisere relationer/edges på begrebsmodel-canvas i overensstemmelse med FDA Modelreglerne (Kapitel 5 & 7). Relationer skal routes med rene 90-graders Manhattan-knæk frem for skrå linjer, pilehoveder (lukkede hvide trekanter for generalisering) skal ramme nodens ydre kant eksakt og være fuldt synlige, forankringsporte skal automatisk skifte side når noder kommer for tæt på hinanden, relationer af samme type skal samles i fælles anker/stamme, relationer af forskellig type skal løbe parallelt i adskilte kanaler, og linjekrydsninger skal markeres med visuelle broer (line-hops).

## 📋 Acceptance Criteria
- [x] **Synlige og præcise pilehoveder**: Spidsen af pilehovedet (hvid lukket trekant for generalisering, vinkelret på kanten) rører præcist nodens ydre afgrænsning uden at blive skjult bag nodens fyld eller streg.
- [x] **FDA Label Semantik**: Almindelige generaliseringspile har ingen vilkårlig fritekst-label jf. FDA regel §5.6 / linje 1474 & 1526; associationer viser associationsnavn centreret på det primære linjesegment.
- [x] **90-graders ortogonal Manhattan-routing**: Alle relationer tegnes udelukkende med horisontale og vertikale segmenter med præcise 90° vinkler.
- [x] **Nærheds-håndtering og port-skift**: Når to forbundne noder bringes tættere sammen end det nødvendige pilerum ($D_{\text{min}} = 36\text{px}$), skifter portene automatisk til sideporte (eller ekstern omløbskorridor), så pilen aldrig klemmes flad eller inverteres.
- [x] **Multi-relation anker & parallelle kanaler**: Flere relationer af samme type på samme nodeside samles i samme ankerpunkt/stamme mod målet; relationer af forskellig type fordeles symmetrisk i slots langs siden og rutes i parallelle baner med fast afstand (mindst 12px) uden overlap.
- [x] **Krydsende linjebroer**: Når to ortogonale edges krydser hinanden, markeres skæringspunktet med en visuel bue/bro (jump arc) på den ene linje for at indikere at de ikke forbinder.
- [x] **Fuld testverifikation & ren kode**: Matematiske enhedstests for ortogonal routing, port-allokering og krydsningsdetektion passerer 100%, og `cargo clippy -- -D warnings` rapporterer 0 advarsler.

## 🚫 Must NOT
- Spidsen af et pilehoved må ALDRIG placeres inde under en nodes rektangel så det skjules.
- Relationer må IKKE tegnes som skrå eller kurvede Bézier-linjer (skal være strenge 90° ortogonale segmenter).
- Forskellige relationer må IKKE have sammenfaldende koordinater eller overlappe visuelt.
- Må IKKE bryde FDA Modelreglernes semantiske krav om generaliseringers betydning.
- Må IKKE foretage remote publication (`git push`).

## 📝 Revisions
- 2026-09-19: Oprettet task 009 baseret på bruger-sparring og FDA Modelreglerne.
- 2026-09-19: Præciseret nærhedsløsning for pilerum ($D_{\text{min}}$) og FDA regel om udeladelse af labels på enkeltstående generaliseringer.

## 🧪 Verifikation
- `cargo test --tests`
- `cargo clippy -- -D warnings`
