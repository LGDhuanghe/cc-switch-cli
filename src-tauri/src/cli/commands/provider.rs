use clap::Subcommand;
use std::sync::RwLock;

use crate::app_config::{AppType, MultiAppConfig};
use crate::error::AppError;
use crate::services::ProviderService;
use crate::store::AppState;
use crate::cli::ui::{create_table, success, error, highlight, info};

#[derive(Subcommand)]
pub enum ProviderCommand {
    /// List all providers
    List,
    /// Show current provider
    Current,
    /// Switch to a provider
    Switch {
        /// Provider ID to switch to
        id: String,
    },
    /// Add a new provider (interactive)
    Add,
    /// Edit a provider
    Edit {
        /// Provider ID to edit
        id: String,
    },
    /// Delete a provider
    Delete {
        /// Provider ID to delete
        id: String,
    },
    /// Duplicate a provider
    Duplicate {
        /// Provider ID to duplicate
        id: String,
    },
    /// Test provider endpoint speed
    Speedtest {
        /// Provider ID to test
        id: String,
    },
}

pub fn execute(cmd: ProviderCommand, app: Option<AppType>) -> Result<(), AppError> {
    let app_type = app.unwrap_or(AppType::Claude);

    match cmd {
        ProviderCommand::List => list_providers(app_type),
        ProviderCommand::Current => show_current(app_type),
        ProviderCommand::Switch { id } => switch_provider(app_type, &id),
        ProviderCommand::Add => add_provider(app_type),
        ProviderCommand::Edit { id } => edit_provider(app_type, &id),
        ProviderCommand::Delete { id } => delete_provider(app_type, &id),
        ProviderCommand::Duplicate { id } => duplicate_provider(app_type, &id),
        ProviderCommand::Speedtest { id } => speedtest_provider(app_type, &id),
    }
}

fn get_state() -> Result<AppState, AppError> {
    let config = MultiAppConfig::load()?;
    Ok(AppState {
        config: RwLock::new(config),
    })
}

