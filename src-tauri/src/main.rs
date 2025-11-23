use clap::Parser;
use cc_switch_lib::cli::{Cli, Commands};
use cc_switch_lib::AppError;
use std::process;

fn main() {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 执行命令
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Commands::Provider(cmd) => cc_switch_lib::cli::commands::provider::execute(cmd, cli.app),
        Commands::Mcp(cmd) => cc_switch_lib::cli::commands::mcp::execute(cmd, cli.app),
        Commands::Prompts(cmd) => cc_switch_lib::cli::commands::prompts::execute(cmd, cli.app),
        Commands::Skills(cmd) => cc_switch_lib::cli::commands::skills::execute(cmd),
        Commands::Config(cmd) => cc_switch_lib::cli::commands::config::execute(cmd),
        Commands::Env(cmd) => cc_switch_lib::cli::commands::env::execute(cmd, cli.app),
        Commands::App(cmd) => cc_switch_lib::cli::commands::app::execute(cmd),
        Commands::Interactive => cc_switch_lib::cli::interactive::run(cli.app),
        Commands::Completions { shell } => {
            cc_switch_lib::cli::generate_completions(shell);
            Ok(())
        }
    }
}
