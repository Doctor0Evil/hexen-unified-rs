use bioscale_upgrade_store::registry::default_bci_upgrade;
use clap::Parser;
use serde::Serialize;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "evolution_manifest.yaml")]
    ci_manifest: String,
}

#[derive(Serialize)]
struct Manifest {
    upgrade: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let u = default_bci_upgrade();
    let manifest = Manifest { upgrade: u.id };
    let yaml = serde_yaml::to_string(&manifest)?;
    fs::write(&args.ci_manifest, yaml)?;
    Ok(())
}
