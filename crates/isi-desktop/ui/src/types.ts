/**
 * Application configuration from Tauri backend
 */
export interface Config {
  deepgram_api_key: string | null
  gemini_api_key: string | null
  shortcut: string
  mode: 'voice' | 'manual'
  manual_prompt: string
}

/**
 * Parsed keyboard shortcut key for display
 */
export interface ShortcutKey {
  symbol: string
  label: string
}
