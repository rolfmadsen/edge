# ADR 013: Evidensbaseret Rendering, Diagnostik og WGPU-genopretning på Windows

## Status
`accepted` (2026-10-03)

## Kontekst
I version 0.5.0 blev `tiny-skia` software rendering indført som tvungen standard på Windows (Task 062) under en ubekræftet hypotese om, at Intel Iris Xe grafikdrivere fejlede ved vinduesmaksimering i WGPU (DirectX 12).

Erfaringen og empirisk observation på Windows 11 (fx Lenovo ThinkPad T14 Gen 2) påviste to alvorlige utilsigtede konsekvenser:
1. **Dramatiske svartider (Latency Regression)**: At rasterisere et rigt, responsivt desktop GUI med interaktivt canvas og 60 fps pan/zoom via CPU software rendering (`tiny-skia`) på skærme med høj DPI/opløsning førte til voldsom forsinkelse og hakken i brugeroplevelsen.
2. **Visuelle Artefakter (Sorte Containere og Paneler)**: Iced's `tiny-skia` rasterizer understøtter ikke CSS-lignende `Shadow` (blur-effekter) eller semi-transparente alpha-lag (`Color::from_rgba`) korrekt, hvilket forårsagede massive sorte firkanter omkring diagrammet, venstre palet og højre inspector.
3. **Mangel på Diagnostisk Evidens**: Applikationen kørte uden logning, konsoloutput eller information om hardwareadaptere, hvilket gjorde det umuligt at diagnosticere reelle GPU- eller swapchain-begivenheder objektivt.

## Beslutning
1. **Genopretning af WGPU som Standard på Windows**:
   - `resolve_default_iced_backend()` gennemtvinger ikke længere `tiny-skia` som standard på Windows.
   - Iced får lov til at benytte `wgpu` som hardware-accelereret motor på alle platforme.
   - Miljøvariablen `ICED_BACKEND=tiny-skia` bevares fortsat som overstyringsmulighed for miljøer uden GPU (fx rene VM'er eller headless CI).
2. **Lokal Struktureret Diagnostik & Logning**:
   - Implementere et letvægts, lokalt diagnostikmodul i `src/features/diagnostics/` (eller `src/ui/diagnostics.rs`), der skriver roterende logfiler til platformens standard-appdata (`%APPDATA%\Kant\logs\kant.log` på Windows).
   - Logge opstarts-konfiguration, aktiv backend, hardware/OS-information og eventuelle fejl.
   - CLI-flag `--diagnostics` / `-d` til øjeblikkelig terminal-inspektion.
   - In-app "System- og grafikdiagnostik..." under "Hjælp"-menuen med knap til udklipsholder-eksport.
3. **Hærdet Opak Styling**:
   - Skifte container- og panelbaggrunde til 100% opake farver (`ThemeColors::SURFACE_CARD`, `Color::WHITE`), så defekt alpha-blending under eventuel fallback ikke resulterer i sorte paneler.
   - Justere skyggeparametre så de forbliver æstetiske på WGPU uden at degenerere til sorte blokke.

## Konsekvenser
- **Positivt**: Genopretter fuld 60 fps GPU-acceleration på Windows uden CPU-flaskehalse.
- **Positivt**: Fjerner de sorte bjælker og container-artefakter.
- **Positivt**: Giver udviklere og brugere direkte evidens (hardwareadapter, driver, backend, tidsstempler) til at fejlfinde eventuelle fremtidige regressionsfejl.
- **Neutralt**: Lokale logfiler optager minimal diskplads (< 5 MB) og overholder strengt Zero Ambient Authority (ingen ekstern telemetri).
