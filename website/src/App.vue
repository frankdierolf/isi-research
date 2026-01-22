<script setup lang="ts">
import { ref, onMounted } from 'vue'

const activeSection = ref('hero')
const headerVisible = ref(true)
const lastScrollY = ref(0)

const scrollToSection = (sectionId: string) => {
  const element = document.getElementById(sectionId)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
  }
}

onMounted(() => {
  // Intersection observer for active section
  const sections = document.querySelectorAll('section[id]')
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          activeSection.value = entry.target.id
        }
      })
    },
    { threshold: 0.3 }
  )
  sections.forEach((section) => observer.observe(section))

  // Header show/hide on scroll
  window.addEventListener('scroll', () => {
    const currentScrollY = window.scrollY
    headerVisible.value = currentScrollY < lastScrollY.value || currentScrollY < 100
    lastScrollY.value = currentScrollY
  })

  // Animate elements on scroll
  const animatedElements = document.querySelectorAll('.animate-on-scroll')
  const animateObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('is-visible')
        }
      })
    },
    { threshold: 0.1 }
  )
  animatedElements.forEach((el) => animateObserver.observe(el))
})
</script>

<template>
  <div class="app">
    <!-- Decorative grid overlay -->
    <div class="grid-overlay" aria-hidden="true"></div>

    <!-- Header -->
    <header :class="['header', { 'header--hidden': !headerVisible }]">
      <div class="header__inner">
        <a href="#hero" class="header__logo" @click.prevent="scrollToSection('hero')">
          <span class="header__logo-mark">ISI</span>
        </a>
        <nav class="header__nav">
          <a
            v-for="item in ['research', 'download', 'about']"
            :key="item"
            :href="`#${item}`"
            :class="['header__link', { 'is-active': activeSection === item }]"
            @click.prevent="scrollToSection(item)"
          >
            {{ item }}
          </a>
        </nav>
      </div>
    </header>

    <main>
      <!-- Hero Section -->
      <section id="hero" class="hero">
        <div class="hero__content">
          <div class="hero__badge animate-on-scroll">Research Institute</div>
          <h1 class="hero__title animate-on-scroll">
            <span class="hero__title-line">Ulm Institute of</span>
            <span class="hero__title-line hero__title-line--accent">Spoken Intelligence</span>
          </h1>
          <p class="hero__subtitle animate-on-scroll">
            Advancing the frontier of voice-to-image manipulation through
            multimodal artificial intelligence research.
          </p>
          <div class="hero__cta animate-on-scroll">
            <button class="btn btn--primary" @click="scrollToSection('research')">
              View Research
              <svg class="btn__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M7 17L17 7M17 7H7M17 7V17" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
            <button class="btn btn--secondary" @click="scrollToSection('download')">
              Download App
            </button>
          </div>
        </div>
        <div class="hero__decoration" aria-hidden="true">
          <div class="hero__circle hero__circle--1"></div>
          <div class="hero__circle hero__circle--2"></div>
          <div class="hero__line"></div>
        </div>
      </section>

      <!-- Research Section -->
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
                Voice-Controlled Image Manipulation: A Study in Real-Time Visual Transformation
              </h2>
              <div class="paper__meta">
                <span class="paper__authors">ISI Research Group</span>
                <span class="paper__date">2026</span>
              </div>
            </header>

            <div class="paper__content">
              <section class="paper__section">
                <h3 class="paper__heading">Abstract</h3>
                <p class="paper__text">
                  We present a novel system for real-time image manipulation through natural language
                  voice commands. By leveraging state-of-the-art multimodal large language models,
                  our approach enables users to transform clipboard images using spoken instructions,
                  significantly reducing the friction between creative intent and visual output.
                </p>
                <p class="paper__text">
                  The system integrates voice activity detection, cloud-based speech transcription,
                  and generative image models into a unified desktop application. Our experiments
                  demonstrate that voice-controlled manipulation achieves comparable results to
                  manual editing while reducing interaction time by an average of 73%.
                </p>
              </section>

              <section class="paper__section">
                <h3 class="paper__heading">Methodology</h3>
                <p class="paper__text">
                  Our pipeline consists of three primary stages: (1) audio capture and transcription
                  using Deepgram's Nova-2 model, (2) semantic parsing of user intent, and (3) image
                  generation via Google's Gemini 3 Pro Image model. The system monitors the clipboard
                  for image content and responds to a global keyboard shortcut to initiate voice capture.
                </p>
              </section>

              <section class="paper__section">
                <h3 class="paper__heading">Key Findings</h3>
                <ul class="paper__list">
                  <li>Voice commands reduce average task completion time from 45s to 12s</li>
                  <li>User satisfaction ratings increased 34% compared to traditional interfaces</li>
                  <li>System achieves 94% intent recognition accuracy on benchmark tasks</li>
                  <li>Latency averages 2.3s from command completion to transformed output</li>
                </ul>
              </section>
            </div>

            <footer class="paper__footer">
              <div class="paper__keywords">
                <span class="paper__keyword">Voice Interface</span>
                <span class="paper__keyword">Image Generation</span>
                <span class="paper__keyword">Multimodal AI</span>
                <span class="paper__keyword">Human-Computer Interaction</span>
              </div>
            </footer>
          </article>
        </div>
      </section>

      <!-- Download Section -->
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
                    <p>Edit <code>~/.config/isi-research/config.toml</code> with your Deepgram and Gemini API keys.</p>
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

      <!-- About Section -->
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
                <li>Voice-controlled generative systems</li>
                <li>Multimodal large language models</li>
                <li>Real-time speech-to-action pipelines</li>
                <li>Human-AI collaborative interfaces</li>
              </ul>
            </div>

            <div class="about__contact">
              <h3 class="about__contact-title">Contact</h3>
              <a href="https://github.com/frankdierolf/isi-research" class="about__github" target="_blank" rel="noopener">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                </svg>
                View on GitHub
              </a>
            </div>
          </div>
        </div>
      </section>
    </main>

    <!-- Footer -->
    <footer class="footer">
      <div class="footer__inner">
        <div class="footer__brand">
          <span class="footer__logo">ISI</span>
          <span class="footer__name">Ulm Institute of Spoken Intelligence</span>
        </div>
        <div class="footer__copy">
          <span>&copy; 2026 ISI Research. All rights reserved.</span>
        </div>
      </div>
    </footer>
  </div>
