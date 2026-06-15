/// Shortcuts configuration types matching Rust structs (config/shortcuts.rs).
/// Reference: docs/architecture.md §5.4.2

export interface ShortcutsConfig {
  global: GlobalShortcuts
  app: AppShortcuts
  editor: EditorShortcuts
}

export interface GlobalShortcuts {
  capture: string
  captureClipboard: string
}

export interface AppShortcuts {
  newNote: string
  newProject: string
  quickSearch: string
  toggleSidebar: string
  openSettings: string
}

export interface EditorShortcuts {
  bold: string
  italic: string
  underline: string
  strikethrough: string
  code: string
  link: string
  save: string
}