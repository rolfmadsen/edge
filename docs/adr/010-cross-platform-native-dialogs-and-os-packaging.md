---
type: Architectural Decision Record
title: 'ADR 010: Native Cross-Platform Fildialoger (rfd), Windows Ressource-Indlejring og Rendering Optimering'
status: accepted
tags: [architecture, adr, windows, rfd, dialog, packaging, iced, graphics]
---

# 10. Native Cross-Platform Fildialoger (rfd), Windows Ressource-Indlejring og Rendering Optimering

**Status**: `accepted`  
**Date**: `2026-09-21`  

## Context
Kant distribueres som desktop-applikation til Linux, macOS og Windows.
Ved brugertest på Windows 11 blev der observeret tre platformsspecifikke defekter:
1. **Fildialogfejl**: Åbn/Gem Som-dialoger ("Gennemse...") åbnede ikke Windows Stifinder, da koden hårdkodede eksekveringen af Linux-værktøjet `zenity`.
2. **Manglende Applikationsikon**: `kant.exe` havde standard systemikon i Windows Stifinder/Desktop, og vinduet manglede proceslinje-/titellinje-ikon under kørsel.
3. **Resize Lag og Sort Skærm**: Ved Aero Snap (Win+Arrow eller træk til skærmkant) frøs renderingen, og skærmen/vinduet blev midlertidigt sort grundet WGPU DX12 swapchain-rekonfiguration og Iceds standard 4x MSAA antialiasing.

## Decision
1. **Erstatning af `zenity` med `rfd`**:
   - `src/ui/file_dialog.rs` migrerer til biblioteket `rfd` (Rust File Dialog).
   - På Windows anvender `rfd` direkte Windows native COM API (`IFileDialog`), hvilket åbner ægte Windows Stifinder uden subprocesser.
   - På macOS anvendes Cocoa `NSOpenPanel`/`NSSavePanel`.
   - På Linux anvendes XDG Desktop Portals via D-Bus med Zenity som fallback.
   - Den indbyggede inline fildialog i Iced bevares som fejlsikker fallback ved `DialogResult::Unavailable`.

2. **Windows PE Ressource-Indlejring (`winres`)**:
   - Et multi-resolution ikon (`assets/icons/kant.ico` med 16x16, 32x32, 48x48, 256x256) oprettes fra `assets/icons/kant.svg`.
   - En `build.rs` fil tilføjes med `winres` konfigureret til `target_os = "windows"`, så ikonet indlejres i PE-headeren på `kant.exe` under release-builds.
   - For runtime titellinje og proceslinje indlejres et 32x32 RGBA-ikon direkte i binæren og gives til `iced::window::Settings { icon: ... }`.

3. **Rendering & Resize Optimering**:
   - Iced konfigureres med `.antialiasing(false)` i `src/main.rs`. 4x MSAA reallokering ved dynamisk resize er en primær kilde til GPU-stalls under DX12/DWM på Windows, og for 2D diagram/vektorgrafik giver subpixel tekstrendering kombineret med 1x crisp rendering den optimale balance mellem ydeevne og skarphed.

## Consequences
- **Positive konsekvenser**:
  - Fuld native oplevelse på Windows: Ægte Stifinder-dialoger, korrekt applikationsikon på skrivebord og proceslinje.
  - Væsentligt forbedret responsivitet ved Aero Snap og dynamisk vinduesstørrelsesændring.
  - Ingen eksterne processer (`zenity`) påkrævet på Windows eller macOS.
- **Trade-offs**:
  - `rfd` tilføjer en lille dependency til workspace, men fjerner ustabile kommandolinjekald.
