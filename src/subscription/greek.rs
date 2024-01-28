use super::SubKind;
use barter_integration::model::Side;
use barter_macro::{DeSubKind, SerSubKind};
use serde::{Deserialize, Serialize};

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`PublicTrade`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, DeSubKind, SerSubKind)]
pub struct Greeks;

impl SubKind for Greeks {
    type Event = Greek;
}

/// Normalised Barter [`PublicTrade`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Greek {
    pub ccy: String,
    pub ts: i32,
    pub theta_bs: f64,
    pub theta_pa: f64,
    pub delta_bs: f64,
    pub delta_pa: f64,
    pub gamma_bs: f64,
    pub gamma_pa: f64,
    pub vega_bs: f64,
    pub vega_pa: f64,
}
