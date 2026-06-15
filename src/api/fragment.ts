import { invoke } from '@tauri-apps/api/core'
import type { FragmentRow } from '../types/fragment'

export async function createFragment(content: string, noteType: string): Promise<FragmentRow> {
  return invoke('create_fragment', { content, noteType })
}

export async function listFragments(): Promise<FragmentRow[]> {
  return invoke('list_fragments')
}

export async function getFragment(id: string): Promise<FragmentRow> {
  return invoke('get_fragment', { id })
}

export async function updateFragment(id: string, content: string): Promise<FragmentRow> {
  return invoke('update_fragment', { id, content })
}

export async function deleteFragment(id: string): Promise<void> {
  return invoke('delete_fragment', { id })
}
