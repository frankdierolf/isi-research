# ISI Voice Image

Voice-controlled clipboard image transformation for macOS.

> A 2-day experiment from the Ulm Institute of Spoken Intelligence (ISI).
> **[Read the full story →](./JOURNEY.md)**

## What It Does

Copy an image to your clipboard. Press `Cmd+Shift+I`. Speak what you want ("make it black and white", "remove the background", "turn this into a sketch"). Press the shortcut again. Paste the transformed result.

## Quick Start

1. **Download** from [Releases](https://github.com/frankdierolf/isi-research/releases)
2. **Configure** API keys in Settings (Deepgram + Gemini)
3. **Copy** any image to your clipboard
4. **Press** `Cmd+Shift+I`, speak your instruction, press again
5. **Paste** — the transformed image is ready

## Build from Source

```bash
git clone https://github.com/frankdierolf/isi-research
cd isi-research
just dev-desktop
```

Requires: Rust, Node.js, and [just](https://github.com/casey/just) command runner.

See `just --list` for all available commands.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Desktop Framework | [Tauri 2](https://tauri.app/) |
| Frontend | Vue.js 3 + TypeScript |
| Voice Recording | [whis-core](https://crates.io/crates/whis-core) |
| Transcription | [Deepgram](https://deepgram.com/) Nova-2 |
| Image Transformation | [Google Gemini](https://ai.google.dev/) 3 Pro Image |
| Clipboard | [arboard](https://crates.io/crates/arboard) |

## Project Structure

```
isi-research/
├── crates/isi-desktop/     # Tauri desktop app (Rust + Vue)
│   ├── src/                # Rust backend
│   └── ui/                 # Vue frontend
├── website/                # Marketing website
├── justfile                # Build automation
└── JOURNEY.md              # The story behind this project
```

## Links

- **Website**: The "academic paper" framing of this project
- **Competition**: [Kiberatung Developer Contest](https://career.kiberatung.de/developer-contest)
- **whis**: [whis.ink](https://whis.ink) — the voice app this library comes from

## License

MIT
