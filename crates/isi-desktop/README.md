<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/isi-research/main/crates/isi-desktop/icons/128x128.png" alt="ISI Voice Image" width="80" height="80" />

<h3>ISI Desktop</h3>
<p>
  The Tauri desktop application for ISI Voice Image.
  <br />
  <a href="https://isi-research.org">Website</a>
  ·
  <a href="https://github.com/frankdierolf/isi-research">Repository</a>
  ·
  <a href="https://github.com/frankdierolf/isi-research/releases">Releases</a>
</p>
</div>

---

## Overview

A macOS menu bar application that transforms clipboard images using voice commands. Built with Tauri 2, Rust, and Vue.js.

## Architecture

```
Copy Image → Speak → Paste Transformed
```

Pipeline: Hotkey → whis-core (Record) → Deepgram (Transcription) → Gemini 3 Pro (Transform) → Clipboard

## Key Files

| File | Purpose |
|------|---------|
| `src/recording.rs` | State machine for voice-to-image pipeline |
| `src/gemini.rs` | Gemini API integration |
| `src/clipboard.rs` | Clipboard image read/write |
| `src/lib.rs` | Tauri setup, commands, system tray |

## Development

```bash
# From repository root
just dev
```

## License

MIT
