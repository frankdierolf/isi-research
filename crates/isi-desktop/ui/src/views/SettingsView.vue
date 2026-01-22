<script setup lang="ts">
import { inject } from 'vue'
import FormField from '../components/FormField.vue'
import TextInput from '../components/TextInput.vue'
import ShortcutRecorder from '../components/ShortcutRecorder.vue'
import type { useConfig } from '../composables/useConfig'

const config = inject<ReturnType<typeof useConfig>>('config')!
</script>

<template>
  <div class="view">
    <FormField
      v-if="config.mode.value === 'voice'"
      label="Deepgram API Key"
      html-for="deepgram"
    >
      <template #description>
        For voice transcription. Get your key at
        <a href="https://console.deepgram.com/" target="_blank">console.deepgram.com</a>
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
        <a href="https://aistudio.google.com/apikey" target="_blank">aistudio.google.com</a>
      </template>
      <TextInput
        id="gemini"
        v-model="config.geminiKey.value"
        type="password"
        placeholder="Enter your Gemini API key"
      />
    </FormField>

    <FormField label="Keyboard Shortcut">
      <template #description>
        Global shortcut to trigger {{ config.mode.value === 'voice' ? 'voice recording' : 'image transformation' }}
      </template>
      <ShortcutRecorder
        v-model="config.shortcut.value"
        hint="Click to change"
      />
    </FormField>
  </div>
</template>

<style scoped>
.view {
  /* View container */
}
</style>
