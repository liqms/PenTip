import { invoke } from '@tauri-apps/api/core'
import type { ProjectRow } from '../types/project'

export async function createProject(title: string, projectType: string): Promise<ProjectRow> {
  return invoke('create_project', { title, projectType })
}

export async function listProjects(): Promise<ProjectRow[]> {
  return invoke('list_projects')
}

export async function updateProject(id: string, title: string): Promise<ProjectRow> {
  return invoke('update_project', { id, title })
}

export async function deleteProject(id: string): Promise<void> {
  return invoke('delete_project', { id })
}
