export interface Fragment {
  id: string
  content: string
  type: 'idea' | 'todo' | 'memo' | 'diary' | 'work-log'
  source?: string
  created_at: number
  updated_at: number
  deleted_at: number | null
}

export interface Project {
  id: string
  title: string
  description?: string
  type: 'novel' | 'article' | 'script' | 'image-text' | 'video' | 'audio'
  status: 'in-progress' | 'completed'
  metadata: Record<string, unknown>
  created_at: number
  updated_at: number
  deleted_at: number | null
}

export interface Page {
  id: string
  project_id: string
  title: string
  content: string
  order: number
  created_at: number
  updated_at: number
  deleted_at: number | null
}

export interface FragmentRelation {
  id: string
  fragment_id: string
  target_type: 'project' | 'page'
  target_id: string
  created_at: number
}
