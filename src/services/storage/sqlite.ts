// SQLite 数据库服务（通过 Tauri SQL 插件）
import Database from '@tauri-apps/plugin-sql'

let db: Database | null = null

export async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:pentip.db')
    await initSchema()
  }
  return db
}

async function initSchema() {
  const d = await getDb()
  await d.execute(`
    CREATE TABLE IF NOT EXISTS fragments (
      id TEXT PRIMARY KEY,
      content TEXT NOT NULL,
      type TEXT NOT NULL,
      source TEXT,
      created_at INTEGER NOT NULL,
      updated_at INTEGER NOT NULL,
      deleted_at INTEGER
    )
  `)
  await d.execute(`
    CREATE TABLE IF NOT EXISTS tags (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL UNIQUE,
      created_at INTEGER NOT NULL
    )
  `)
  await d.execute(`
    CREATE TABLE IF NOT EXISTS tag_relations (
      tag_id TEXT NOT NULL,
      target_type TEXT NOT NULL,
      target_id TEXT NOT NULL,
      created_at INTEGER NOT NULL,
      PRIMARY KEY (tag_id, target_type, target_id),
      FOREIGN KEY (tag_id) REFERENCES tags(id)
    )
  `)
  await d.execute(`
    CREATE TABLE IF NOT EXISTS projects (
      id TEXT PRIMARY KEY,
      title TEXT NOT NULL,
      description TEXT,
      type TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'in-progress',
      metadata TEXT DEFAULT '{}',
      created_at INTEGER NOT NULL,
      updated_at INTEGER NOT NULL,
      deleted_at INTEGER
    )
  `)
  await d.execute(`
    CREATE TABLE IF NOT EXISTS pages (
      id TEXT PRIMARY KEY,
      project_id TEXT NOT NULL,
      title TEXT NOT NULL,
      content TEXT DEFAULT '',
      "order" INTEGER NOT NULL DEFAULT 0,
      created_at INTEGER NOT NULL,
      updated_at INTEGER NOT NULL,
      deleted_at INTEGER,
      FOREIGN KEY (project_id) REFERENCES projects(id)
    )
  `)
  await d.execute(`
    CREATE TABLE IF NOT EXISTS fragment_relations (
      id TEXT PRIMARY KEY,
      fragment_id TEXT NOT NULL,
      target_type TEXT NOT NULL,
      target_id TEXT NOT NULL,
      created_at INTEGER NOT NULL,
      FOREIGN KEY (fragment_id) REFERENCES fragments(id)
    )
  `)
  // FTS5 全文搜索（tags 通过关联表查询，不在此索引）
  await d.execute(`
    CREATE VIRTUAL TABLE IF NOT EXISTS fragments_fts USING fts5(
      content, content=fragments, content_rowid=rowid
    )
  `)
}
