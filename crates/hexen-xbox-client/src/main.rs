use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "http://localhost:8080")]
    backend: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    tracing::info!("connecting to backend {}", args.backend);
    Ok(())
}
