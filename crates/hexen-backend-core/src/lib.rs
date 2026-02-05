use serde::{Deserialize, Serialize};

pub mod config;
pub mod security;
pub mod middleware;
pub mod approval_api;
pub mod apps_api;
pub mod xbox_api;
pub mod dev_endpoints;
pub mod suggestion_engine;
pub mod action_executor;
pub mod resilient_executor;
pub mod security_diagnostics;
pub mod security_upgrades;
pub mod fallback_mechanisms;
pub mod circuit_breaker;
pub mod health_checks;
pub mod emergency_system;
pub mod metrics;
pub mod game_metrics;
pub mod storage;
pub mod validation;
pub mod models;
pub mod csp_hardening;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuronSafetyConfig {
    pub max_firing_rate_hz: f32,
    pub min_isi_ms: f32,
    pub max_ttfs_ms: f32,
}

impl NeuronSafetyConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        use anyhow::bail;
        if self.max_firing_rate_hz <= 0.0 {
            bail!("max_firing_rate_hz must be > 0");
        }
        let implied_min_isi = 1000.0 / self.max_firing_rate_hz;
        if self.min_isi_ms + 1e-3 < implied_min_isi {
            bail!("min_isi_ms < 1 / max_firing_rate_hz");
        }
        if self.max_ttfs_ms <= 0.0 {
            bail!("max_ttfs_ms must be > 0");
        }
        Ok(())
    }
}
