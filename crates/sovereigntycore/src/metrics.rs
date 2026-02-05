use once_cell::sync::Lazy;
use prometheus::{
    register_gauge_vec, register_int_counter_vec, GaugeVec, IntCounterVec,
};

// RoH scalar per subject
pub static ROH_GAUGE: Lazy<GaugeVec> = Lazy::new(|| {
    register_gauge_vec!(
        "sovereignty_roh_after",
        "Risk-of-Harm after proposed change",
        &["subject_id"]
    ).expect("metric registered")
});

// Envelope tightening violations
pub static ENVELOPE_VIOLATION_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "sovereignty_envelope_violation_total",
        "Count of rejected proposals due to envelope monotonicity break",
        &["subject_id", "scope"]
    ).expect("metric registered")
});

// Decision outcomes
pub static EVOLUTION_DECISIONS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "sovereignty_evolution_decisions_total",
        "Evolution decisions by outcome",
        &["subject_id", "decision"]
    ).expect("metric registered")
});
