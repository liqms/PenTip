export interface Tag {
  id: string
  name: string
  created_at: number
}

export interface TagRelation {
  tag_id: string
  target_type: 'fragment' | 'project' | 'page'
  target_id: string
  created_at: number
}