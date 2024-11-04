use app::api;
use app::cmd;
use clap::Parser;
use pkg::core::{cache, config, db, logger};
use tracing_appender::non_blocking::WorkerGuard;

pub mod app;
pub mod pkg;

#[tokio::main]
async fn main() {
    let cli = cmd::Cli::parse();
    // _guard must be in the main function for the logging to take effect
    let _guard = init(&cli.config).await;
    // Handle subcommand
    if let Some(v) = cli.command {
        match v {
            cmd::Command::Hello { name } => cmd::hello::exec(name),
            cmd::Command::Serve => app::serve().await,
        }
    }
}

async fn init(cfg_file: &str) -> WorkerGuard {
    // Initialize configuration
    config::init(cfg_file);
    // Initialize logging
    let _guard = logger::init(Some(config::global()));
    // Initialize database
    db::init(config::global()).await;
    // Initialize Redis
    cache::init_redis(config::global());

    _guard
}
