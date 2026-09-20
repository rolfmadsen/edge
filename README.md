# Edge

[![GitHub Release](https://img.shields.io/github/v/release/rolfmadsen/edge?color=blue&logo=github)](https://github.com/rolfmadsen/edge/releases/latest)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)

**Edge** er en moderne, lynhurtig skrivebordsapplikation til FDA begrebs- og informationsmodellering bygget i [Rust](https://www.rust-lang.org) med den reaktive GUI-ramme [Iced](https://iced.rs).

Applikationen giver arkitekter og modelleringsfolk et visuelt lærred til opbygning af FDA-kompatible begrebs- og informationsmodeller med automatisk orthogonal relation-routing, live koncept-studio, multivalente egenskaber og robust JSON-persistens.

---

## 💾 Download installationsfiler

Installationspakker genereres automatisk ved hver ny version og findes under [GitHub Releases (Seneste version)](https://github.com/rolfmadsen/edge/releases/latest).

| Operativsystem | Arkitektur | Filformat | Beskrivelse | Direkte Download |
| :--- | :--- | :--- | :--- | :--- |
| 🐧 **Linux (Debian / Ubuntu / Mint / Pop!_OS)** | x86_64 | `.deb` | Fuld systeminstallation m. app-ikon & menu | [Hent `.deb` pakke](https://github.com/rolfmadsen/edge/releases/latest) |
| 🐧 **Linux (Alle distributioner)** | x86_64 | `.tar.gz` | Transportabel binær | [edge-linux-x86_64.tar.gz](https://github.com/rolfmadsen/edge/releases/latest/download/edge-linux-x86_64.tar.gz) |
| 🍏 **macOS** | Apple Silicon (M1/M2/M3/M4) | `.tar.gz` | Transportabel binær | [edge-macos-aarch64.tar.gz](https://github.com/rolfmadsen/edge/releases/latest/download/edge-macos-aarch64.tar.gz) |
| 🪟 **Windows** | x86_64 | `.zip` | Transportabel applikation (`edge.exe`) | [edge-windows-x86_64.zip](https://github.com/rolfmadsen/edge/releases/latest/download/edge-windows-x86_64.zip) |

*(macOS Intel og Linux ARM64 tilføjes ved efterspørgsel).*

---

## 🚀 Installations- og kørselsvejledning

### 🐧 Linux (x86_64)

#### Metode A: Debian / Ubuntu / Mint / Pop!_OS (`.deb` — Anbefalet)
Installerer Edge direkte i dit system og tilføjer programmet til din applikationsmenu med officielt logo:
1. Hent den seneste `.deb`-fil fra [GitHub Releases](https://github.com/rolfmadsen/edge/releases/latest).
2. Dobbeltklik på den hentede fil for at åbne den i dit Software Center, eller kør i Terminal:
   ```bash
   sudo apt install ./edge_*_amd64.deb
   ```
3. Start Edge fra dit skrivebords programstarter eller ved blot at skrive `edge` i en terminal.

#### Metode B: Transportabel arkiv (`.tar.gz` — Alle distributioner)
1. Hent `edge-linux-x86_64.tar.gz`.
2. Pak arkivet ud og kør programmet:
   ```bash
   tar -xzf edge-linux-x86_64.tar.gz
   chmod +x edge
   ./edge
   ```
3. **Systemafhængigheder:**
   Edge benytter Vulkan/OpenGL via `wgpu`. De fleste moderne desktop-distributioner har disse forudinstalleret. Ved en minimal installation:
   - **Ubuntu/Debian:** `sudo apt update && sudo apt install -y libwayland-client0 libx11-6 libxkbcommon0 libvulkan1`
   - **Fedora:** `sudo dnf install wayland-libs libX11 libxkbcommon vulkan-loader`

---

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

## 🌐 E2EE Realtids-kollaborering & Relay Server (`edge-relay`)

Edge understøtter synkron, end-to-end krypteret (E2EE) modellering i realtid mellem flere deltagere via en uafhængig, ultralet og "blind" WebSocket relay-server ([ADR 008](docs/adr/008-e2ee-realtime-collaboration-and-stateless-relay.md)).

Relayen opbevarer **nul data på disk** (100% in-memory), kræver ingen ekstern database og router udelukkende krypterede frames mellem klienter forbundet til samme sessionskode.

### 1. Kør relay-serveren lokalt med Docker (Anbefalet)
Repositoryet indeholder en færdig [`docker-compose.yml`](docker-compose.yml):
```bash
# Start relay-serveren i baggrunden (port 8080)
docker compose up -d

# Bekræft at servicen kører og svarer sundt
curl http://localhost:8080/health
```
I Edge vælges preset: `Lokal Docker (ws://localhost:8080/ws)`.

### 2. Kør lokalt via Cargo (Uden Docker)
```bash
cargo run -p edge-relay
```

### 3. Sky-udrulning (Koyeb PaaS / Egen organisation)
Relayen kan udrulles på enhver containerplatform eller PaaS uden driftsomkostninger:
- **1-Klik Koyeb Deploy:** Benyt [`koyeb.yaml`](koyeb.yaml) eller klik på deploy-knappen i [`crates/edge-relay/README.md`](crates/edge-relay/README.md).
- **Miljøvariable:** `PORT=8080`, `HOST=0.0.0.0`, `RUST_LOG=info`.

For API-specifikation og yderligere tekniske detaljer henvises til [crates/edge-relay/README.md](crates/edge-relay/README.md).

---

## 📄 Licens

Dette projekt er licenseret under de vilkår, der fremgår af [LICENSE](LICENSE).
