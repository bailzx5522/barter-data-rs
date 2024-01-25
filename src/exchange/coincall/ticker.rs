use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{subscription::ExchangeSub, ExchangeId},
    subscription::book::{Level, OrderBookL1},
    Identifier,
};

use barter_integration::{
    de::extract_next,
    model::{instrument::Instrument, Exchange, SubscriptionId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type OkxOrderBookL1 = OkxMessage<OkxTicker>;

/// doc : https://www.okx.com/docs-v5/zh/#order-book-trading-market-data-ws-tickers-channel
/// ```json
/// {
///     "arg": {
///         "channel": "tickers",
///         "instId": "LTC-USD-200327"
///     },
///     "data": [{
///         "instType": "SWAP",
///         "instId": "LTC-USD-200327",
///         "last": "9999.99",
///         "lastSz": "0.1",
///         "askPx": "9999.99",
///         "askSz": "11",
///         "bidPx": "8888.88",
///         "bidSz": "5",
///         "open24h": "9000",
///         "high24h": "10000",
///         "low24h": "8888.88",
///         "volCcy24h": "2222",
///         "vol24h": "2222",
///         "sodUtc0": "2222",
///         "sodUtc8": "2222",
///         "ts": "1597026383085"
///     }]
/// }
/// ```

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct OkxTicker {
    #[serde(
        rename = "ts",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,

    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "askPx", deserialize_with = "barter_integration::de::de_str")]
    pub ask_px: f64,
    #[serde(rename = "askSz", deserialize_with = "barter_integration::de::de_str")]
    pub ask_sz: f64,
    #[serde(rename = "bidPx", deserialize_with = "barter_integration::de::de_str")]
    pub bid_px: f64,
    #[serde(rename = "bidSz", deserialize_with = "barter_integration::de::de_str")]
    pub bid_sz: f64,
}

// 实现从(ExchangId, Inst, OKxOrderBook) 转换成 MarketIter<>
impl From<(ExchangeId, Instrument, OkxOrderBookL1)> for MarketIter<OrderBookL1> {
    fn from((exchange_id, instrument, books): (ExchangeId, Instrument, OkxOrderBookL1)) -> Self {
        books
            .data
            .into_iter()
            .map(|book| {
                Ok(MarketEvent {
                    exchange_time: book.ts,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: OrderBookL1 {
                        last_update_time: book.ts,
                        best_bid: Level {
                            price: book.bid_px,
                            amount: book.bid_sz,
                        },
                        best_ask: Level {
                            price: book.ask_px,
                            amount: book.ask_sz,
                        },
                    },
                })
            })
            .collect()
    }
}
