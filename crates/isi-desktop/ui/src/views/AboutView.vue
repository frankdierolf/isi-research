<script setup lang="ts">
const version = '0.1.0'
</script>

<template>
  <div class="view">
    <div class="about-logo">
      <span class="wordmark">ISI</span>
    </div>
    <p class="version">Version {{ version }}</p>
    <p class="description">Voice-controlled image manipulation powered by AI.</p>
    <div class="links">
      <a href="https://isi-research.org" target="_blank" class="link">Website</a>
      <a href="https://github.com/anthropics/isi-research" target="_blank" class="link">GitHub</a>
    </div>
  </div>
</template>

<style scoped>
.view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 40px 0;
}

.about-logo {
  margin-bottom: 16px;
}

.wordmark {
  font-family: 'Cormorant Garamond', Georgia, serif;
  font-size: 48px;
  font-weight: 500;
  color: var(--color-ink);
  letter-spacing: 0.05em;
}

.version {
  font-family: 'IBM Plex Mono', monospace;
  font-size: 12px;
  color: var(--color-muted);
  margin: 0 0 16px 0;
}

.description {
  font-size: 14px;
  color: var(--color-muted);
  margin: 0 0 24px 0;
  max-width: 240px;
}

.links {
  display: flex;
  gap: 16px;
}

.link {
  font-size: 13px;
  color: var(--color-ink);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.link:hover {
  text-decoration: none;
}
</style>
