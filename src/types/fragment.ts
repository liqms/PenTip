/// Business scenario types for fragment notes.
export type FragmentType = 'idea' | 'todo' | 'memo' | 'diary' | 'work-log'

/// Target entity types for fragment relations.
export type FragmentTargetType = 'project' | 'page'

export interface FragmentRow {
  id: string
  content: string
  type: FragmentType
  source?: string
  created_at: number
  updated_at: number
  deleted_at: number | null
}

export interface FragmentRelation {
  id: string
  fragment_id: string
  target_type: FragmentTargetType
  target_id: string
  created_at: number
}

export interface SearchResultRow {
  id: string
  content: string
}