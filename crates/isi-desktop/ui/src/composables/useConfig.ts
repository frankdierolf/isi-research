import { ref, readonly, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Config } from '../types'

function debounce<T extends (...args: any[]) => any>(fn: T, ms: number) {
  let timeoutId: ReturnType<typeof setTimeout>
  return (...args: Parameters<T>) => {
    clearTimeout(timeoutId)
    timeoutId = setTimeout(() => fn(...args), ms)
  }
}

/**
 * Composable for managing application configuration
 * Handles loading, saving, and state management for settings
 */
export function useConfig() {
  const deepgramKey = ref('')
  const geminiKey = ref('')
  const shortcut = ref('Cmd+Shift+I')
  const mode = ref<'voice' | 'manual'>('voice')
  const manualPrompt = ref('make it black and white')

  const loading = ref(false)
  const saving = ref(false)
  const message = ref('')
  const hasError = ref(false)
  const initialized = ref(false)

  async function load() {
    loading.value = true
    try {
      const config = await invoke<Config>('get_config')
      deepgramKey.value = config.deepgram_api_key || ''
      geminiKey.value = config.gemini_api_key || ''
      shortcut.value = config.shortcut || 'Cmd+Shift+I'
      mode.value = config.mode === 'manual' ? 'manual' : 'voice'
      manualPrompt.value = config.manual_prompt || 'make it black and white'
      initialized.value = true
    } catch (error) {
      console.error('Failed to load config:', error)
      hasError.value = true
      message.value = 'Failed to load settings'
    } finally {
      loading.value = false
    }
  }

  async function save() {
    saving.value = true
    message.value = ''
    hasError.value = false

    try {
      await invoke('save_config', {
        configDto: {
          deepgram_api_key: deepgramKey.value || null,
          gemini_api_key: geminiKey.value || null,
          shortcut: shortcut.value,
          mode: mode.value,
          manual_prompt: manualPrompt.value,
        }
      })
      message.value = 'Settings saved. Restart app for shortcut changes.'
      setTimeout(() => { message.value = '' }, 3000)
    } catch (error) {
      hasError.value = true
      message.value = `Error: ${error}`
    } finally {
      saving.value = false
    }
  }

  const debouncedSave = debounce(save, 500)

  watch(
    [deepgramKey, geminiKey, shortcut, mode, manualPrompt],
    () => {
      if (initialized.value) {
        debouncedSave()
      }
    },
    { deep: true }
  )

  return {
    // State
    deepgramKey,
    geminiKey,
    shortcut,
    mode,
    manualPrompt,

    // Status
    loading: readonly(loading),
    saving: readonly(saving),
    message: readonly(message),
    hasError: readonly(hasError),

    // Actions
    load,
    save,
  }
}
