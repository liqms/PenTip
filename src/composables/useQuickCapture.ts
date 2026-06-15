import { ref } from 'vue'
import { generateId } from '../utils/format'
import type { FragmentRow } from '../types/fragment'

export function useQuickCapture() {
  const visible = ref(false)
  const content = ref('')

  function open() { visible.value = true }
  function close() { visible.value = false; content.value = '' }

  function save(): FragmentRow | null {
    if (!content.value.trim()) return null
    const note: FragmentRow = {
      id: generateId(),
      content: content.value,
      type: 'idea',
      created_at: Date.now(),
      updated_at: Date.now(),
      deleted_at: null,
    }
    content.value = ''
    visible.value = false
    return note
  }

  return { visible, content, open, close, save }
}
