use serde::{Deserialize, Serialize};

/// 健康状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Operational,
    Degraded,
    Failed,
}

/// 流式检查配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamCheckConfig {
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_degraded_threshold_ms")]
    pub degraded_threshold_ms: u64,
    #[serde(default = "default_claude_model")]
    pub claude_model: String,
    #[serde(default = "default_codex_model")]
    pub codex_model: String,
    #[serde(default = "default_gemini_model")]
    pub gemini_model: String,
    #[serde(default = "default_test_prompt")]
    pub test_prompt: String,
}

fn default_timeout_secs() -> u64 {
    45
}

fn default_max_retries() -> u32 {
    2
}

fn default_degraded_threshold_ms() -> u64 {
    6000
}

fn default_claude_model() -> String {
    "claude-haiku-4-5-20251001".to_string()
}

fn default_codex_model() -> String {
    "gpt-5.1-codex@low".to_string()
}

fn default_gemini_model() -> String {
    "gemini-3-pro-preview".to_string()
}

fn default_test_prompt() -> String {
    "Who are you?".to_string()
}

impl Default for StreamCheckConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 45,
            max_retries: 2,
            degraded_threshold_ms: 6000,
            claude_model: "claude-haiku-4-5-20251001".to_string(),
            codex_model: "gpt-5.1-codex@low".to_string(),
            gemini_model: "gemini-3-pro-preview".to_string(),
            test_prompt: default_test_prompt(),
        }
    }
}

/// 流式检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamCheckResult {
    pub status: HealthStatus,
    pub success: bool,
    pub message: String,
    pub response_time_ms: Option<u64>,
    pub http_status: Option<u16>,
    pub model_used: String,
    pub tested_at: i64,
    pub retry_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_category: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthInfo {
    pub(crate) api_key: String,
    pub(crate) strategy: AuthStrategy,
    pub(crate) access_token: Option<String>,
}

impl AuthInfo {
    pub(crate) fn new(api_key: String, strategy: AuthStrategy) -> Self {
        Self {
            api_key,
            strategy,
            access_token: None,
        }
    }

    pub(crate) fn with_access_token(api_key: String, access_token: String) -> Self {
        Self {
            api_key,
            strategy: AuthStrategy::GoogleOAuth,
            access_token: Some(access_token),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthStrategy {
    Anthropic,
    ClaudeAuth,
    Bearer,
    Google,
    GoogleOAuth,
}
