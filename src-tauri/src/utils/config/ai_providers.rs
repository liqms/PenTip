use serde::{Deserialize, Serialize};

/// AI provider configuration (ai-providers.json).
///
/// Reference: docs/architecture.md §5.4.3
/// Placeholder for future AI phase — not yet wired to commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProvidersConfig {
    pub active_provider: Option<String>,
    #[serde(default)]
    pub providers: Vec<AiProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProvider {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: String,
    pub base_url: String,
    /// Placeholder — in production the key is encrypted via system keychain.
    pub api_key: String,
    #[serde(default)]
    pub models: Vec<AiModel>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiModel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub model_type: String,
}

fn default_enabled() -> bool { true }