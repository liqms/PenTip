import { onMounted, onUnmounted } from 'vue'

export function useShortcuts(handlers: Record<string, () => void>) {
  function onKeydown(e: KeyboardEvent) {
    const key = [e.ctrlKey || e.metaKey ? 'Ctrl' : '', e.shiftKey ? 'Shift' : '', e.altKey ? 'Alt' : '', e.key.toUpperCase()]
      .filter(Boolean)
      .join('+')
    if (handlers[key]) handlers[key]()
  }

  onMounted(() => window.addEventListener('keydown', onKeydown))
  onUnmounted(() => window.removeEventListener('keydown', onKeydown))
}
