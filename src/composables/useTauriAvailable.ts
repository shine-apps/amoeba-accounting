// Check if we're running inside a real Tauri webview with the IPC bridge.
// When running via "pnpm dev" (browser only), this returns false.
export function useTauriAvailable(): boolean {
  if (!('__TAURI_INTERNALS__' in window)) return false
  const tauri = window.__TAURI_INTERNALS__ as Record<string, unknown>
  if (!tauri || !tauri.invoke) return false
  return !tauri.__isMock
}
