# ext-miracast

> **Pure-Rust Autonomous Universal Miracast (Wi-Fi Display / WFD 1.0 / MS-MICE) Client**

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Protocol: WFD 1.0](https://img.shields.io/badge/Protocol-Wi--Fi%20Display%201.0-blue.svg)]()
[![Hardware Acceleration](https://img.shields.io/badge/GPU-AMD%20%7C%20Intel%20%7C%20NVIDIA-orange.svg)]()

`ext-miracast` é um cliente Miracast (Wi-Fi Display / WFD 1.0 / MS-MICE) de alta performance, 100% implementado em Rust, independente de GTK ou GNOME Network Displays. Projetado para transmissão de vídeo de ultrabaixa latência com aceleração nativa por hardware e múltiplos backends de captura selecionáveis.

---

## 🚀 Principais Recursos

- **Zero GTK / Zero GUI Bloat:** Opera de forma autônoma como binário CLI de linha de comando ou biblioteca Rust (`lib.rs`).
- **Aceleração Multi-Arquitetura por Hardware:**
  - **AMD Radeon:** Direct VA-API via `vah264enc` / `vaapih264enc` no `/dev/dri/renderD128`
  - **Intel:** QuickSync VA-API (`qsvh264enc` / `vaapih264enc`)
  - **NVIDIA:** NVENC direto (`nvh264enc`)
  - **CPU Fallback:** Cisco OpenH264 / `x264enc` ultrabaixa latência
- **Backends de Captura Selecionáveis:**
  - `--capture mutter` (Padrão no GNOME Wayland): Usa `org.gnome.Mutter.ScreenCast` D-Bus e PipeWire para criar telas virtuais sem hardware extra.
  - `--capture kernel` ou `--kms`: Captura direta no nível de Kernel Linux via subsistema DRM/KMS (`/dev/dri/card*`) e Framebuffer (`/dev/fb0`), com bypass total de D-Bus e do compositor.
  - `--capture x11`: Captura direta via X11 XShm (`ximagesrc`).
- **Modos de Exibição:**
  - `--mode extend` (ou `--extend`): Cria uma segunda área de trabalho virtual estendida independente.
  - `--mode clone` (ou `--clone`): Espelha a tela primária (`eDP-1`) em tempo real.
- **Protocolo Wi-Fi Display Completo:**
  - Máquina de estados RTSP 1.0 WFD (mensagens M1 a M7) e MS-MICE (porta 7250 e 7236).
  - Multiplexador MPEG-TS com pacotes de 188 bytes, cálculo de CRC32 (PAT/PMT), injeção de PCR e PTS de 90 kHz.
  - Encapsulamento RTP RFC 2250 (Payload Type 33 MP2T).
- **Varredura e Descoberta Automática:**
  - Busca receptores Miracast locais via mDNS (`_display._tcp.local`), SSDP (UPnP) e Direct-USB.
- **Encerramento Limpo em Sub-100ms:**
  - Captura `SIGINT`/`SIGTERM` para enviar `TEARDOWN` imediato ao receptor e restaurar o terminal instantaneamente.

---

## 📦 Instalação e Compilação

Requisitos: Rust 1.75+ e GStreamer com plugins base/good/bad/vaapi.

```bash
# Clonar o repositório
git clone https://github.com/programandosolucoes/ext-miracast.git
cd ext-miracast

# Compilar em modo release
cargo build --release

# O binário estará pronto em:
./target/release/ext-miracast --help
```

---

## 💻 Uso e Exemplos de Linha de Comando

### 1. Conexão Direta em Modo Clone (Espelhamento da Tela Principal)
```bash
ext-miracast 192.168.7.2 --clone
# ou especificando explicitamente:
ext-miracast 192.168.7.2 --mode clone
```

### 2. Conexão em Modo Estendido (Segunda Tela Virtual)
```bash
ext-miracast 192.168.7.2 --extend
# ou:
ext-miracast 192.168.7.2 --mode extend
```

### 3. Seleção do Backend de Captura
```bash
# Captura direta a nível de Kernel (DRM/KMS scanout, sem Mutter/D-Bus)
ext-miracast 192.168.7.2 --capture kernel --clone

# Captura sob X11 clássico
ext-miracast 192.168.7.2 --capture x11 --clone

# Captura Wayland Mutter (padrão)
ext-miracast 192.168.7.2 --capture mutter --extend
```

### 4. Ajuste de Resolução, FPS e Bitrate
```bash
# 720p60 a 4 Mbps (Nativo CEA index 6 - ideal para Raspberry Pi Zero)
ext-miracast 192.168.7.2 --res 720p --fps 60 --bitrate 4000

# 1080p30 a 6 Mbps
ext-miracast 192.168.7.2 --res 1080p --fps 30 --bitrate 6000
```

### 5. Descoberta e Varredura de Redes
```bash
# Escaneia receptores na rede e lista para seleção interativa
ext-miracast --scan

# Sem informar IP, o scanner interativo é acionado automaticamente:
ext-miracast
```

---

## 🏗️ Arquitetura Interna

```
┌────────────────────────────────────────────────────────┐
│                      ext-miracast                      │
├──────────────────────────┬─────────────────────────────┤
│  Capture Backends        │  Protocol Stack             │
│  - Mutter D-Bus ScreenCast│  - RTSP WFD M1-M7 / MS-MICE │
│  - Kernel DRM/KMS Scanout│  - MPEG-TS 188-byte Muxer   │
│  - X11 XShm Screen Grab  │  - RTP MP2T (PT=33) Sender  │
├──────────────────────────┴─────────────────────────────┤
│  GPU Hardware Encoders (AMD VA-API, Intel, NVIDIA)     │
└──────────────────────────┬─────────────────────────────┘
                           │ RTP UDP
                           ▼
             Raspberry Pi Zero W / Miracast Sink
```

---

## 🧪 Testes Automatizados

O projeto conta com ampla cobertura de testes unitários para todos os módulos:

```bash
cargo test
```

---

## 📄 Licença

Distribuído sob licença MIT. Consulte o arquivo [LICENSE](LICENSE) para obter detalhes.
Desenvolvido por **Carlos Alberto** (<psncarlosalberto4ti@gmail.com>).
