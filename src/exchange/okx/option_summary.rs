use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter}, exchange::ExchangeId, subscription::option_summary::{OptionSummaries, OptionSummary}
};

use barter_integration::model::{instrument::Instrument, Exchange, SubscriptionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type OkxOptionSummaries = OkxMessage<OkxOptionSummary>;

/// doc : https://www.okx.com/docs-v5/zh/#public-data-websocket-option-summary-channel
/// #### request
/// ```json
/// {
/// "op": "subscribe",
/// "args": [{
///     "channel": "opt-summary",
///     "instFamily": "BTC-USD"
/// }]
/// }
/// #### response
///json```
/// ```json
// {
//     "arg": {
//         "channel": "opt-summary",
//         "instFamily": "BTC-USD"
//     },
//     "data": [{
//         "instType": "OPTION",
//         "instId": "BTC-USD-200103-5500-C",
//         "uly": "BTC-USD",
//         "delta": "0.7494223636",
//         "gamma": "-0.6765419039",
//         "theta": "-0.0000809873",
//         "vega": "0.0000077307",
//         "deltaBS": "0.7494223636",
//         "gammaBS": "-0.6765419039",
//         "thetaBS": "-0.0000809873",
//         "vegaBS": "0.0000077307",
//         "realVol": "0",
//         "volLv": "0",
//         "bidVol": "",
//         "askVol": "1.5625",
//         "markVol": "0.9987",
//         "lever": "4.0342",
//         "fwdPx": "4.0342",
//         "ts": "1597026383085"
//     }]
// }
/// ```

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct OkxOptionSummary {
    #[serde(rename = "instType")]
    pub inst_type: String,

    #[serde(rename = "instId")]
    pub inst_id: String,

    #[serde(rename = "uly")]
    pub underlying: String,

    #[serde(rename = "delta", deserialize_with = "barter_integration::de::de_str")]
    pub delta: f64,
    #[serde(rename = "gamma", deserialize_with = "barter_integration::de::de_str")]
    pub gamma: f64,
    #[serde(rename = "theta", deserialize_with = "barter_integration::de::de_str")]
    pub theta: f64,
    #[serde(rename = "vega", deserialize_with = "barter_integration::de::de_str")]
    pub vega: f64,

    #[serde(rename = "volLv", deserialize_with = "barter_integration::de::de_str")]
    pub atm_vol: f64,

    #[serde(
        rename = "markVol",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub mark_vol: f64,

    #[serde(rename = "fwdPx", deserialize_with = "barter_integration::de::de_str")]
    pub forward_px: f64,

    #[serde(
        rename = "ts",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub ts: DateTime<Utc>,
}

// 实现从(ExchangId, Inst, OKxOrderBook) 转换成 MarketIter<>
impl From<(ExchangeId, Instrument, OkxOptionSummaries)> for MarketIter<OptionSummary> {
    fn from(
        (exchange_id, instrument, options): (ExchangeId, Instrument, OkxOptionSummaries),
    ) -> Self {
        options
            .data
            .into_iter()
            .map(|option| {
                Ok(MarketEvent {
                    exchange_time: option.ts,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: OptionSummary {
                        inst_id: option.inst_id,
                        underlying: option.underlying,
                        delta: option.delta,
                        gamma: option.gamma,
                        theta: option.theta,
                        vega: option.vega,
                        atm_vol: option.atm_vol,
                        mark_vol: option.mark_vol,
                        foward_px: option.forward_px,
                        ts: option.ts,
                    },
                })
            })
            .collect()
    }
}
