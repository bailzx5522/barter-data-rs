use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{subscription::ExchangeSub, ExchangeId},
    subscription::{
        balance::{Balance, BalanceData, Balances, PositionData, Trade},
        book::{Level, OrderBookL1},
        mark_price::{MarkPrice, MarkPrices},
        position::{Position, Positions},
    },
    Identifier,
};

use barter_integration::{
    de::extract_next,
    model::{instrument::Instrument, Exchange, SubscriptionId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type OkxBalances = OkxMessage<OkxBalance>;

/// doc : https://www.okx.com/docs-v5/zh/#trading-account-websocket-balance-and-position-channel
/// ```json
/// {
///"arg": {
///    "channel": "balance_and_position",
///    "uid": "77982378738415879"
///},
///"data": [{
///    "pTime": "1597026383085",
///    "eventType": "snapshot",
///    "balData": [{
///        "ccy": "BTC",
///        "cashBal": "1",
///        "uTime": "1597026383085"
///    }],
///    "posData": [{
///        "posId": "1111111111",
///        "tradeId": "2",
///        "instId": "BTC-USD-191018",
///        "instType": "FUTURES",
///        "mgnMode": "cross",
///        "posSide": "long",
///        "pos": "10",
///        "ccy": "BTC",
///        "posCcy": "",
///        "avgPx": "3320",
///        "uTime": "1597026383085"
///    }],
///    "trades": [{
///        "instId": "BTC-USD-191018",
///        "tradeId": "2",
///    }]
///}]
///}
/// ```
///
///
///
// #[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct OkxPositionData {
//     pub pos_id: String,
//     #[serde(deserialize_with = "barter_integration::de::de_str")]
//     pub avg_px: f64,
//     #[serde(deserialize_with = "barter_integration::de::de_str")]
//     pub base_bal: f64,
//     pub ccy: String,
//     pub inst_id: String,
//     pub inst_type: String,
//     pub mgn_mode: String,
//     #[serde(deserialize_with = "barter_integration::de::de_str")]
//     pub pos: f64,
//     pub pos_ccy: String,
//     pub pos_side: String,
//     pub quote_bal: String,
//     pub trade_id: String,
//     #[serde(deserialize_with = "barter_integration::de::de_str")]
//     pub u_time: i32,
// }

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkxBalance {
    #[serde(
        rename = "pTime",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,
    pub event_type: String,
    pub bal_data: Vec<BalanceData>,
    pub pos_data: Vec<PositionData>,
    pub trades: Vec<Trade>,
}

// 实现从(ExchangId, Inst, OKxOrderBook) 转换成 MarketIter<>
impl From<(ExchangeId, Instrument, OkxBalances)> for MarketIter<Balance> {
    fn from((exchange_id, instrument, marks): (ExchangeId, Instrument, OkxBalances)) -> Self {
        marks
            .data
            .into_iter()
            .map(|bal| {
                Ok(MarketEvent {
                    exchange_time: bal.ts,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: Balance {
                        p_time: bal.ts.to_string(),
                        event_type: bal.event_type,
                        bal_data: bal.bal_data,
                        pos_data: bal.pos_data,
                        trades: bal.trades,
                    },
                })
            })
            .collect()
    }
}

pub type OkxPositions = OkxMessage<OkxPosition>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkxPosition {
    #[serde(
        rename = "pTime",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,
}

// 实现从(ExchangId, Inst, OKxOrderBook) 转换成 MarketIter<>
impl From<(ExchangeId, Instrument, OkxPositions)> for MarketIter<Positions> {
    fn from((exchange_id, instrument, positions): (ExchangeId, Instrument, OkxPositions)) -> Self {
        positions
            .data
            .into_iter()
            .map(|pos| {
                Ok(MarketEvent {
                    exchange_time: pos.ts,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: Position {},
                })
            })
            .collect()
    }
}
