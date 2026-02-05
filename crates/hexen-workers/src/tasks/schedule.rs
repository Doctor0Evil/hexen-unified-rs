use anyhow::Result;

pub async fn run_scheduler() -> Result<()> {
    tracing::info!("hexen-workers scheduler starting");
    Ok(())
}
