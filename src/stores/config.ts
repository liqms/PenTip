import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppConfig, ShortcutsConfig } from '@/types'
import { getConfig, updateAppConfig, updateShortcutsConfig, setWorkspace, resetConfig } from '@/api'

export const useConfigStore = defineStore('config', () => {
  const app = ref<AppConfig>()
  const shortcuts = ref<ShortcutsConfig>()
  const workspace = ref<string>()
  const loading = ref(false)

  // ── Actions ──────────────────────────────

  async function load() {
    loading.value = true
    try {
      const config = await getConfig()
      app.value = config.app
      shortcuts.value = config.shortcuts
      workspace.value = config.workspace
      theme.value = config.app.theme
      locale.value = config.app.locale
    } finally {
      loading.value = false
    }
  }

  async function saveApp(updates: AppConfig) {
    const config = await updateAppConfig(updates)
    app.value = config.app
    theme.value = config.app.theme
    locale.value = config.app.locale
  }

  async function saveShortcuts(updates: ShortcutsConfig) {
    const config = await updateShortcutsConfig(updates)
    shortcuts.value = config.shortcuts
  }

  async function changeWorkspace(path: string) {
    const config = await setWorkspace(path)
    workspace.value = config.workspace
  }

  async function reset() {
    const config = await resetConfig()
    app.value = config.app
    shortcuts.value = config.shortcuts
    workspace.value = config.workspace
    theme.value = config.app.theme
    locale.value = config.app.locale
  }

  function setSidebarCollapsed(v: boolean) { sidebarCollapsed.value = v }
  function toggleSidebar() { sidebarCollapsed.value = !sidebarCollapsed.value }

  // ── State ────────────────────────────────

  const theme = ref<'light' | 'dark' | 'system'>('system')
  const locale = ref<'zh-CN' | 'en-US'>('zh-CN')
  const sidebarCollapsed = ref(false)

  const isDark = () => {
    if (theme.value === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches
    }
    return theme.value === 'dark'
  }

  return {
    app, shortcuts, workspace, loading,
    theme, locale, sidebarCollapsed, isDark,
    load, saveApp, saveShortcuts, changeWorkspace, reset,
    setSidebarCollapsed, toggleSidebar,
  }
})
