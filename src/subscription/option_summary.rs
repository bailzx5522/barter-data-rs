use super::SubKind;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`Candle`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct OptionSummaries;

impl SubKind for OptionSummaries {
    type Event = OptionSummary;
}

/// Normalised Barter Option Summary model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct OptionSummary {
    pub inst_id: String,
    pub underlying: String,
    pub ts: DateTime<Utc>,
    pub foward_px: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub mark_vol: f64,
    pub atm_vol: f64,
}
