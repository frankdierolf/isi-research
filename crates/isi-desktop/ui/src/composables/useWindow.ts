import { getCurrentWindow } from '@tauri-apps/api/window'

/**
 * Composable for window management
 */
export function useWindow() {
  async function close() {
    const window = getCurrentWindow()
    await window.hide()
  }

  return {
    close,
  }
}
