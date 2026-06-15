import { invoke } from '@tauri-apps/api/core'
import type { PageRow } from '../types/page'

export async function createPage(projectId: string, title: string): Promise<PageRow> {
  return invoke('create_page', { projectId, title })
}

export async function listPages(projectId: string): Promise<PageRow[]> {
  return invoke('list_pages', { projectId })
}

export async function updatePage(id: string, title: string, content: string): Promise<PageRow> {
  return invoke('update_page', { id, title, content })
}

export async function deletePage(id: string): Promise<void> {
  return invoke('delete_page', { id })
}
