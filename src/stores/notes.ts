import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { FragmentRow } from '../types/fragment'
import * as api from '../api/fragment'

export const useNotesStore = defineStore('notes', () => {
  const notes = ref<FragmentRow[]>([])
  const loading = ref(false)

  async function loadAll() {
    loading.value = true
    try {
      notes.value = await api.listFragments()
    } finally {
      loading.value = false
    }
  }

  async function addNote(content: string, type: FragmentRow['type']) {
    const row = await api.createFragment(content, type)
    notes.value.unshift(row)
  }

  async function removeNote(id: string) {
    await api.deleteFragment(id)
    const idx = notes.value.findIndex(n => n.id === id)
    if (idx > -1) notes.value.splice(idx, 1)
  }

  async function updateNote(id: string, content: string) {
    const row = await api.updateFragment(id, content)
    const idx = notes.value.findIndex(n => n.id === id)
    if (idx > -1) notes.value.splice(idx, 1, row)
  }

  return { notes, loading, loadAll, addNote, removeNote, updateNote }
})