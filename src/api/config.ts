import { invoke } from '@tauri-apps/api/core'
import type { AppConfig, ShortcutsConfig } from '../types'

export interface ConfigResult {
  app: AppConfig
  shortcuts: ShortcutsConfig
  workspace: string
}

export async function getConfig(): Promise<ConfigResult> {
  return invoke('get_config')
}

export async function updateAppConfig(updates: AppConfig): Promise<ConfigResult> {
  return invoke('update_app_config', { updates })
}

export async function updateShortcutsConfig(updates: ShortcutsConfig): Promise<ConfigResult> {
  return invoke('update_shortcuts_config', { updates })
}

export async function setWorkspace(path: string): Promise<ConfigResult> {
  return invoke('set_workspace', { path })
}

export async function resetConfig(): Promise<ConfigResult> {
  return invoke('reset_config')
}
