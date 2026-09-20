# 📡 Edge Relay (`edge-relay`)

Ultralet, in-memory, blind Axum WebSocket pub/sub relay til **Edge E2EE Realtids-kollaborering** ([ADR 008](../../docs/adr/008-e2ee-realtime-collaboration-and-stateless-relay.md)).

[![Deploy to Koyeb](https://www.koyeb.com/static/images/deploy/button.svg)](https://app.koyeb.com/deploy?type=git&repository=github.com/rolfmadsen/edge&branch=main&env[PORT]=8080)

---

## 🎯 Arkitektur & Egenskaber
- **100% Stateless & In-Memory:** Gemmer nul data på disk, ingen ekstern database eller cache nødvendig.
- **Blind Byte-Router:** Relayen inspicerer aldrig modeller eller payloads. Al data krypteres klientside med ChaCha20-Poly1305.
- **Last-Snapshot Buffer:** Første frame i rummet (eller snapshot-frame) bufferes i RAM, så nytilkomne gæster automatisk modtager den aktuelle modeltilstand ved tilslutning.
- **Automatisk RAM-oprydning:** Inaktive rum uden forbundne klienter frigives automatisk fra hukommelsen efter 60 sekunder.
- **Minimalt Footprint:** Containerimage < 15MB, RAM-forbrug typisk under 10MB.

---

## 🔌 API Endpoints
- `GET /health` -> `200 OK` (`"OK"`)
- `GET /ws?room=<ROOM_ID>` -> WebSocket handshake og pub/sub kanal.

---

## ⚙️ Miljøvariable
| Variabel | Standard | Beskrivelse |
|---|---|---|
| `HOST` | `0.0.0.0` | Bind-adresse for HTTP/WebSocket server |
| `PORT` | `8080` | Bind-port |
| `RUST_LOG` | `info` | Tracing log-niveau (f.eks. `info`, `debug`, `trace`) |

---

## 🚀 Kørsel & Udrulning

### Lokal afvikling via Cargo
```bash
cargo run -p edge-relay
```

### Docker Compose
```bash
docker compose up -d
```

### 1-Klik Udrulning på Koyeb (PaaS)
Brug den deklarative konfiguration i [`koyeb.yaml`](../../koyeb.yaml) eller klik på Koyeb-udrulningsknappen ovenfor.