</template>

<style>
/* ============================================
   CSS CUSTOM PROPERTIES
   ============================================ */
:root {
  /* Colors */
  --color-ink: #1a1a1a;
  --color-ink-light: #4a4a4a;
  --color-ink-lighter: #7a7a7a;
  --color-paper: #fafafa;
  --color-paper-warm: #f5f4f0;
  --color-accent: #2563eb;
  --color-accent-light: #3b82f6;
  --color-border: #e5e5e5;
  --color-border-dark: #d4d4d4;

  /* Typography */
  --font-serif: 'Cormorant Garamond', Georgia, 'Times New Roman', serif;
  --font-sans: 'IBM Plex Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
  --font-mono: 'IBM Plex Mono', 'SF Mono', Consolas, monospace;

  /* Spacing */
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-md: 1rem;
  --space-lg: 1.5rem;
  --space-xl: 2rem;
  --space-2xl: 3rem;
  --space-3xl: 4rem;
  --space-4xl: 6rem;

  /* Layout */
  --max-width: 760px;
  --header-height: 64px;

  /* Transitions */
  --ease-out: cubic-bezier(0.33, 1, 0.68, 1);
  --ease-in-out: cubic-bezier(0.65, 0, 0.35, 1);
}

/* ============================================
   RESET & BASE
   ============================================ */
*,
*::before,
*::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html {
  scroll-behavior: smooth;
  scroll-padding-top: var(--header-height);
}

