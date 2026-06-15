import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ProjectRow } from '../types/project'
import * as api from '../api/project'

export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<ProjectRow[]>([])
  const loading = ref(false)

  async function loadAll() {
    loading.value = true
    try {
      projects.value = await api.listProjects()
    } finally {
      loading.value = false
    }
  }

  async function addProject(title: string, type: ProjectRow['type']) {
    const row = await api.createProject(title, type)
    projects.value.unshift(row)
  }

  async function removeProject(id: string) {
    await api.deleteProject(id)
    const idx = projects.value.findIndex(p => p.id === id)
    if (idx > -1) projects.value.splice(idx, 1)
  }

  async function updateProject(id: string, title: string) {
    const row = await api.updateProject(id, title)
    const idx = projects.value.findIndex(p => p.id === id)
    if (idx > -1) projects.value.splice(idx, 1, row)
  }

  return { projects, loading, loadAll, addProject, removeProject, updateProject }
})