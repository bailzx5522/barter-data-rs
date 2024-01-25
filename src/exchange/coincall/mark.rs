use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{subscription::ExchangeSub, ExchangeId},
    subscription::{
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

pub type OkxMarkPrices = OkxMessage<OkxMarkPrice>;

/// doc : https://www.okx.com/docs-v5/zh/#order-book-trading-market-data-ws-tickers-channel
/// ```json
// {
//     "arg": {
//       "channel": "mark-price",
//       "instId": "BTC-USDT"
//     },
//     "data": [
//       {
//         "instType": "MARGIN",
//         "instId": "BTC-USDT",
//         "markPx": "42310.6",
//         "ts": "1630049139746"
//       }
//     ]
//   }
/// ```

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct OkxMarkPrice {
    #[serde(
        rename = "ts",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,

    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "markPx", deserialize_with = "barter_integration::de::de_str")]
    pub mark_px: f64,
}

// 实现从(ExchangId, Inst, OKxOrderBook) 转换成 MarketIter<>
impl From<(ExchangeId, Instrument, OkxMarkPrices)> for MarketIter<MarkPrice> {
    fn from((exchange_id, instrument, marks): (ExchangeId, Instrument, OkxMarkPrices)) -> Self {
        marks
            .data
            .into_iter()
            .map(|mark| {
                Ok(MarketEvent {
                    exchange_time: mark.ts,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: MarkPrice {
                        ts: mark.ts,
                        px: mark.mark_px,
                    },
                })
            })
            .collect()
    }
}
