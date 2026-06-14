import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Fragment } from '../types/note'

export const useNotesStore = defineStore('notes', () => {
  const notes = ref<Fragment[]>([])

  function addNote(note: Fragment) { notes.value.push(note) }
  function removeNote(id: string) {
    const idx = notes.value.findIndex(n => n.id === id)
    if (idx > -1) notes.value.splice(idx, 1)
  }

  return { notes, addNote, removeNote }
})
