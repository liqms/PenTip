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
| Database | SQLite (via Tauri SQL plugin) |
| Full-text search | SQLite FTS5 |
| ORM | Drizzle ORM |
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
│   ├── components/
│   │   ├── common/             # Naive UI wrapper components
│   │   ├── layout/             # Layout (Sidebar, Header, Content)
│   │   ├── capture/            # Quick capture panels
│   │   └── ai/                 # AI components (future use)
│   ├── views/                  # Page views (Home, Notes, Projects, Settings, Editor)
│   ├── services/
│   │   └── storage/            # SQLite + FTS5 services
│   ├── utils/
│   ├── types/                  # TypeScript interfaces
│   └── i18n/                   # i18n (enUS, zhCN)
├── src-tauri/                  # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs / lib.rs
│   │   ├── commands/           # IPC commands (notes, file)
│   │   ├── db/                 # DB operations + migrations
│   │   └── utils/
│   └── capabilities/
├── tests/                      # unit/ + e2e/
├── docs/                       # Project docs
└── ...root config files
```

## Data Model (Core Entities)

### Fragment (碎片笔记)
```typescript
interface Fragment {
  id: string
  content: string
  type: 'idea' | 'todo' | 'memo' | 'diary' | 'work-log'  // Business scenario
  tags: string[]
  source?: string
  created_at: number
  updated_at: number
  deleted_at: number | null      // Soft delete
}
```

### Project (项目)
```typescript
interface Project {
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
```

### Page (页面)
```typescript
interface Page {
  id: string
  project_id: string
  title: string
  content: string
  order: number
  created_at: number
  updated_at: number
  deleted_at: number | null
}
```

### FragmentRelation (关联表)
```typescript
interface FragmentRelation {
  id: string
  fragment_id: string
  target_type: 'project' | 'page'
  target_id: string
  created_at: number
}
```

Storage: all data in SQLite, Markdown only for export/backup.

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

## Commands

```bash
pnpm install          # Install deps
pnpm run dev          # Start dev server (Vite + Tauri)
pnpm run build        # Build for production
pnpm test             # Run unit tests (Vitest)
pnpm test run         # Run tests once (CI mode)
pnpm run lint         # ESLint check
pnpm run format       # Prettier format
pnpm run tauri        # Tauri CLI commands
```

## Storage Paths

| Platform | App Config | User Data (default) |
|----------|-----------|-------------------|
| Windows | `%APPDATA%/PenTip/` | `~/Documents/PenTip/` |
| macOS | `~/Library/Application Support/PenTip/` | `~/Documents/PenTip/` |
| Linux | `~/.local/share/PenTip/` | `~/Documents/PenTip/` |

User data path is configurable at install time.
