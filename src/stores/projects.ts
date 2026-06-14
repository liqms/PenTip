import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Project } from '../types/note'

export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])

  function addProject(p: Project) { projects.value.push(p) }
  function removeProject(id: string) {
    const idx = projects.value.findIndex(p => p.id === id)
    if (idx > -1) projects.value.splice(idx, 1)
  }

  return { projects, addProject, removeProject }
})
