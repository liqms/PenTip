/// App configuration types matching Rust structs (config/app.rs).
/// Reference: docs/architecture.md §5.4.1

export interface AppConfig {
  window: WindowConfig
  theme: 'light' | 'dark' | 'system'
  locale: 'zh-CN' | 'en-US'
  userDataPath: string | null
  quickCapture: QuickCaptureConfig
  editor: EditorConfig
  autoSave: AutoSaveConfig
  backup: BackupConfig
  search: SearchConfig
  startup: StartupConfig
}

export interface WindowConfig {
  width: number
  height: number
  x: number | null
  y: number | null
  maximized: boolean
  fullscreen: boolean
}

export interface QuickCaptureConfig {
  shortcut: string
  autoSave: boolean
  stayOnTop: boolean
}

export interface EditorConfig {
  fontSize: number
  fontFamily: string
  lineHeight: number
  showLineNumber: boolean
  enableSpellCheck: boolean
  tabSize: number
  toolbar: 'default' | 'minimal' | 'none'
  bubbleMenu: boolean
  slashCommand: boolean
  autoFormat: boolean
  typewriterMode: boolean
}

export interface AutoSaveConfig {
  enabled: boolean
  intervalMs: number
}

export interface BackupConfig {
  enabled: boolean
  intervalHours: number
  maxCount: number
  compress: boolean
}

export interface SearchConfig {
  maxResults: number
  includeDeleted: boolean
}

export interface StartupConfig {
  launchOnBoot: boolean
  restoreLastSession: boolean
  minimizeToTray: boolean
}