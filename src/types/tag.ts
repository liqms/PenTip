/// Entity types that can have tags attached.
export type TagTargetType = 'fragment' | 'project' | 'page'

export interface TagRow {
  id: string
  name: string
  created_at: number
}

export interface TagRelation {
  tag_id: string
  target_type: TagTargetType
  target_id: string
  created_at: number
}