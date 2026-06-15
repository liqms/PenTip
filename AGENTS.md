# PenTip Agents Guide

This file provides context and conventions for AI agents working on the PenTip project.

## Project Overview

PenTip（笔触）is a cross-platform desktop note-taking and creation app built with **Tauri 2.x** + **Vue 3** + **TypeScript**. It focuses on the loop of "fragment capture → AI auto-organization → deep creation". The current phase implements capture and local storage; AI features are planned for future phases.

## Tech Stack

| Category | Technology |
|----------|-----------|
| Desktop framework | Tauri 2.x (Rust backend) |
| Frontend | Vue 3 + Composition API + `<script setup>` |
| Language | TypeScript (strict) |
| UI library | Naive UI |
| Styles | SCSS + CSS variables |
| State management | Pinia |
| Router | Vue Router 4 |
| Database | SQLite (via sqlx) |
| Full-text search | SQLite FTS5 |
| Build tool | Vite |
| Package manager | pnpm |
| Linter | ESLint + @antfu/eslint-config |
| Formatter | Prettier |
| Unit test | Vitest |
| E2E test | Playwright |
| Git hooks | husky + lint-staged |
| Commit convention | Conventional Commits + commitlint |

## Key Architecture Decisions

1. **Local-first** — All core features work offline. Cloud sync is optional.
2. **Zero-organization** — Users just write; AI handles classification later.
3. **Single SQLite DB** — All structured data in one `pentip.db` file.
4. **App config ≠ user data** — Config directory managed by Tauri; user data in `~/Documents/PenTip/`.
5. **Soft delete** — Fragment, Project, Page all use `deleted_at: number | null` for recovery.
6. **Global shortcut** — Tauri registers system-level hotkey for quick capture panel.

## Project Structure

```
PenTip/
├── src/                        # Vue 3 frontend
│   ├── main.ts
│   ├── App.vue
│   ├── assets/
│   ├── styles/                 # SCSS styles
│   ├── router/
│   ├── stores/                 # Pinia (app, notes, projects, settings)
│   ├── composables/            # Reusable composables
│   ├── api/                    # Tauri IPC call wrappers
│   │   ├── index.ts
│   │   ├── fragment.ts
│   │   ├── tag.ts
│   │   ├── project.ts
│   │   ├── page.ts
│   │   └── search.ts
│   ├── components/
│   │   ├── common/             # Naive UI wrapper components
│   │   ├── layout/             # Layout (Sidebar, Header, Content)
│   │   ├── capture/            # Quick capture panels
│   │   └── ai/                 # AI components (future use)
│   ├── views/                  # Page views (Home, Notes, Projects, Settings, Editor)
│   ├── utils/
│   ├── types/                  # TypeScript interfaces & type aliases
│   │   ├── index.ts               #   Barrel export
│   │   ├── fragment.ts            #   FragmentRow, FragmentType, FragmentRelation
│   │   ├── tag.ts                 #   TagRow, TagTargetType, TagRelation
│   │   ├── project.ts            #   ProjectRow, ProjectContentType
│   │   ├── page.ts               #   PageRow
│   │   └── global.d.ts           #   Vite/Vue type declarations
│   └── i18n/                   # i18n (enUS, zhCN)
├── src-tauri/                  # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── sql/                    # SQL migration files
│   │   └── 001_create_tables.sql
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/           # Tauri IPC command handlers
│   │   │   ├── mod.rs
│   │   │   ├── fragment.rs
│   │   │   ├── tag.rs
│   │   │   ├── project.rs
│   │   │   ├── page.rs
│   │   │   └── file.rs
│   │   ├── db/                 # Unified data access layer (sqlx + SQLite)
│   │   │   ├── mod.rs
│   │   │   └── queries.rs
│   │   ├── services/           # Business logic layer
│   │   │   ├── mod.rs
│   │   │   └── storage/        # Storage service (SQLite + FTS5)
│   │   │       └── mod.rs
│   │   └── utils/              # Shared utilities
│   │       ├── mod.rs
│   │       └── helpers.rs
│   └── capabilities/           # Tauri permissions
│       └── default.json
├── tests/                      # unit/ + e2e/
├── tests/                      # Integration tests (discovered by cargo test)
│   ├── tests.rs                #   Test entry point
│   ├── common/                 #   Shared test helpers
│   │   └── mod.rs
│   └── unit/                   #   Unit tests per component
│       ├── mod.rs
│       └── queries.rs          #   Data layer tests (21 cases)
├── docs/                       # Project docs
└── ...root config files
```

## Data Model (Core Entities)

