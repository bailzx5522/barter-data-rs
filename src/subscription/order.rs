use super::SubKind;
use barter_integration::model::Side;
use barter_macro::{DeSubKind, SerSubKind};
use serde::{Deserialize, Serialize};

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`PublicTrade`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, DeSubKind, SerSubKind)]
pub struct PublicTrades;

impl SubKind for PrivateOrders {
    type Event = PrivateOrder;
}

/// Normalised Barter [`PublicTrade`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct PrivateOrder {
    pub inst_id: String,
    pub ccy: String,
    pub order_id: String,
    pub client_order_id: String,

    pub price: f64,
    pub fill_price: f64,
    pub fill_size: f64,
    pub fill_time: i64,
    pub acc_fill_size: f64,
    pub order_state: String,

    pub price_usd: Option<f64>,
    pub price_vol: Option<f64>,
    pub amount: f64,
    pub side: Side,
    pub pos_side: Side,
    pub order_type: String,

    pub u_time: i64,
    pub c_time: i64,
    pub amend_result: String,
}
