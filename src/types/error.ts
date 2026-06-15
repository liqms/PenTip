/// Machine-readable error codes matching Rust `AppError.code`.
export const ErrorCode = {
  DB_INIT: 'DB_INIT',
  DB_QUERY: 'DB_QUERY',
  FRAGMENT_NOT_FOUND: 'FRAGMENT_NOT_FOUND',
  PROJECT_NOT_FOUND: 'PROJECT_NOT_FOUND',
  PAGE_NOT_FOUND: 'PAGE_NOT_FOUND',
  TAG_NOT_FOUND: 'TAG_NOT_FOUND',
  CONFIG_LOAD: 'CONFIG_LOAD',
  CONFIG_SAVE: 'CONFIG_SAVE',
  INTERNAL: 'INTERNAL',
} as const

export type ErrorCode = (typeof ErrorCode)[keyof typeof ErrorCode]

/// Error object returned by Tauri commands on failure.
export interface AppError {
  code: ErrorCode
  message: string
}