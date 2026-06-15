use serde::{Deserialize, Serialize};

/// Keyboard shortcuts configuration (shortcuts.json).
///
/// Reference: docs/architecture.md §5.4.2
/// All shortcuts use Tauri Accelerator format ("Mod" → Cmd on macOS, Ctrl elsewhere).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsConfig {
    #[serde(default)]
    pub global: GlobalShortcuts,
    #[serde(default)]
    pub app: AppShortcuts,
    #[serde(default)]
    pub editor: EditorShortcuts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalShortcuts {
    pub capture: String,
    pub capture_clipboard: String,
}

impl Default for GlobalShortcuts {
    fn default() -> Self {
        Self {
            capture: "Ctrl+Shift+K".into(),
            capture_clipboard: "Ctrl+Shift+M".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppShortcuts {
    pub new_note: String,
    pub new_project: String,
    pub quick_search: String,
    pub toggle_sidebar: String,
    pub open_settings: String,
}

impl Default for AppShortcuts {
    fn default() -> Self {
        Self {
            new_note: "Ctrl+N".into(),
            new_project: "Ctrl+Shift+N".into(),
            quick_search: "Ctrl+P".into(),
            toggle_sidebar: "Ctrl+B".into(),
            open_settings: "Ctrl+,".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorShortcuts {
    pub bold: String,
    pub italic: String,
    pub underline: String,
    pub strikethrough: String,
    pub code: String,
    pub link: String,
    pub save: String,
}

impl Default for EditorShortcuts {
    fn default() -> Self {
        Self {
            bold: "Ctrl+B".into(),
            italic: "Ctrl+I".into(),
            underline: "Ctrl+U".into(),
            strikethrough: "Ctrl+Shift+S".into(),
            code: "Ctrl+E".into(),
            link: "Ctrl+K".into(),
            save: "Ctrl+S".into(),
        }
    }
}