<div align="center">
<img src="https://raw.githubusercontent.com/frankdierolf/isi-research/main/crates/isi-desktop/icons/128x128.png" alt="ISI Voice Image" width="80" height="80" />

<h3>The Journey</h3>
<p>
  How we built a voice-controlled image transformer in 48 hours.
  <br />
  <a href="../">← Back to Project</a>
</p>
</div>

---


# Podcast 
[podcast.webm](https://github.com/user-attachments/assets/a81f7086-6563-4720-9b4c-536a8a272d03)

# The story

## The Premise

January 2026. I had this idea: what if "spinning up a research institute" could be as lightweight as spinning up a startup? Not the bureaucracy, not the funding cycles—just the essence: focused exploration of a question that matters to you.

The concept: a pop-up research institute. Like a pop-up store, it exists for a limited time, produces something tangible, and then closes. Ours would exist for exactly two days.

The question we wanted to explore: **How seamlessly can voice control drive image manipulation with today's multimodal models?**

The context: I found the [Kiberatung Developer Contest](https://career.kiberatung.de/developer-contest), a German competition that fit nicely. Build a voice intelligence app—desktop, hotkey-activated, speech-to-something-useful. Perfect for a feasibility study.

---

## Day 1: MacinCloud from Linux

**Location:** My desk, staring at a Linux machine
**Challenge:** Build a macOS app without owning a Mac

I've been building [whis](https://whis.ink)—a voice application—and its core library `whis-core` handles audio recording and transcription. This project would be a test: can I just `cargo add whis` and have voice capabilities in a new app?

For the macOS problem, I found MacinCloud—a service that lets you VNC into a Mac in the cloud through your browser. It's finicky. The connection drops. The latency makes everything feel sluggish. But it works.

The real challenge: testing voice features through a cloud VNC session. Sharing microphone audio over the internet to a remote Mac? Not ideal. I had to create workarounds, testing audio locally and praying it would work the same on real hardware.

I chose Tauri over Electron. Rust backend, smaller binaries, more native feeling. The frontend would be Vue.js because I wanted something lightweight and the project scope was small.

Early on, I wanted a floating overlay—something that appears when you activate the hotkey, shows recording state, maybe displays the transcription in real-time. I looked at other Tauri projects doing similar things: spotlight clones, launchers, audio visualizers. They all deal with the same platform-specific quirks. Window management on macOS is tricky. After wrestling with it longer than I wanted, I pivoted: system tray + native notifications instead. Ship something that works.

**Technical aside:** The `whis-core` crate came from my earlier work on [whis.ink](https://whis.ink). It handles voice activity detection, audio chunking, and streaming transcription to providers like Deepgram. Being able to `cargo add whis-core` and have that functionality immediately available felt like compound interest on past work.

---

## Day 2: Coffee Shop Hackathon

**Location:** A coffee shop in Ulm, Germany
**Partner:** Nadja

I met up with Nadja. She's a UI/UX designer I know from hackathons. I asked if she wanted to spend a day in a coffee shop and talk to talk. When I arrived, I learned she had a Mac.

That changed everything.

We shared her MacBook. I'm a senior software engineer; she's a designer. We taught each other as we worked—I'd explain why the Rust code was structured a certain way, she'd point out UX issues I'd missed. Both of us are *Mädchen für alles*—generalists who do whatever needs doing.

The debugging was collaborative. When the clipboard reading failed silently, we traced it together. When the notifications weren't showing, she noticed the permission popup I'd dismissed. When the Gemini API returned malformed responses, we figured out the base64 encoding issue side by side.

The use case crystallized from her workflow: as a designer, she constantly finds herself grabbing images from the internet and needing quick edits. Open ChatGPT, upload the image, describe the change, download the result, paste it somewhere. What if the whole thing was: copy, speak, paste?

By the end of the day, it worked. Copy an image to clipboard. Press `Cmd+Shift+I`. Say "make it black and white" or "remove the background" or "turn this into a sketch." Press the shortcut again. The transformed image is in your clipboard. Paste.

---

## What We Built

```
┌─────────────────────────────────────────────────────────┐
│                    ISI Voice Image                      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐   ┌───────────┐   ┌────────────────────┐   │
│  │ Hotkey  │ → │ whis-core │ → │     Deepgram       │   │
│  │ Trigger │   │ (Record)  │   │  (Transcription)   │   │
│  └─────────┘   └───────────┘   └────────────────────┘   │
│       ↓                               ↓                 │
│  ┌─────────┐                  ┌───────────────────┐     │
│  │Clipboard│ ───────────────→ │ Gemini 3 Pro      │     │
│  │ (Image) │                  │ (Transform Image) │     │
│  └─────────┘                  └───────────────────┘     │
│                                       ↓                 │
│                              ┌───────────────┐          │
│                              │   Clipboard   │          │
│                              │   (Result)    │          │
│                              └───────────────┘          │
└─────────────────────────────────────────────────────────┘
```

### The Pipeline

1. **Trigger**: Global shortcut (`Cmd+Shift+I`) via `tauri-plugin-global-shortcut`
2. **Record**: `whis-core::AudioRecorder` captures microphone input
3. **Transcribe**: Audio sent to Deepgram Nova-2 via `progressive_transcribe_cloud()`
4. **Read**: `arboard` crate reads the current clipboard image
5. **Transform**: Image + transcription sent to Gemini 3 Pro Image
6. **Write**: Transformed image written back to clipboard
7. **Notify**: Success notification via `tauri-plugin-notification`

### Key Files

| File | What It Does |
|------|--------------|
| `crates/isi-desktop/src/recording.rs` | State machine for the full pipeline (386 lines) |
| `crates/isi-desktop/src/gemini.rs` | Gemini API integration |
| `crates/isi-desktop/src/clipboard.rs` | Clipboard image I/O |
| `crates/isi-desktop/src/lib.rs` | Tauri app setup, commands, system tray |
| `crates/isi-desktop/ui/src/views/HomeView.vue` | Settings interface |
| `website/src/App.vue` | The "academic paper" marketing site |

---

## Learnings

### On Building

**Tauri overlays are finicky.** We wanted a floating UI during recording—something like a mini waveform or transcription preview. We tried. We looked at how other projects handle it. It's definitely possible, but it's not a one-shot AI prompt to get right. The platform-specific window management takes real time. We pivoted to system tray + notifications and shipped.

**`cargo add` is magic.** Pulling in `whis-core` took seconds. One line in `Cargo.toml` and I had audio recording, voice activity detection, and Deepgram integration. The Rust ecosystem rewards building reusable crates. Past work compounds.

**MacinCloud works, barely.** Good enough for compilation and basic UI testing. Terrible for anything involving audio or real-time interaction. Having Nadja's Mac on day two wasn't just nice—it was essential for polish.

### On the Pop-Up Institute Concept

**Two days is enough for a proof of concept.** Not a product. Not a paper. But enough to answer "is this possible?" and "is this interesting?" Yes and yes.

**The constraint is generative.** Knowing we had 48 hours forced decisions. The overlay got cut. The settings UI stayed simple. We scoped to what mattered: does the core loop work?

**The marketing site matters.** We built a website that frames ISI as a research institute. Fake academic paper, abstract, methodology, key findings. It's playful, but it does something useful: it forces you to articulate what you learned. "Abstract" and "Methodology" make you think differently than "Features" and "Installation."

### On Voice + Image

**Latency is acceptable.** About 2-3 seconds from finishing your command to seeing the result. Fast enough to feel magical, slow enough to notice.

**Gemini 3 Pro Image is surprisingly capable.** Complex transformations work. "Make this image look like a watercolor painting"—it gets it. Simple ones sometimes overthink. "Make it smaller" might give you a philosophical reinterpretation instead of a resized image.

**Voice feels right for quick commands.** "Remove the background" is faster to say than to find the eraser tool. For short, clear instructions, voice wins.

---

## Why It Matters (Or Doesn't)

This isn't going to change the world. It's a feasibility study—a snapshot of what's possible in January 2026 with:

- Multimodal LLMs that can edit images from text instructions
- Voice transcription that's fast and accurate enough
- Desktop frameworks that mostly work cross-platform
- AI coding assistants that let two people spin up infrastructure in hours

A year ago, this project would have been significantly harder. The image models weren't as capable. The transcription had more friction. The tooling for desktop apps was rougher.

A year from now, this will feel dated. That's the point.

I do this every year: take stock of where the frontier is. Build something at the edge of what's possible. Use it as a reference point for what comes next.

Last year's results were already mind-bending. This year, in two days, we built a working desktop app, a marketing website framed as an academic paper, and put it all on GitHub. The infrastructure that used to take weeks now takes hours. The models that used to fail now mostly work.

What happens next year? Can I push this further—same timeframe, but add a business model? Ship something with actual users in 48 hours? The ceiling keeps rising.

---

## The Fake Academic Framing

Visit the website and you'll find the "Ulm Institute of Spoken Intelligence" presenting a research paper: *Voice-Controlled Image Manipulation: A Study in Real-Time Visual Transformation*.

It's not real research. It's two people in a coffee shop.

But the framing does something useful. It forces you to articulate findings. "Abstract," "Methodology," "Key Findings"—these sections make you structure your thoughts. The statistics are made up, but the insights are real.

And there's something delightful about a research institute that exists for one day only: January 22nd, 2026, in Ulm, Germany.

---

## Credits

**Frank** — Code, whis-core, late night MacinCloud sessions
**Nadja** — Design, debugging, the Mac, the coffee shop idea

Built with:
- [Tauri](https://tauri.app/) — Desktop framework
- [whis-core](https://crates.io/crates/whis-core) — Voice recording and transcription
- [Deepgram](https://deepgram.com/) — Speech-to-text (Nova-2)
- [Google Gemini](https://ai.google.dev/) — Image transformation (Gemini 3 Pro Image)
- [Vue.js](https://vuejs.org/) — UI framework
- [arboard](https://crates.io/crates/arboard) — Clipboard with image support

And a coffee shop in Ulm that didn't kick us out.

---

## Try It

The goal wasn't to build something others would use. But if you want to:

```bash
git clone https://github.com/frankdierolf/isi-research
cd isi-research
just dev-desktop
```

Add your API keys (Deepgram + Gemini) in the settings. Copy an image. Press the shortcut. Speak. Press again. Paste.

And maybe consider: what would *your* pop-up research institute explore?
