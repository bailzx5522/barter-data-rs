use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{subscription::ExchangeSub, ExchangeId},
    subscription::{
        balance::{Balance, Balances},
        book::{Level, OrderBookL1},
        mark_price::{MarkPrice, MarkPrices},
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

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkxBalance {
    #[serde(
        rename = "ts",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,

    pub p_time: String,
    pub event_type: String,
    // pub bal_data: Vec<BalanceData>
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
                        p_time: bal.p_time,
                        event_type: bal.event_type,
                        // bal_data: todo!(),
                        // pos_data: todo!(),
                        // trades: todo!(),
                    },
                })
            })
            .collect()
    }
}
