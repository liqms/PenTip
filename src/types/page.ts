export interface PageRow {
  id: string
  project_id: string
  title: string
  content?: string
  order: number
  created_at: number
  updated_at: number
  deleted_at: number | null
}