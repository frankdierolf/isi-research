<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import KeyBadge from './KeyBadge.vue'
import type { ShortcutKey } from '../types'

const KEY_SYMBOLS: Record<string, { symbol: string; label: string }> = {
  cmd: { symbol: '\u2318', label: 'Command' },
  meta: { symbol: '\u2318', label: 'Command' },
  command: { symbol: '\u2318', label: 'Command' },
  ctrl: { symbol: '\u2303', label: 'Control' },
  control: { symbol: '\u2303', label: 'Control' },
  alt: { symbol: '\u2325', label: 'Option' },
  option: { symbol: '\u2325', label: 'Option' },
  shift: { symbol: '\u21E7', label: 'Shift' },
}

const props = defineProps<{
  modelValue: string
  hint?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const recording = ref(false)

const keys = computed<ShortcutKey[]>(() => {
  if (!props.modelValue) return []

  return props.modelValue.split('+').map(part => {
    const lower = part.toLowerCase()
    const mapped = KEY_SYMBOLS[lower]
    if (mapped) return mapped
    return { symbol: part.toUpperCase(), label: part }
  })
})

function handleKeydown(event: KeyboardEvent) {
  event.preventDefault()
  event.stopPropagation()

  // Ignore modifier-only presses
  if (['Meta', 'Control', 'Alt', 'Shift'].includes(event.key)) {
    return
  }

  const parts: string[] = []
  if (event.metaKey) parts.push('Cmd')
  if (event.ctrlKey) parts.push('Ctrl')
  if (event.altKey) parts.push('Alt')
  if (event.shiftKey) parts.push('Shift')

  if (event.key) {
    parts.push(event.key.toUpperCase())
  }

  if (parts.length > 0) {
    emit('update:modelValue', parts.join('+'))
  }

  stopRecording()
}

function startRecording() {
  recording.value = true
  window.addEventListener('keydown', handleKeydown)
}

function stopRecording() {
  recording.value = false
  window.removeEventListener('keydown', handleKeydown)
}

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="shortcut-recorder">
    <button
      class="shortcut-display"
      :class="{ recording }"
      @click="startRecording"
    >
      <template v-if="recording">
        <span class="recording-text">Press keys...</span>
      </template>
      <template v-else-if="keys.length">
        <KeyBadge
          v-for="(key, i) in keys"
          :key="i"
          :symbol="key.symbol"
          :label="key.label"
        />
      </template>
      <span v-else>Click to record</span>
    </button>
    <span v-if="hint" class="hint">{{ hint }}</span>
  </div>
</template>

<style scoped>
.shortcut-recorder {
  display: flex;
  align-items: center;
  gap: 12px;
}

.shortcut-display {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  min-width: 120px;
  padding: 8px 12px;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
  font-size: 16px;
  color: var(--color-ink);
  background: var(--color-paper);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s;
}

.shortcut-display:hover {
  border-color: var(--color-ink);
}

.shortcut-display.recording {
  border-color: var(--color-ink);
  box-shadow: 0 0 0 3px rgba(26, 26, 26, 0.1);
  animation: pulse 1s infinite;
}

.recording-text {
  font-family: 'IBM Plex Sans', sans-serif;
  font-size: 14px;
  color: var(--color-muted);
}

.hint {
  font-size: 13px;
  color: var(--color-muted);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}
</style>
