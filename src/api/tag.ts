import { invoke } from '@tauri-apps/api/core'
import type { TagRow } from '../types/tag'

export async function createTag(name: string): Promise<TagRow> {
  return invoke('create_tag', { name })
}

export async function listTags(): Promise<TagRow[]> {
  return invoke('list_tags')
}

export async function deleteTag(id: string): Promise<void> {
  return invoke('delete_tag', { id })
}

export async function attachTag(tagId: string, targetType: string, targetId: string): Promise<void> {
  return invoke('attach_tag', { tagId, targetType, targetId })
}

export async function detachTag(tagId: string, targetType: string, targetId: string): Promise<void> {
  return invoke('detach_tag', { tagId, targetType, targetId })
}

export async function listTagsForTarget(targetType: string, targetId: string): Promise<TagRow[]> {
  return invoke('list_tags_for_target', { targetType, targetId })
}