---
type: Task Package
title: "Task 015: Multi-Platform GitHub Release Workflow & README Installationsguide"
description: "Opsætning af GitHub Actions release matrix for Linux, macOS (Apple Silicon) og Windows, suppression af Windows sort konsolvindue, og README med installationsvejledning og release links"
status: completed
generated: { by: process:antigravity-task-init, at: "2026-09-20T00:08:00Z" }
tags: [ci-cd, github-actions, release, multi-platform, readme, installation, windows-subsystem]
---

# Task 015: Multi-Platform GitHub Release Workflow & README Installationsguide

**Status**: `DONE`
**Intent**: `🚀 NEW FEATURE`
**Oprettet**: `2026-09-20`

## 🎯 Formål
1. Oprette et GitHub Actions release-workflow (`.github/workflows/release.yml`), der automatisk kompilerer, pakker og udgiver installationsfiler for Linux (x86_64), macOS (Apple Silicon / aarch64) og Windows (x86_64) ved nye versions-tags (`v*.*.*`) samt via manuel kørsel (`workflow_dispatch`).
2. Konfigurere `src/main.rs` med `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` så Windows-versionen ikke åbner et tomt konsolvindue.
3. Forfatte en komplet og professionel `README.md` der udstiller links til de seneste installationsfiler og giver letforståelige trin-for-trin installationsinstruktioner for Linux, macOS og Windows (inkl. omgåelse af Gatekeeper for u-signede binaries).

## 📋 Acceptance Criteria
- [x] `src/main.rs` konfigureret med `windows_subsystem = "windows"` for release-builds.
- [x] `.github/workflows/release.yml` oprettet med matrix build for Linux x86_64, macOS aarch64 og Windows x86_64, pakning af arkiver (`.tar.gz` og `.zip`), sha256 checksums og automatisk GitHub Release oprettelse.
- [x] `README.md` udbygget med projektbeskrivelse, status badges, direkte download-links for alle tre operativsystemer og udførlig installationsguide for hhv. Linux, macOS og Windows.
- [x] Alle eksisterende tests og lintere valideret lokalt (`cargo check`, `cargo test`, `cargo clippy`).

## 🚫 Must NOT
- Må IKKE ændre eksisterende forretningslogik i modellering eller UI.
- Må IKKE foretage remote publication handlinger (`git push`).
- Må IKKE bryde kompilering på Linux, macOS eller Windows.

## 📝 Revisions
- 2026-09-20: Oprettet efter brugerønske om automatiseret multi-platform release flow ved nye versioner og en brugervenlig README.md.

## 🧪 Verifikation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`
