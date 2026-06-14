// FTS5 全文搜索服务
import { getDb } from './sqlite'

export async function searchFragments(query: string) {
  const db = await getDb()
  const results = await db.select<{ id: string; content: string }[]>(
    `SELECT f.id, f.content FROM fragments f
     INNER JOIN fragments_fts fts ON f.id = fts.rowid
     WHERE fragments_fts MATCH $1 AND f.deleted_at IS NULL
     ORDER BY rank LIMIT 50`,
    [query]
  )
  return results
}
