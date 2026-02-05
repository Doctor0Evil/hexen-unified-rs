use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BciHostSnapshot {
    pub captured_at: OffsetDateTime,
    pub eeg_rms: f32,
    pub hrv_ms: f32,
    pub skull_temp_c: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BciSafetyThresholds {
    pub max_eeg_rms: f32,
    pub min_hrv_ms: f32,
    pub max_skull_temp_c: f32,
}

impl BciHostSnapshot {
    pub fn within(&self, t: &BciSafetyThresholds) -> bool {
        self.eeg_rms <= t.max_eeg_rms
            && self.hrv_ms >= t.min_hrv_ms
            && self.skull_temp_c <= t.max_skull_temp_c
    }
}
