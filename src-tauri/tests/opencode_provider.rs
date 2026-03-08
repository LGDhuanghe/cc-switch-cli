use serde_json::json;
use std::str::FromStr;

use cc_switch_lib::{AppType, MultiAppConfig, Provider, ProviderService};

#[path = "support.rs"]
mod support;
use support::{ensure_test_home, lock_test_mutex, reset_test_fs, state_from_config};

#[test]
fn opencode_add_syncs_all_providers_to_live_config() {
    let _guard = lock_test_mutex();
    reset_test_fs();
    let home = ensure_test_home();

    let app_type = AppType::from_str("opencode").expect("opencode app type should parse");
    let state = state_from_config(MultiAppConfig::default());

    let first = Provider::with_id(
        "openai".to_string(),
        "OpenAI Compatible".to_string(),
        json!({
            "npm": "@ai-sdk/openai-compatible",
            "options": {
                "baseURL": "https://api.example.com/v1",
                "apiKey": "sk-first"
            },
            "models": {
                "gpt-4o": { "name": "GPT-4o" }
            }
        }),
        None,
    );

    let second = Provider::with_id(
        "anthropic".to_string(),
        "Anthropic".to_string(),
        json!({
            "npm": "@ai-sdk/anthropic",
            "options": {
                "apiKey": "sk-second"
            },
            "models": {
                "claude-3-7-sonnet": { "name": "Claude 3.7 Sonnet" }
            }
        }),
        None,
    );

    ProviderService::add(&state, app_type.clone(), first).expect("first add should succeed");
    ProviderService::add(&state, app_type, second).expect("second add should succeed");

    let opencode_path = home.join(".config").join("opencode").join("opencode.json");
    let live: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&opencode_path).expect("read opencode live config"),
    )
    .expect("parse opencode live config");

    let providers = live
        .get("provider")
        .and_then(|value| value.as_object())
        .expect("opencode config should contain provider map");

    assert!(providers.contains_key("openai"));
    assert!(providers.contains_key("anthropic"));
}
