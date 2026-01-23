<template>
  <section id="download" class="download">
    <div class="section__inner">
      <div class="section__label animate-on-scroll">
        <span class="section__number">02</span>
        <span class="section__divider"></span>
        <span>Download</span>
      </div>

      <div class="download__content animate-on-scroll">
        <h2 class="download__title">ISI Voice Image</h2>
        <p class="download__desc">
          Transform images with your voice. A desktop application that brings
          our research directly to your workflow.
        </p>

        <div class="download__card download__card--paper">
          <div class="download__platform">
            <svg class="download__icon" viewBox="0 0 24 24" fill="currentColor">
              <path d="M14 2H6C5.46957 2 4.96086 2.21071 4.58579 2.58579C4.21071 2.96086 4 3.46957 4 4V20C4 20.5304 4.21071 21.0391 4.58579 21.4142C4.96086 21.7893 5.46957 22 6 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V8L14 2Z"/>
              <path d="M14 2V8H20" fill="none" stroke="currentColor" stroke-width="1.5"/>
              <path d="M16 13H8M16 17H8M10 9H8" fill="none" stroke="var(--color-paper)" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
            <div>
              <span class="download__platform-name">Research Paper</span>
              <span class="download__platform-version">PDF, 124 KB</span>
            </div>
          </div>

          <a href="/isi-paper.pdf" class="btn btn--download" target="_blank" rel="noopener">
            Download Paper
            <svg class="btn__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M7 10L12 15L17 10" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15V3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </a>
        </div>

        <div class="download__card">
          <div class="download__platform">
            <svg class="download__icon" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.71 19.5C17.88 20.74 17 21.95 15.66 21.97C14.32 22 13.89 21.18 12.37 21.18C10.84 21.18 10.37 21.95 9.1 22C7.79 22.05 6.8 20.68 5.96 19.47C4.25 17 2.94 12.45 4.7 9.39C5.57 7.87 7.13 6.91 8.82 6.88C10.1 6.86 11.32 7.75 12.11 7.75C12.89 7.75 14.37 6.68 15.92 6.84C16.57 6.87 18.39 7.1 19.56 8.82C19.47 8.88 17.39 10.1 17.41 12.63C17.44 15.65 20.06 16.66 20.09 16.67C20.06 16.74 19.67 18.11 18.71 19.5ZM13 3.5C13.73 2.67 14.94 2.04 15.94 2C16.07 3.17 15.6 4.35 14.9 5.19C14.21 6.04 13.07 6.7 11.95 6.61C11.8 5.46 12.36 4.26 13 3.5Z"/>
            </svg>
            <div>
              <span class="download__platform-name">macOS</span>
              <span class="download__platform-version">10.15+ required</span>
            </div>
          </div>

          <a href="https://github.com/frankdierolf/isi-research/releases" class="btn btn--download" target="_blank" rel="noopener">
            Download for macOS
            <svg class="btn__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M7 10L12 15L17 10" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15V3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </a>
        </div>

        <div class="download__quickstart">
          <h3 class="download__quickstart-title">Quick Start</h3>
          <ol class="download__steps">
            <li class="download__step">
              <span class="download__step-num">1</span>
              <div class="download__step-content">
                <strong>Configure API Keys</strong>
                <p>Open the app settings to add your Deepgram and Gemini API keys.</p>
              </div>
            </li>
            <li class="download__step">
              <span class="download__step-num">2</span>
              <div class="download__step-content">
                <strong>Copy an Image</strong>
                <p>Copy any image to your clipboard (screenshot, photo, etc.)</p>
              </div>
            </li>
            <li class="download__step">
              <span class="download__step-num">3</span>
              <div class="download__step-content">
                <strong>Speak Your Command</strong>
                <p>Press <kbd>Cmd</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd>, speak your transformation, press again to process.</p>
              </div>
            </li>
            <li class="download__step">
              <span class="download__step-num">4</span>
              <div class="download__step-content">
                <strong>Paste Result</strong>
                <p>The transformed image is automatically copied to your clipboard. Just paste!</p>
              </div>
            </li>
          </ol>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.download__content {
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.download__title {
  font-family: var(--font-serif);
  font-size: 2.5rem;
  font-weight: 500;
  letter-spacing: -0.02em;
}

.download__desc {
  font-size: 1.125rem;
  color: var(--color-ink-light);
  max-width: 480px;
}

.download__card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-xl);
  background: var(--color-paper-warm);
  border: 1px solid var(--color-border);
  gap: var(--space-xl);
  flex-wrap: wrap;
}

.download__card + .download__card {
  margin-top: var(--space-md);
}

.download__platform {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.download__icon {
  width: 40px;
  height: 40px;
  color: var(--color-ink);
}

.download__platform-name {
  display: block;
  font-weight: 500;
  font-size: 1.125rem;
}

.download__platform-version {
  display: block;
  font-size: 0.875rem;
  color: var(--color-ink-lighter);
}

.download__quickstart {
  background: var(--color-ink);
  color: var(--color-paper);
  padding: var(--space-2xl);
}

.download__quickstart-title {
  font-family: var(--font-serif);
  font-size: 1.5rem;
  font-weight: 500;
  margin-bottom: var(--space-xl);
}

.download__steps {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.download__step {
  display: flex;
  gap: var(--space-lg);
}

.download__step-num {
  font-family: var(--font-mono);
  font-size: 0.875rem;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--color-ink-lighter);
  border-radius: 50%;
  flex-shrink: 0;
  color: var(--color-paper);
}

.download__step-content strong {
  display: block;
  font-weight: 500;
  margin-bottom: var(--space-xs);
}

.download__step-content p {
  font-size: 0.9375rem;
  color: rgba(250, 250, 250, 0.7);
  line-height: 1.5;
}

.download__step-content code {
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 6px;
  border-radius: 3px;
}

.download__step-content kbd {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  background: rgba(255, 255, 255, 0.15);
  padding: 2px 8px;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

@media (max-width: 768px) {
  .download__card {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
