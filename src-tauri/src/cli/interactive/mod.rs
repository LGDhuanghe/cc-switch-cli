use std::sync::RwLock;

use inquire::{Select, Confirm};

use crate::app_config::{AppType, MultiAppConfig};
use crate::error::AppError;
use crate::services::{ProviderService, McpService, PromptService};
use crate::store::AppState;
use crate::cli::ui::{create_table, success, error, highlight, info};

pub fn run(app: Option<AppType>) -> Result<(), AppError> {
    let mut app_type = app.unwrap_or(AppType::Claude);

    // 显示欢迎信息
    print_welcome(&app_type);

    loop {
        match show_main_menu(&app_type)? {
            MainMenuChoice::ManageProviders => {
                if let Err(e) = manage_providers_menu(&app_type) {
                    println!("\n{}", error(&format!("Error: {}", e)));
                    pause();
                }
            }
            MainMenuChoice::ManageMCP => {
                if let Err(e) = manage_mcp_menu(&app_type) {
                    println!("\n{}", error(&format!("Error: {}", e)));
                    pause();
                }
            }
            MainMenuChoice::ManagePrompts => {
                if let Err(e) = manage_prompts_menu(&app_type) {
                    println!("\n{}", error(&format!("Error: {}", e)));
                    pause();
                }
            }
            MainMenuChoice::ViewCurrentConfig => {
                if let Err(e) = view_current_config(&app_type) {
                    println!("\n{}", error(&format!("Error: {}", e)));
                    pause();
                }
            }
            MainMenuChoice::SwitchApp => {
                if let Ok(new_app) = select_app() {
                    app_type = new_app;
                    print_welcome(&app_type);
                }
            }
            MainMenuChoice::Exit => {
                println!("\n{}", success("👋 Goodbye!"));
                break;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum MainMenuChoice {
    ManageProviders,
    ManageMCP,
    ManagePrompts,
    ViewCurrentConfig,
    SwitchApp,
    Exit,
}

impl std::fmt::Display for MainMenuChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManageProviders => write!(f, "🔌 Manage Providers"),
            Self::ManageMCP => write!(f, "🛠️  Manage MCP Servers"),
            Self::ManagePrompts => write!(f, "💬 Manage Prompts"),
            Self::ViewCurrentConfig => write!(f, "👁️  View Current Configuration"),
            Self::SwitchApp => write!(f, "🔄 Switch Application"),
            Self::Exit => write!(f, "🚪 Exit"),
        }
    }
}

fn print_welcome(app_type: &AppType) {
    println!("\n{}", "═".repeat(60));
    println!("{}", highlight("    🎯 CC-Switch Interactive Mode"));
    println!("{}", "═".repeat(60));
    println!("{} Application: {}", info("📱"), highlight(app_type.as_str()));
    println!("{}", "─".repeat(60));
    println!();
}

fn show_main_menu(app_type: &AppType) -> Result<MainMenuChoice, AppError> {
    let choices = vec![
        MainMenuChoice::ManageProviders,
        MainMenuChoice::ManageMCP,
        MainMenuChoice::ManagePrompts,
        MainMenuChoice::ViewCurrentConfig,
        MainMenuChoice::SwitchApp,
        MainMenuChoice::Exit,
    ];

    let choice = Select::new(&format!("What would you like to do? (Current: {})", app_type.as_str()), choices)
        .prompt()
        .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

    Ok(choice)
}

fn select_app() -> Result<AppType, AppError> {
    let apps = vec![AppType::Claude, AppType::Codex, AppType::Gemini];

    let app = Select::new("Select application:", apps)
        .prompt()
        .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

    println!("\n{}", success(&format!("✓ Switched to {}", app.as_str())));
    pause();

    Ok(app)
}

// ============================================================================
// Provider Management
// ============================================================================

fn manage_providers_menu(app_type: &AppType) -> Result<(), AppError> {
    loop {
        println!("\n{}", highlight("🔌 Provider Management"));
        println!("{}", "─".repeat(60));

        let state = get_state()?;
        let providers = ProviderService::list(&state, app_type.clone())?;
        let current_id = ProviderService::current(&state, app_type.clone())?;

        // 显示 providers 表格
        if providers.is_empty() {
            println!("{}", info("No providers found."));
        } else {
            let mut table = create_table();
            table.set_header(vec!["", "Name", "Category"]);

            let mut provider_list: Vec<_> = providers.iter().collect();
            provider_list.sort_by(|(_, a), (_, b)| {
                match (a.sort_index, b.sort_index) {
                    (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.created_at.cmp(&b.created_at),
                }
            });

            for (id, provider) in &provider_list {
                let marker = if *id == &current_id { "✓" } else { "" };
                let category = provider.category.as_deref().unwrap_or("unknown");

                if *id == &current_id {
                    table.add_row(vec![
                        highlight(marker),
                        highlight(&provider.name),
                        highlight(category),
                    ]);
                } else {
                    table.add_row(vec![
                        marker.to_string(),
                        provider.name.clone(),
                        category.to_string(),
                    ]);
                }
            }

            println!("{}", table);
        }

        println!();
        let choices = vec![
            "📋 View Current Provider Details",
            "🔄 Switch Provider",
            "🗑️  Delete Provider",
            "⬅️  Back to Main Menu",
        ];

        let choice = Select::new("Choose an action:", choices)
            .prompt()
            .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

        match choice {
            "📋 View Current Provider Details" => {
                view_current_provider(&state, app_type, &current_id)?;
                pause();
            }
            "🔄 Switch Provider" => {
                switch_provider_interactive(&state, app_type, &providers, &current_id)?;
            }
            "🗑️  Delete Provider" => {
                delete_provider_interactive(&state, app_type, &providers, &current_id)?;
            }
            "⬅️  Back to Main Menu" => break,
            _ => {}
        }
    }

    Ok(())
}

fn view_current_provider(state: &AppState, app_type: &AppType, current_id: &str) -> Result<(), AppError> {
    let providers = ProviderService::list(state, app_type.clone())?;
    if let Some(provider) = providers.get(current_id) {
        println!("\n{}", highlight("Current Provider Details"));
        println!("{}", "─".repeat(60));
        println!("ID:       {}", current_id);
        println!("Name:     {}", provider.name);
        println!("Category: {}", provider.category.as_deref().unwrap_or("unknown"));
    }
    Ok(())
}

fn switch_provider_interactive(state: &AppState, app_type: &AppType, providers: &std::collections::HashMap<String, crate::provider::Provider>, current_id: &str) -> Result<(), AppError> {
    if providers.len() <= 1 {
        println!("\n{}", info("Only one provider available. Cannot switch."));
        pause();
        return Ok(());
    }

    let mut provider_choices: Vec<_> = providers.iter()
        .filter(|(id, _)| *id != current_id)
        .map(|(id, p)| format!("{} ({})", p.name, id))
        .collect();
    provider_choices.sort();

    if provider_choices.is_empty() {
        println!("\n{}", info("No other providers to switch to."));
        pause();
        return Ok(());
    }

    let choice = Select::new("Select provider to switch to:", provider_choices)
        .prompt()
        .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

    // 提取 ID
    let id = choice.split('(').nth(1)
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| AppError::Message("Invalid choice".to_string()))?;

    ProviderService::switch(state, app_type.clone(), id)?;

    println!("\n{}", success(&format!("✓ Switched to provider '{}'", id)));
    println!("{}", info("Note: Restart your CLI client to apply the changes."));
    pause();

    Ok(())
}

fn delete_provider_interactive(state: &AppState, app_type: &AppType, providers: &std::collections::HashMap<String, crate::provider::Provider>, current_id: &str) -> Result<(), AppError> {
    let deletable: Vec<_> = providers.iter()
        .filter(|(id, _)| *id != current_id)
        .map(|(id, p)| format!("{} ({})", p.name, id))
        .collect();

    if deletable.is_empty() {
        println!("\n{}", info("No providers available for deletion (cannot delete current provider)."));
        pause();
        return Ok(());
    }

    let choice = Select::new("Select provider to delete:", deletable)
        .prompt()
        .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

    let id = choice.split('(').nth(1)
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| AppError::Message("Invalid choice".to_string()))?;

    let confirm = Confirm::new(&format!("Are you sure you want to delete provider '{}'?", id))
        .with_default(false)
        .prompt()
        .map_err(|_| AppError::Message("Confirmation failed".to_string()))?;

    if !confirm {
        println!("\n{}", info("Cancelled."));
        pause();
        return Ok(());
    }

    ProviderService::delete(state, app_type.clone(), id)?;
    println!("\n{}", success(&format!("✓ Deleted provider '{}'", id)));
    pause();

    Ok(())
}

