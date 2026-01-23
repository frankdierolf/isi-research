<template>
  <section id="about" class="about">
    <div class="section__inner">
      <div class="section__label animate-on-scroll">
        <span class="section__number">03</span>
        <span class="section__divider"></span>
        <span>About</span>
      </div>

      <div class="about__content animate-on-scroll">
        <h2 class="about__title">About the Pop-up Institute</h2>
        <div class="about__text">
          <p>
            The Ulm Institute of Spoken Intelligence (ISI) is a pop-up research
            institute—inspired by the concept of a pop-up store, it exists for
            one day only: January 22nd, 2026, in Ulm, Germany.
          </p>
          <p>
            Dedicated to advancing research in multimodal artificial intelligence,
            we focus on voice-based interaction paradigms for creative and productive
            workflows, exploring how natural language understanding and generative
            models can create more intuitive human-computer interaction.
          </p>
        </div>

        <div class="about__focus">
          <h3 class="about__focus-title">Research Focus Areas</h3>
          <ul class="about__focus-list">
            <li>Clipboard-based voice-to-image transformation</li>
            <li>Research Through Design methodology</li>
            <li>Feasibility studies with rapid prototyping</li>
            <li>Multimodal interaction for creative workflows</li>
          </ul>
        </div>

        <div class="about__contact">
          <h3 class="about__contact-title">Contact</h3>
          <div class="about__contact-links">
            <a href="mailto:frank_dierolf@web.de" class="about__email">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M22 6l-10 7L2 6" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              Email us
            </a>
            <a href="https://github.com/frankdierolf/isi-research" class="about__github" target="_blank" rel="noopener">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
              </svg>
              View on GitHub
            </a>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.about__content {
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.about__title {
  font-family: var(--font-serif);
  font-size: 2.5rem;
  font-weight: 500;
  letter-spacing: -0.02em;
}

.about__text {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.about__text p {
  font-size: 1.0625rem;
  line-height: 1.75;
  color: var(--color-ink-light);
}

.about__focus {
  padding: var(--space-xl);
  background: var(--color-paper-warm);
  border-left: 3px solid var(--color-ink);
}

.about__focus-title {
  font-family: var(--font-sans);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  margin-bottom: var(--space-md);
}

.about__focus-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.about__focus-list li {
  padding-left: var(--space-lg);
  position: relative;
  color: var(--color-ink-light);
}

.about__focus-list li::before {
  content: '→';
  position: absolute;
  left: 0;
  color: var(--color-accent);
}

.about__contact {
  padding-top: var(--space-xl);
  border-top: 1px solid var(--color-border);
}

.about__contact-title {
  font-family: var(--font-sans);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  margin-bottom: var(--space-md);
}

.about__contact-links {
  display: flex;
  gap: var(--space-xl);
  flex-wrap: wrap;
}

.about__email {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--color-ink);
  text-decoration: none;
  font-size: 0.9375rem;
  font-weight: 500;
  transition: color 0.2s var(--ease-out);
}

.about__email svg {
  width: 20px;
  height: 20px;
}

.about__email:hover {
  color: var(--color-accent);
}

.about__github {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--color-ink);
  text-decoration: none;
  font-size: 0.9375rem;
  font-weight: 500;
  transition: color 0.2s var(--ease-out);
}

.about__github svg {
  width: 20px;
  height: 20px;
}

.about__github:hover {
  color: var(--color-accent);
}
</style>
