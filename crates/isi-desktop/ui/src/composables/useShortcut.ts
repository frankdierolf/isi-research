import { ref, computed, onUnmounted, type Ref } from 'vue'
import type { ShortcutKey } from '../types'

/**
 * Maps key names to macOS keyboard symbols
 */
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

/**
 * Composable for recording and displaying keyboard shortcuts
 */
export function useShortcut(shortcutRef: Ref<string>) {
  const recording = ref(false)

  const keys = computed<ShortcutKey[]>(() => {
    if (!shortcutRef.value) return []

    return shortcutRef.value.split('+').map(part => {
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
      shortcutRef.value = parts.join('+')
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

  return {
    recording,
    keys,
    startRecording,
    stopRecording,
  }
}
