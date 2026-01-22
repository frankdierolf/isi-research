<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/isi-research/main/crates/isi-desktop/icons/128x128.png" alt="ISI Voice Image" width="80" height="80" />

<h3>ISI Voice Image</h3>
<p>
  Voice-controlled clipboard image transformation for macOS.
  <br />
  <a href="https://isi-research.org">Website</a>
  ·
  <a href="./crates/isi-desktop">Desktop App</a>
  ·
  <a href="https://github.com/frankdierolf/isi-research/releases">Releases</a>
</p>
</div>

---

> A 48-hour experiment from the Ulm Institute of Spoken Intelligence (ISI).
> **[Read the full story →](./journey/)**

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
just dev
```

Requires: Rust, Node.js, and [just](https://github.com/casey/just) command runner.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Desktop Framework | [Tauri 2](https://tauri.app/) |
| Frontend | Vue.js 3 + TypeScript |
| Voice Recording | [whis-core](https://crates.io/crates/whis-core) |
| Transcription | [Deepgram](https://deepgram.com/) Nova-2 |
| Image Transformation | [Google Gemini](https://ai.google.dev/) 3 Pro |
| Clipboard | [arboard](https://crates.io/crates/arboard) |

## Project Structure

```
isi-research/
├── crates/isi-desktop/   # Tauri desktop app (Rust + Vue)
├── website/              # Marketing site at isi-research.org
├── journey/              # The story behind this project
└── justfile              # Build automation
```

## Media

- **[Demo Video](./demo-video.mp4)** — Showcase of the app in action
- **[The Journey](./journey/)** — The 48-hour story behind this project
  - [Podcast](./journey/podcast.m4a) — Audio version via NotebookLM
  - [Infographic](./journey/infographic.png) — Visual overview

## Links

- **Website**: [isi-research.org](https://isi-research.org)
- **Competition**: [Kiberatung Developer Contest](https://career.kiberatung.de/developer-contest)
- **whis**: [whis.ink](https://whis.ink) — the voice library this project uses

## License

MIT
