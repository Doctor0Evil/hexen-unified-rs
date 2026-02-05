use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct HostBudget {
    pub max_duty_cycle: f32,
    pub max_thermal_c: f32,
    pub max_power_w: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrainSpecs {
    pub subject_id: String,
    pub bci_channel_count: u16,
    pub eeg_sampling_hz: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub listen_addr: String,
    pub host_budget: HostBudget,
    pub brain_specs: BrainSpecs,
}

pub fn load() -> ServiceConfig {
    let mut cfg = config::Config::builder()
        .add_source(config::Environment::default().separator("__"))
        .build()
        .expect("config");

    cfg.try_deserialize().unwrap_or_else(|_| ServiceConfig {
        listen_addr: "0.0.0.0:8080".into(),
        host_budget: HostBudget {
            max_duty_cycle: 0.35,
            max_thermal_c: 39.0,
            max_power_w: 5.0,
        },
        brain_specs: BrainSpecs {
            subject_id: "anonymous".into(),
            bci_channel_count: 64,
            eeg_sampling_hz: 1000,
        },
    })
}

pub fn listen_addr() -> SocketAddr {
    load()
        .listen_addr
        .parse()
        .expect("valid LISTEN_ADDR like 0.0.0.0:8080")
}