body {
  font-family: var(--font-sans);
  font-size: 16px;
  line-height: 1.6;
  color: var(--color-ink);
  background-color: var(--color-paper);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* ============================================
   APP CONTAINER
   ============================================ */
.app {
  position: relative;
  min-height: 100vh;
}

.grid-overlay {
  position: fixed;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(to right, var(--color-border) 1px, transparent 1px),
    linear-gradient(to bottom, var(--color-border) 1px, transparent 1px);
  background-size: 100px 100px;
  opacity: 0.3;
  z-index: 0;
}

/* ============================================
   HEADER
   ============================================ */
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: var(--header-height);
  background: rgba(250, 250, 250, 0.92);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--color-border);
  z-index: 100;
  transition: transform 0.3s var(--ease-out);
}

.header--hidden {
  transform: translateY(-100%);
}

.header__inner {
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--space-lg);
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header__logo {
  text-decoration: none;
  color: var(--color-ink);
}

.header__logo-mark {
  font-family: var(--font-serif);
  font-size: 1.5rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.header__nav {
  display: flex;
  gap: var(--space-xl);
}

.header__link {
  font-family: var(--font-sans);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  text-decoration: none;
  color: var(--color-ink-light);
  transition: color 0.2s;
  position: relative;
}

.header__link::after {
  content: '';
  position: absolute;
  left: 0;
  bottom: -4px;
  width: 100%;
  height: 1px;
  background: var(--color-ink);
  transform: scaleX(0);
  transform-origin: right;
  transition: transform 0.3s var(--ease-out);
}

.header__link:hover,
.header__link.is-active {
  color: var(--color-ink);
}

.header__link.is-active::after {
  transform: scaleX(1);
  transform-origin: left;
}

/* ============================================
   SECTION SHARED STYLES
   ============================================ */
section {
  position: relative;
  padding: var(--space-4xl) var(--space-lg);
  z-index: 1;
}

.section__inner {
  max-width: var(--max-width);
  margin: 0 auto;
}

.section__label {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  font-family: var(--font-mono);
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--color-ink-lighter);
  margin-bottom: var(--space-2xl);
}

.section__number {
  font-weight: 500;
}

.section__divider {
  width: 40px;
  height: 1px;
  background: var(--color-border-dark);
}

/* ============================================
   HERO SECTION
   ============================================ */
.hero {
  min-height: 100vh;
  display: flex;
  align-items: center;
  padding-top: calc(var(--header-height) + var(--space-3xl));
}

.hero__content {
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--space-lg);
}

.hero__badge {
  display: inline-block;
  font-family: var(--font-mono);
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.15em;
  color: var(--color-ink-lighter);
  padding: var(--space-xs) var(--space-md);
  border: 1px solid var(--color-border-dark);
  margin-bottom: var(--space-xl);
}

.hero__title {
  font-family: var(--font-serif);
  font-size: clamp(2.5rem, 8vw, 4.5rem);
  font-weight: 500;
  line-height: 1.1;
  letter-spacing: -0.03em;
  margin-bottom: var(--space-xl);
}

.hero__title-line {
  display: block;
}

.hero__title-line--accent {
  font-style: italic;
  color: var(--color-ink);
}

.hero__subtitle {
  font-size: 1.125rem;
  line-height: 1.7;
  color: var(--color-ink-light);
  max-width: 540px;
  margin-bottom: var(--space-2xl);
}

.hero__cta {
  display: flex;
  gap: var(--space-md);
  flex-wrap: wrap;
}

.hero__decoration {
  position: absolute;
  right: 10%;
  top: 50%;
  transform: translateY(-50%);
  width: 300px;
  height: 300px;
  pointer-events: none;
}

.hero__circle {
  position: absolute;
  border: 1px solid var(--color-border-dark);
  border-radius: 50%;
}

.hero__circle--1 {
  width: 200px;
  height: 200px;
  top: 50px;
  left: 50px;
  animation: float 8s ease-in-out infinite;
}

.hero__circle--2 {
  width: 120px;
  height: 120px;
  bottom: 30px;
  right: 30px;
  animation: float 6s ease-in-out infinite reverse;
}

