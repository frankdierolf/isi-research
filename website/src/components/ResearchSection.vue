<template>
  <section id="research" class="research">
    <div class="section__inner">
      <div class="section__label animate-on-scroll">
        <span class="section__number">01</span>
        <span class="section__divider"></span>
        <span>Research</span>
      </div>

      <article class="paper animate-on-scroll">
        <header class="paper__header">
          <h2 class="paper__title">
            ISI Voice Image: Voice-Controlled Clipboard Image Transformation
          </h2>
          <div class="paper__meta">
            <span class="paper__authors">Frank Dierolf, Nadja Pirchheim</span>
            <span class="paper__date">2026</span>
          </div>
        </header>

        <div class="paper__content">
          <section class="paper__section">
            <h3 class="paper__heading">Abstract</h3>
            <p class="paper__text">
              We present ISI Voice Image, a system that enables voice-controlled image transformation
              through a simple interaction pattern: Copy. Speak. Paste. Users copy an image to their
              clipboard, speak a transformation command, and paste the modified result—no dedicated
              app window or context switch required.
            </p>
            <p class="paper__text">
              Developed as a 48-hour rapid prototype in the Research Through Design tradition,
              the system demonstrates the technical feasibility of integrating voice input,
              clipboard interaction, and AI image generation into a seamless desktop workflow.
            </p>
          </section>

          <section class="paper__section">
            <h3 class="paper__heading">Methodology</h3>
            <p class="paper__text">
              The system implements a five-stage pipeline: clipboard image capture, audio recording
              triggered by a global hotkey (Cmd+Shift+I), speech-to-text transcription via Deepgram,
              image transformation using Gemini 3 Pro Image, and clipboard output of the result.
              This architecture enables system-wide availability without requiring a dedicated application context.
            </p>
          </section>

          <section class="paper__section">
            <h3 class="paper__heading">Key Findings</h3>
            <ul class="paper__list">
              <li>Voice-controlled clipboard image transformation is technically feasible</li>
              <li>Copy-speak-paste workflow requires no dedicated app context or window</li>
              <li>System-wide availability via global hotkey enables seamless integration</li>
              <li>10-20 second typical latency, comparable to existing AI image tools</li>
            </ul>
          </section>
        </div>

        <footer class="paper__footer">
          <div class="paper__keywords">
            <span class="paper__keyword">Voice Interface</span>
            <span class="paper__keyword">Image Generation</span>
            <span class="paper__keyword">Multimodal AI</span>
            <span class="paper__keyword">Research Through Design</span>
          </div>
          <a href="/isi-paper.pdf" class="btn btn--paper" target="_blank" rel="noopener">
            Download Full Paper
            <svg class="btn__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M7 10L12 15L17 10" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15V3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </a>
        </footer>
      </article>
    </div>
  </section>
</template>

<style scoped>
.research {
  background: var(--color-paper-warm);
  border-top: 1px solid var(--color-border);
  border-bottom: 1px solid var(--color-border);
}

.paper {
  background: var(--color-paper);
  border: 1px solid var(--color-border);
  padding: var(--space-2xl);
}

.paper__header {
  margin-bottom: var(--space-2xl);
  padding-bottom: var(--space-xl);
  border-bottom: 1px solid var(--color-border);
}

.paper__title {
  font-family: var(--font-serif);
  font-size: 1.75rem;
  font-weight: 500;
  line-height: 1.3;
  letter-spacing: -0.01em;
  margin-bottom: var(--space-md);
}

.paper__meta {
  display: flex;
  gap: var(--space-lg);
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  color: var(--color-ink-lighter);
}

.paper__content {
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
}

.paper__section {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.paper__section + .paper__section {
  padding-top: var(--space-xl);
  border-top: 1px solid var(--color-border);
}

.paper__heading {
  font-family: var(--font-sans);
  font-size: 0.8125rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--color-ink-light);
}

.paper__text {
  font-size: 1rem;
  line-height: 1.75;
  color: var(--color-ink);
}

.paper__list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.paper__list li {
  padding-left: var(--space-lg);
  position: relative;
  line-height: 1.6;
}

.paper__list li::before {
  content: '—';
  position: absolute;
  left: 0;
  color: var(--color-ink-lighter);
}

.paper__footer {
  margin-top: var(--space-xl);
  padding-top: var(--space-xl);
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: var(--space-lg);
}

.paper__keywords {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
}

.paper__keyword {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  padding: var(--space-xs) var(--space-sm);
  background: var(--color-paper-warm);
  border: 1px solid var(--color-border);
  color: var(--color-ink-light);
}

@media (max-width: 768px) {
  .paper {
    padding: var(--space-lg);
  }
}
</style>
