<script setup lang="ts">
const emit = defineEmits<{
  navigate: [sectionId: string]
}>()
</script>

<template>
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
        <button class="btn btn--primary" @click="emit('navigate', 'research')">
          View Research
          <svg class="btn__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M7 17L17 7M17 7H7M17 7V17" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <button class="btn btn--secondary" @click="emit('navigate', 'download')">
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
</template>

<style scoped>
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

/* Stagger children animations */
.hero__badge.is-visible { transition-delay: 0.1s; }
.hero__title.is-visible { transition-delay: 0.2s; }
.hero__subtitle.is-visible { transition-delay: 0.3s; }
.hero__cta.is-visible { transition-delay: 0.4s; }

@media (max-width: 768px) {
  .hero__title {
    font-size: 2.5rem;
  }

  .hero__decoration {
    display: none;
  }
}
</style>