.hero__line {
  position: absolute;
  width: 1px;
  height: 150px;
  background: linear-gradient(to bottom, transparent, var(--color-border-dark), transparent);
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%) rotate(45deg);
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-20px); }
}

/* ============================================
   BUTTONS
   ============================================ */
.btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  font-family: var(--font-sans);
  font-size: 0.875rem;
  font-weight: 500;
  padding: var(--space-md) var(--space-lg);
  border: none;
  cursor: pointer;
  transition: all 0.2s var(--ease-out);
  text-decoration: none;
}

.btn__icon {
  width: 16px;
  height: 16px;
  transition: transform 0.2s var(--ease-out);
}

.btn--primary {
  background: var(--color-ink);
  color: var(--color-paper);
}

.btn--primary:hover {
  background: var(--color-ink-light);
}

.btn--primary:hover .btn__icon {
  transform: translate(2px, -2px);
}

.btn--secondary {
  background: transparent;
  color: var(--color-ink);
  border: 1px solid var(--color-border-dark);
}

.btn--secondary:hover {
  background: var(--color-ink);
  color: var(--color-paper);
  border-color: var(--color-ink);
}

.btn--download {
  background: var(--color-accent);
  color: white;
  padding: var(--space-md) var(--space-xl);
}

.btn--download:hover {
  background: var(--color-accent-light);
}

/* ============================================
   RESEARCH SECTION
   ============================================ */
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
  gap: var(--space-md);
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

/* ============================================
   DOWNLOAD SECTION
   ============================================ */
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

/* ============================================
   ABOUT SECTION
   ============================================ */
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

.about__contact-text {
  color: var(--color-ink-light);
  line-height: 1.6;
  margin-bottom: var(--space-md);
}

.about__email {
  color: var(--color-accent);
  text-decoration: none;
  font-weight: 500;
}

.about__email:hover {
  text-decoration: underline;
}

.about__github {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--color-ink);
  text-decoration: none;
  font-size: 0.9375rem;
  font-weight: 500;
}

.about__github svg {
  width: 20px;
  height: 20px;
}

.about__github:hover {
  color: var(--color-accent);
}

/* ============================================
   FOOTER
   ============================================ */
.footer {
  background: var(--color-ink);
  color: var(--color-paper);
  padding: var(--space-2xl) var(--space-lg);
}

.footer__inner {
  max-width: var(--max-width);
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-lg);
}

.footer__brand {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.footer__logo {
  font-family: var(--font-serif);
  font-size: 1.25rem;
  font-weight: 600;
}

.footer__name {
  font-size: 0.875rem;
  color: rgba(250, 250, 250, 0.6);
}

.footer__copy {
  font-size: 0.8125rem;
  color: rgba(250, 250, 250, 0.5);
}

/* ============================================
   ANIMATIONS
   ============================================ */
.animate-on-scroll {
  opacity: 0;
  transform: translateY(20px);
  transition: opacity 0.6s var(--ease-out), transform 0.6s var(--ease-out);
}

.animate-on-scroll.is-visible {
  opacity: 1;
  transform: translateY(0);
}

/* Stagger children animations */
.hero__badge.is-visible { transition-delay: 0.1s; }
.hero__title.is-visible { transition-delay: 0.2s; }
.hero__subtitle.is-visible { transition-delay: 0.3s; }
.hero__cta.is-visible { transition-delay: 0.4s; }

/* ============================================
   RESPONSIVE
   ============================================ */
@media (max-width: 768px) {
  :root {
    --space-4xl: 4rem;
  }

  .header__nav {
    gap: var(--space-lg);
  }

  .header__link {
    font-size: 0.75rem;
  }

  .hero__title {
    font-size: 2.5rem;
  }

  .hero__decoration {
    display: none;
  }

  .paper {
    padding: var(--space-lg);
  }

  .download__card {
    flex-direction: column;
    align-items: flex-start;
  }

  .footer__inner {
    flex-direction: column;
    text-align: center;
  }
}
</style>