// ============================================================================
// MCP Management
// ============================================================================

fn manage_mcp_menu(_app_type: &AppType) -> Result<(), AppError> {
    loop {
        println!("\n{}", highlight("🛠️  MCP Server Management"));
        println!("{}", "─".repeat(60));

        let state = get_state()?;
        let servers = McpService::get_all_servers(&state)?;

        if servers.is_empty() {
            println!("{}", info("No MCP servers found."));
        } else {
            let mut table = create_table();
            table.set_header(vec!["Name", "Claude", "Codex", "Gemini"]);

            let mut server_list: Vec<_> = servers.iter().collect();
            server_list.sort_by_key(|(id, _)| *id);

            for (_, server) in &server_list {
                table.add_row(vec![
                    server.name.clone(),
                    if server.apps.claude { "✓" } else { "" }.to_string(),
                    if server.apps.codex { "✓" } else { "" }.to_string(),
                    if server.apps.gemini { "✓" } else { "" }.to_string(),
                ]);
            }

            println!("{}", table);
        }

        println!();
        let choices = vec![
            "🔄 Sync All Servers",
            "⬅️  Back to Main Menu",
        ];

        let choice = Select::new("Choose an action:", choices)
            .prompt()
            .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

        match choice {
            "🔄 Sync All Servers" => {
                McpService::sync_all_enabled(&state)?;
                println!("\n{}", success("✓ All MCP servers synced successfully"));
                pause();
            }
            "⬅️  Back to Main Menu" => break,
            _ => {}
        }
    }

    Ok(())
}

