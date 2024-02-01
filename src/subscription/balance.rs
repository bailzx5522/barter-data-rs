use super::SubKind;
use barter_integration::model::Side;
use barter_macro::{DeSubKind, SerSubKind};
use serde::{Deserialize, Serialize};

/// balance and postion
/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`PublicTrade`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, DeSubKind, SerSubKind)]
pub struct Balances;

impl SubKind for Balances {
    type Event = Balance;
}

/// Normalised Barter [`balance_and_position`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub p_time: String,
    pub event_type: String,
    pub bal_data: Vec<BalanceData>,
    pub pos_data: Vec<PositionData>,
    pub trades: Vec<Trade>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    pub ccy: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub cash_bal: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub u_time: i64,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    pub pos_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub avg_px: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub base_bal: f64,
    pub ccy: String,
    pub inst_id: String,
    pub inst_type: String,
    pub mgn_mode: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub pos: f64,
    pub pos_ccy: String,
    pub pos_side: String,
    pub quote_bal: String,
    pub trade_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub u_time: i64,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub inst_id: String,
    pub trade_id: String,
}
