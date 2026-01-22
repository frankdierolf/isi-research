<script setup lang="ts">
import { inject } from 'vue'
import ModeCard from '../components/ModeCard.vue'
import type { useConfig } from '../composables/useConfig'

const config = inject<ReturnType<typeof useConfig>>('config')!
</script>

<template>
  <div class="view">
    <p class="description">Choose how you want to trigger image transformations.</p>

    <div class="mode-cards">
      <ModeCard
        title="Voice"
        description="Speak your transformation instructions using your microphone."
        :selected="config.mode.value === 'voice'"
        @select="config.mode.value = 'voice'"
      >
        <template #icon>
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
            <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
            <line x1="12" y1="19" x2="12" y2="23"/>
            <line x1="8" y1="23" x2="16" y2="23"/>
          </svg>
        </template>
      </ModeCard>

      <ModeCard
        title="Manual"
        description="Use a predefined prompt for consistent transformations."
        :selected="config.mode.value === 'manual'"
        @select="config.mode.value = 'manual'"
      >
        <template #icon>
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"/>
          </svg>
        </template>
      </ModeCard>
    </div>

    <div v-if="config.mode.value === 'manual'" class="prompt-section">
      <label for="manual-prompt">Default Prompt</label>
      <p class="hint">This prompt will be used every time you press the shortcut.</p>
      <textarea
        id="manual-prompt"
        v-model="config.manualPrompt.value"
        placeholder="e.g., make it black and white"
        rows="3"
      />
    </div>
  </div>
</template>

<style scoped>
.view {
  /* View container */
}

.description {
  font-size: 13px;
  color: var(--color-muted);
  margin: 0 0 20px 0;
}

.mode-cards {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-bottom: 20px;
}

.prompt-section {
  margin-top: 8px;
}

.prompt-section label {
  display: block;
  font-weight: 500;
  margin-bottom: 4px;
  color: var(--color-ink);
}

.hint {
  font-size: 13px;
  color: var(--color-muted);
  margin: 0 0 8px 0;
}

.prompt-section textarea {
  width: 100%;
  padding: 10px 12px;
  font-family: 'IBM Plex Mono', monospace;
  font-size: 13px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: var(--color-paper);
  color: var(--color-ink);
  resize: vertical;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.prompt-section textarea:focus {
  outline: none;
  border-color: var(--color-ink);
  box-shadow: 0 0 0 3px rgba(26, 26, 26, 0.1);
}
</style>