// ============================================================================
// Prompts Management
// ============================================================================

fn manage_prompts_menu(app_type: &AppType) -> Result<(), AppError> {
    loop {
        println!("\n{}", highlight("💬 Prompt Management"));
        println!("{}", "─".repeat(60));

        let state = get_state()?;
        let prompts = PromptService::get_prompts(&state, app_type.clone())?;

        if prompts.is_empty() {
            println!("{}", info("No prompt presets found."));
        } else {
            let mut table = create_table();
            table.set_header(vec!["", "Name", "Description"]);

            let mut prompt_list: Vec<_> = prompts.iter().collect();
            prompt_list.sort_by(|(_, a), (_, b)| {
                b.updated_at.unwrap_or(0).cmp(&a.updated_at.unwrap_or(0))
            });

            for (_, prompt) in &prompt_list {
                let marker = if prompt.enabled { "✓" } else { "" };
                let desc = prompt.description.as_deref().unwrap_or("")
                    .chars().take(40).collect::<String>();

                if prompt.enabled {
                    table.add_row(vec![
                        highlight(marker),
                        highlight(&prompt.name),
                        desc,
                    ]);
                } else {
                    table.add_row(vec![
                        marker.to_string(),
                        prompt.name.clone(),
                        desc,
                    ]);
                }
            }

            println!("{}", table);
        }

        println!();
        let choices = vec![
            "🔄 Switch Active Prompt",
            "⬅️  Back to Main Menu",
        ];

        let choice = Select::new("Choose an action:", choices)
            .prompt()
            .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

        match choice {
            "🔄 Switch Active Prompt" => {
                switch_prompt_interactive(&state, app_type, &prompts)?;
            }
            "⬅️  Back to Main Menu" => break,
            _ => {}
        }
    }

    Ok(())
}

fn switch_prompt_interactive(state: &AppState, app_type: &AppType, prompts: &std::collections::HashMap<String, crate::prompt::Prompt>) -> Result<(), AppError> {
    if prompts.is_empty() {
        println!("\n{}", info("No prompts available."));
        pause();
        return Ok(());
    }

    let prompt_choices: Vec<_> = prompts.iter()
        .map(|(id, p)| format!("{} ({})", p.name, id))
        .collect();

    let choice = Select::new("Select prompt to activate:", prompt_choices)
        .prompt()
        .map_err(|_| AppError::Message("Selection cancelled".to_string()))?;

    let id = choice.split('(').nth(1)
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| AppError::Message("Invalid choice".to_string()))?;

    PromptService::enable_prompt(state, app_type.clone(), id)?;

    println!("\n{}", success(&format!("✓ Activated prompt '{}'", id)));
    println!("{}", info("Note: The prompt has been synced to the live configuration file."));
    pause();

    Ok(())
}

// ============================================================================
// Configuration View
// ============================================================================

fn view_current_config(app_type: &AppType) -> Result<(), AppError> {
    println!("\n{}", highlight("👁️  Current Configuration"));
    println!("{}", "─".repeat(60));

    let state = get_state()?;

    // Provider info
    let current_provider = ProviderService::current(&state, app_type.clone())?;
    let providers = ProviderService::list(&state, app_type.clone())?;
    if let Some(provider) = providers.get(&current_provider) {
        println!("{}", highlight("Provider:"));
        println!("  Name:     {}", provider.name);
        println!("  Category: {}", provider.category.as_deref().unwrap_or("unknown"));
        println!();
    }

    // MCP servers count
    let mcp_servers = McpService::get_all_servers(&state)?;
    let enabled_count = mcp_servers.values()
        .filter(|s| s.apps.is_enabled_for(app_type))
        .count();
    println!("{}", highlight("MCP Servers:"));
    println!("  Total:   {}", mcp_servers.len());
    println!("  Enabled: {}", enabled_count);
    println!();

    // Prompts
    let prompts = PromptService::get_prompts(&state, app_type.clone())?;
    let active_prompt = prompts.iter().find(|(_, p)| p.enabled);
    println!("{}", highlight("Prompts:"));
    println!("  Total:  {}", prompts.len());
    if let Some((_, p)) = active_prompt {
        println!("  Active: {}", p.name);
    } else {
        println!("  Active: None");
    }

    println!();
    pause();

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

fn get_state() -> Result<AppState, AppError> {
    let config = MultiAppConfig::load()?;
    Ok(AppState {
        config: RwLock::new(config),
    })
}

fn pause() {
    use inquire::Confirm;
    let _ = Confirm::new("Press Enter to continue...")
        .with_default(true)
        .with_help_message("(This will close automatically)")
        .prompt();
}
