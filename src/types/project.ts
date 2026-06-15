/// Creative project types.
export type ProjectContentType = 'novel' | 'article' | 'script' | 'image-text' | 'video' | 'audio'

/// Project lifecycle status.
export type ProjectStatus = 'in-progress' | 'completed'

export interface ProjectRow {
  id: string
  title: string
  description?: string
  type: ProjectContentType
  status: ProjectStatus
  metadata: Record<string, unknown>
  created_at: number
  updated_at: number
  deleted_at: number | null
}