```typescript
// ─── Fragment ─────────────────────────────
type FragmentType = 'idea' | 'todo' | 'memo' | 'diary' | 'work-log'
type FragmentTargetType = 'project' | 'page'

interface FragmentRow {
  id: string
  content: string
  type: FragmentType
  source?: string
  created_at: number
  updated_at: number
  deleted_at: number | null            // Soft delete
}

// ─── Tag ──────────────────────────────────
type TagTargetType = 'fragment' | 'project' | 'page'

interface TagRow {
  id: string
  name: string
  created_at: number
}

interface TagRelation {
  tag_id: string
  target_type: TagTargetType
  target_id: string
  created_at: number
}

// ─── Project ──────────────────────────────
type ProjectContentType = 'novel' | 'article' | 'script' | 'image-text' | 'video' | 'audio'
type ProjectStatus = 'in-progress' | 'completed'

interface ProjectRow {
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

// ─── Page ─────────────────────────────────
interface PageRow {
  id: string
  project_id: string
  title: string
  content?: string
  order: number
  created_at: number
  updated_at: number
  deleted_at: number | null
}
```

Storage: all data in SQLite, export/backup via Markdown.

## Coding Conventions

### TypeScript / Vue
- **MUST** use Composition API + `<script setup lang="ts">` — never Options API
- No complex expressions in templates; use computed properties
- Props use `withDefaults` + type annotation, no runtime validation
- Emit typed via `defineEmits<{ (e: string, payload: T): void }>()`
- No `any` — use `unknown` instead
- Template refs typed as `ref<InstanceType<typeof Component>>()`

### SCSS / UI
- Prefer **Naive UI** built-in components over custom HTML
- Custom styles via `<style scoped lang="scss">`
- Global variables in `styles/variables.scss`
- Use CSS variables for theme (light/dark)

### Rust
- Follow Rust 2018+ edition
- Run `clippy` before commits
- Tauri commands return `Result<T, String>`
- Unsafe code REQUIRES `// SAFETY:` comment

### Naming

| Type | Convention | Example |
|------|-----------|---------|
| Component | PascalCase | `QuickCapture.vue` |
| File | kebab-case | `use-theme.ts` |
| Variable/function | camelCase | `userName` |
| Constant | UPPER_SNAKE_CASE | `MAX_FILE_SIZE` |
| Interface/type | PascalCase | `NoteMetadata` |
| Pinia store | use + camelCase | `useAppStore` |
| Directory | kebab-case | `composables/` |
| Rust module | snake_case | `notes.rs` |

### Git
- Branch prefix: `codex/`
- Commits follow Conventional Commits: `feat(capture): add global shortcut`
- PRs merge via squash to `dev`, releases from `dev` to `main`

## Error Codes

Commands return `Result<T, AppError>`. The frontend receives a JSON object:

```json
{ "code": "FRAGMENT_NOT_FOUND", "message": "Fragment not found: abc-123" }
```

### Codes

| Code | Meaning |
|------|---------|
| `DB_INIT` | Database initialization fails |
| `DB_QUERY` | Database query/operation fails |
| `FRAGMENT_NOT_FOUND` | Fragment not found by id |
| `PROJECT_NOT_FOUND` | Project not found by id |
| `PAGE_NOT_FOUND` | Page not found by id |
| `TAG_NOT_FOUND` | Tag not found by id |
| `CONFIG_LOAD` | App config load fails |
| `CONFIG_SAVE` | App config save fails |
| `INTERNAL` | Unexpected internal error |

### TypeScript

```typescript
// src/types/error.ts
const ErrorCode = { ... } as const
type ErrorCode = keyof typeof ErrorCode
interface AppError { code: ErrorCode; message: string }

try {
  const result = await invoke("create_fragment", { content })
} catch (e) {
  const err = e as AppError
  if (err.code === "DB_QUERY") { /* handle */ }
}
```

## Commands

```bash
pnpm dev          # Tauri dev mode (Vite + Rust backend, app window auto-opens)
pnpm build        # Build production desktop app
pnpm test         # Run all tests with detailed report (Vitest)
pnpm lint         # ESLint check
pnpm format       # Prettier format (src/)
pnpm typecheck    # TypeScript type check (vue-tsc)
```

## Storage Paths

| Platform | App Config | User Data (default) |
|----------|-----------|-------------------|
| Windows | `%APPDATA%/PenTip/` | `~/Documents/PenTip/` |
| macOS | `~/Library/Application Support/PenTip/` | `~/Documents/PenTip/` |
| Linux | `~/.local/share/PenTip/` | `~/Documents/PenTip/` |

User data path is configurable at install time.