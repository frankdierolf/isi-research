<script setup lang="ts">
import { inject } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import ModeCard from '../components/ModeCard.vue'
import FormField from '../components/FormField.vue'
import TextInput from '../components/TextInput.vue'
import ShortcutRecorder from '../components/ShortcutRecorder.vue'
import type { useConfig } from '../composables/useConfig'

const config = inject<ReturnType<typeof useConfig>>('config')!

const openExternal = (url: string) => {
  openUrl(url)
}
</script>

<template>
  <div class="view">
    <!-- Mode Selection -->
    <section class="section">
      <h2 class="section-title">Mode</h2>
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
    </section>

    <hr class="divider" />

    <!-- Configuration -->
    <section class="section">
      <h2 class="section-title">Configuration</h2>

      <FormField label="Keyboard Shortcut">
        <template #description>
          Global shortcut to trigger {{ config.mode.value === 'voice' ? 'voice recording' : 'image transformation' }}
        </template>
        <ShortcutRecorder
          v-model="config.shortcut.value"
          hint="Click to change"
        />
      </FormField>

      <!-- Usage Instructions -->
      <div class="usage-instructions">
        <h3 class="usage-title">How to use</h3>
        <ol v-if="config.mode.value === 'voice'" class="usage-list">
          <li>Copy an image to your clipboard</li>
          <li>Press the shortcut to start recording</li>
          <li>Speak your transformation instructions</li>
          <li>Press the shortcut again to submit</li>
        </ol>
        <ol v-else class="usage-list">
          <li>Copy an image to your clipboard</li>
          <li>Press the shortcut to transform</li>
        </ol>
      </div>

      <FormField
        v-if="config.mode.value === 'voice'"
        label="Deepgram API Key"
        html-for="deepgram"
      >
        <template #description>
          For voice transcription. Get your key at
          <a href="#" @click.prevent="openExternal('https://console.deepgram.com/')">console.deepgram.com</a>
        </template>
        <TextInput
          id="deepgram"
          v-model="config.deepgramKey.value"
          type="password"
          placeholder="Enter your Deepgram API key"
        />
      </FormField>

      <FormField label="Gemini API Key" html-for="gemini">
        <template #description>
          For image transformation. Get your key at
          <a href="#" @click.prevent="openExternal('https://aistudio.google.com/apikey')">aistudio.google.com</a>
        </template>
        <TextInput
          id="gemini"
          v-model="config.geminiKey.value"
          type="password"
          placeholder="Enter your Gemini API key"
        />
      </FormField>
    </section>
  </div>
</template>

<style scoped>
.view {
  /* View container */
}

.section {
  margin-bottom: 8px;
}

.section-title {
  font-family: 'Cormorant Garamond', serif;
  font-size: 18px;
  font-weight: 500;
  margin: 0 0 8px 0;
  color: var(--color-ink);
}

.description {
  font-size: 13px;
  color: var(--color-muted);
  margin: 0 0 16px 0;
}

.mode-cards {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-bottom: 16px;
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

.divider {
  border: none;
  border-top: 1px solid var(--color-border);
  margin: 24px 0;
}

.usage-instructions {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 6px;
  padding: 16px;
  margin: 16px 0;
}

.usage-title {
  font-size: 13px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: var(--color-ink);
}

.usage-list {
  margin: 0;
  padding-left: 20px;
  font-size: 13px;
  color: var(--color-muted);
}

.usage-list li {
  margin-bottom: 6px;
}

.usage-list li:last-child {
  margin-bottom: 0;
}
</style>