fn list_providers(app_type: AppType) -> Result<(), AppError> {
    let state = get_state()?;
    let app_str = app_type.as_str().to_string();
    let providers = ProviderService::list(&state, app_type.clone())?;
    let current_id = ProviderService::current(&state, app_type)?;

    if providers.is_empty() {
        println!("{}", info("No providers found."));
        println!("Use 'cc-switch provider add' to create a new provider.");
        return Ok(());
    }

    // 创建表格
    let mut table = create_table();
    table.set_header(vec!["", "ID", "Name", "Category", "Created"]);

    // 按创建时间排序
    let mut provider_list: Vec<_> = providers.into_iter().collect();
    provider_list.sort_by(|(_, a), (_, b)| {
        // 先按 sort_index，再按创建时间
        match (a.sort_index, b.sort_index) {
            (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.created_at.cmp(&b.created_at),
        }
    });

    for (id, provider) in provider_list {
        let current_marker = if id == current_id { "✓" } else { "" };
        let created = provider.created_at
            .map(|ts| {
                use chrono::{DateTime, Utc};
                DateTime::<Utc>::from_timestamp(ts, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string());

        let category = provider.category.as_deref().unwrap_or("unknown");

        let row = if id == current_id {
            vec![
                highlight(current_marker),
                highlight(&id),
                highlight(&provider.name),
                category.to_string(),
                created,
            ]
        } else {
            vec![
                current_marker.to_string(),
                id.clone(),
                provider.name.clone(),
                category.to_string(),
                created,
            ]
        };

        table.add_row(row);
    }

    println!("{}", table);
    println!("\n{} Application: {}", info("ℹ"), app_str);
    println!("{} Current: {}", info("→"), highlight(&current_id));

    Ok(())
}

fn show_current(app_type: AppType) -> Result<(), AppError> {
    let state = get_state()?;
    let app_str = app_type.as_str().to_string();
    let current_id = ProviderService::current(&state, app_type.clone())?;
    let providers = ProviderService::list(&state, app_type)?;

    let provider = providers.get(&current_id)
        .ok_or_else(|| AppError::Message(format!("Current provider '{}' not found", current_id)))?;

    let created = provider.created_at
        .and_then(|ts| {
            use chrono::{DateTime, Utc};
            DateTime::<Utc>::from_timestamp(ts, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        })
        .unwrap_or_else(|| "Unknown".to_string());

    println!("{}", highlight("Current Provider"));
    println!("{}", "=".repeat(50));
    println!("ID:       {}", current_id);
    println!("Name:     {}", provider.name);
    println!("Category: {}", provider.category.as_deref().unwrap_or("unknown"));
    println!("App:      {}", app_str);
    println!("Created:  {}", created);

    Ok(())
}

fn switch_provider(app_type: AppType, id: &str) -> Result<(), AppError> {
    let state = get_state()?;
    let app_str = app_type.as_str().to_string();

    // 检查 provider 是否存在
    let providers = ProviderService::list(&state, app_type.clone())?;
    if !providers.contains_key(id) {
        return Err(AppError::Message(format!("Provider '{}' not found", id)));
    }

    // 执行切换
    ProviderService::switch(&state, app_type, id)?;

    println!("{}", success(&format!("✓ Switched to provider '{}'", id)));
    println!("{}", info(&format!("  Application: {}", app_str)));
    println!("\n{}", info("Note: Restart your CLI client to apply the changes."));

    Ok(())
}

fn delete_provider(app_type: AppType, id: &str) -> Result<(), AppError> {
    let state = get_state()?;

    // 检查是否是当前 provider
    let current_id = ProviderService::current(&state, app_type.clone())?;
    if id == current_id {
        return Err(AppError::Message(
            "Cannot delete the current active provider. Please switch to another provider first.".to_string()
        ));
    }

    // 确认删除
    let confirm = inquire::Confirm::new(&format!("Are you sure you want to delete provider '{}'?", id))
        .with_default(false)
        .prompt()
        .map_err(|e| AppError::Message(format!("Prompt failed: {}", e)))?;

    if !confirm {
        println!("{}", info("Cancelled."));
        return Ok(());
    }

    // 执行删除
    ProviderService::delete(&state, app_type, id)?;

    println!("{}", success(&format!("✓ Deleted provider '{}'", id)));

    Ok(())
}

fn add_provider(_app_type: AppType) -> Result<(), AppError> {
    println!("{}", highlight("Add New Provider"));
    println!("{}", "=".repeat(50));

    // 交互式输入
    let _name = inquire::Text::new("Provider name:")
        .prompt()
        .map_err(|e| AppError::Message(format!("Prompt failed: {}", e)))?;

    let _category = inquire::Select::new(
        "Category:",
        vec!["official", "custom", "packycode", "other"],
    )
    .prompt()
    .map_err(|e| AppError::Message(format!("Prompt failed: {}", e)))?;

    println!("\n{}", info("Note: Provider configuration is complex and varies by app type."));
    println!("{}", info("For now, please use the config file directly to add detailed settings."));
    println!("\n{}", error("Interactive provider creation is not yet fully implemented."));
    println!("{}", info("Coming soon in the next update!"));

    Ok(())
}

fn edit_provider(_app_type: AppType, id: &str) -> Result<(), AppError> {
    println!("{}", info(&format!("Editing provider '{}'...", id)));
    println!("{}", error("Provider editing is not yet implemented."));
    println!("{}", info("Please edit ~/.cc-switch/config.json directly for now."));
    Ok(())
}

fn duplicate_provider(_app_type: AppType, id: &str) -> Result<(), AppError> {
    println!("{}", info(&format!("Duplicating provider '{}'...", id)));
    println!("{}", error("Provider duplication is not yet implemented."));
    Ok(())
}

fn speedtest_provider(_app_type: AppType, id: &str) -> Result<(), AppError> {
    println!("{}", info(&format!("Testing provider '{}'...", id)));
    println!("{}", error("Speedtest is not yet implemented."));
    Ok(())
}
