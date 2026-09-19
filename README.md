# Edge

[![GitHub Release](https://img.shields.io/github/v/release/rolfmadsen/edge?color=blue&logo=github)](https://github.com/rolfmadsen/edge/releases/latest)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)

**Edge** er en moderne, lynhurtig skrivebordsapplikation til FDA begrebs- og informationsmodellering bygget i [Rust](https://www.rust-lang.org) med den reaktive GUI-ramme [Iced](https://iced.rs).

Applikationen giver arkitekter og modelleringsfolk et visuelt lærred til opbygning af FDA-kompatible begrebs- og informationsmodeller med automatisk orthogonal relation-routing, live koncept-studio, multivalente egenskaber og robust JSON-persistens.

---

## 💾 Download installationsfiler

Installationspakker genereres automatisk ved hver ny version og findes under [GitHub Releases (Seneste version)](https://github.com/rolfmadsen/edge/releases/latest).

| Operativsystem | Arkitektur | Filformat | Direkte Download |
| :--- | :--- | :--- | :--- |
| 🍏 **macOS** | Apple Silicon (M1/M2/M3/M4) | `.tar.gz` | [edge-macos-aarch64.tar.gz](https://github.com/rolfmadsen/edge/releases/latest/download/edge-macos-aarch64.tar.gz) |
| 🐧 **Linux** | x86_64 (64-bit) | `.tar.gz` | [edge-linux-x86_64.tar.gz](https://github.com/rolfmadsen/edge/releases/latest/download/edge-linux-x86_64.tar.gz) |
| 🪟 **Windows** | x86_64 (64-bit) | `.zip` | [edge-windows-x86_64.zip](https://github.com/rolfmadsen/edge/releases/latest/download/edge-windows-x86_64.zip) |

*(macOS Intel og Linux ARM64 tilføjes ved efterspørgsel).*

---

## 🚀 Installations- og kørselsvejledning

### 🍏 macOS (Apple Silicon: M1 / M2 / M3 / M4)

1. **Download:** Hent `edge-macos-aarch64.tar.gz` fra tabellen ovenfor.
2. **Pak ud:** Dobbeltklik på filen i Finder eller kør i Terminal:
   ```bash
   tar -xzf edge-macos-aarch64.tar.gz
   ```
3. **Placering:** Flyt den udpakkede `edge`-binær til din foretrukne mappe (f.eks. programmer eller `~/bin`).
4. **Gatekeeper (Vigtigt ved første kørsel):**
   Da Edge er et open source-projekt uden betalt Apple Developer-certifikat, vil macOS Gatekeeper advare om, at programmet kommer fra en uidentificeret udvikler.
   - **Nemmeste løsning:** Højreklik på `edge`-filen i Finder, vælg **Åbn**, og klik derefter på **Åbn** i advarselsdialogen.
   - **Alternativt via Terminal:** Fjern karantæne-flaget:
     ```bash
     xattr -d com.apple.quarantine ./edge
     ./edge
     ```

---

### 🐧 Linux (x86_64)

1. **Download:** Hent `edge-linux-x86_64.tar.gz`.
2. **Pak ud:**
   ```bash
   tar -xzf edge-linux-x86_64.tar.gz
   ```
3. **Gør eksekverbar og kør:**
   ```bash
   chmod +x edge
   ./edge
   ```
4. **Systemafhængigheder:**
   Edge benytter Vulkan/OpenGL via `wgpu`. De fleste moderne desktop-distributioner (Ubuntu, Fedora, Debian, Arch) har de nødvendige biblioteker forudinstalleret. Hvis du kører en minimal installation, installeres afhængighederne via:
   - **Ubuntu/Debian:**
     ```bash
     sudo apt update && sudo apt install -y libwayland-client0 libx11-6 libxkbcommon0 libvulkan1
     ```
   - **Fedora:**
     ```bash
     sudo dnf install wayland-libs libX11 libxkbcommon vulkan-loader
     ```

---

### 🪟 Windows (x86_64)

1. **Download:** Hent `edge-windows-x86_64.zip`.
2. **Pak ud:** Højreklik på `.zip`-filen og vælg **Udpak alle...**.
3. **Start applikationen:** Dobbeltklik på `edge.exe`.
4. **Windows SmartScreen (Ved første kørsel):**
   Hvis Windows Defender SmartScreen viser dialogen *"Windows beskyttede din pc"*:
   - Klik på **Flere oplysninger**.
   - Klik på knappen **Kør alligevel**.

---

## 🛠️ Byg fra kildekode (For udviklere)

Hvis du ønsker at bidrage til Edge eller bygge fra kildekoden, kræver det en fungerende Rust-installation.

### 1. Forudsætninger
- **Rust:** Installér via [rustup.rs](https://rustup.rs/) (Rust 1.80 eller nyere).
- **Linux systembiblioteker (kun på Linux build hosts):**
  ```bash
  sudo apt install -y build-essential pkg-config libx11-dev libxcursor-dev \
    libxrandr-dev libxi-dev libxinerama-dev libwayland-dev libxkbcommon-dev \
    libvulkan1 libasound2-dev
  ```

### 2. Klon og kør
```bash
git clone https://github.com/rolfmadsen/edge.git
cd edge

# Kør i udviklingstilstand
cargo run

# Byg optimeret release-binær
cargo build --release
# Resultatet findes i target/release/edge (eller edge.exe på Windows)
```

### 3. Testsuite
```bash
cargo test
cargo clippy -- -D warnings
```

---

## 📄 Licens

Dette projekt er licenseret under de vilkår, der fremgår af [LICENSE](LICENSE).
