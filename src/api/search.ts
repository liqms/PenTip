import { invoke } from '@tauri-apps/api/core'
import type { SearchResultRow } from '../types/fragment'

export async function searchFragments(query: string): Promise<SearchResultRow[]> {
  return invoke('search_fragments', { query })
}
