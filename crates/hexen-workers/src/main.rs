mod neurotask_runtime;
mod mistral_integration;
mod tasks;

use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tasks::schedule::run_scheduler().await?;
    signal::ctrl_c().await?;
    Ok(())
}
