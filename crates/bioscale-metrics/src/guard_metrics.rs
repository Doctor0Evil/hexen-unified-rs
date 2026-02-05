use lazy_static::lazy_static;
use prometheus::{register_gauge, Gauge};

lazy_static! {
    pub static ref BCI_DUTY_CEILING: Gauge =
        register_gauge!("bcidutyceiling", "BCI duty cycle ceiling").unwrap();
}
