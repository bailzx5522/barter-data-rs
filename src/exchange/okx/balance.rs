use super::trade::OkxMessage;
use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{subscription::ExchangeSub, ExchangeId},
    subscription::{
        account::{Account, Accounts},
        balance::{Balance, BalanceData, Balances, PositionData, Trade},
        book::{Level, OrderBookL1},
        mark_price::{MarkPrice, MarkPrices},
        position::{Position, Positions},
    },
    Identifier,
};

use barter_integration::{
    de::extract_next,
    model::{instrument::Instrument, Exchange, Side, SubscriptionId},
};
use chrono::{DateTime, Timelike, Utc};
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

/// 获取持仓信息，首次订阅按照订阅维度推送数据，此外，当下单、撤单等事件触发时
/// {
///    "op": "subscribe",
///    "args": [{
///        "channel": "positions",
///        "instType": "FUTURES",
///        "instFamily": "BTC-USD",
///        "instId": "BTC-USD-200329"
///    }]
///}
///
///{
///    "arg":{
///        "channel":"positions",
///        "uid": "77982378738415879",
///        "instType":"FUTURES"
///    },
///    "data":[
///        {
///            "adl":"1",
///            "availPos":"1",
///            "avgPx":"2566.31",
///            "cTime":"1619507758793",
///            "ccy":"ETH",
///            "deltaBS":"",
///            "deltaPA":"",
///            "gammaBS":"",
///            "gammaPA":"",
///            "imr":"",
///            "instId":"ETH-USD-210430",
///            "instType":"FUTURES",
///            "interest":"0",
///            "idxPx":"2566.13",
///            "last":"2566.22",
///            "lever":"10",
///            "liab":"",
///            "liabCcy":"",
///            "liqPx":"2352.8496681818233",
///            "markPx":"2353.849",
///            "margin":"0.0003896645377994",
///            "mgnMode":"isolated",
///            "mgnRatio":"11.731726509588816",
///            "mmr":"0.0000311811092368",
///            "notionalUsd":"2276.2546609009605",
///            "optVal":"",
///            "pTime":"1619507761462",
///            "pos":"1",
///            "baseBorrowed": "",
///            "baseInterest": "",
///            "quoteBorrowed": "",
///            "quoteInterest": "",
///            "posCcy":"",
///            "posId":"307173036051017730",
///            "posSide":"long",
///            "spotInUseAmt": "",
///            "spotInUseCcy": "",
///            "bizRefId": "",
///            "bizRefType": "",
///            "thetaBS":"",
///            "thetaPA":"",
///            "tradeId":"109844",
///            "uTime":"1619507761462",
///            "upl":"-0.0000009932766034",
///            "uplLastPx":"-0.0000009932766034",
///            "uplRatio":"-0.0025490556801078",
///            "uplRatioLastPx":"-0.0025490556801078",
///            "vegaBS":"",
///            "vegaPA":"",
///            "realizedPnl":"0.001",
///            "pnl":"0.0011",
///            "fee":"-0.0001",
///            "fundingFee":"0",
///            "liqPenalty":"0",
///            "closeOrderAlgo":[
///                {
///                    "algoId":"123",
///                    "slTriggerPx":"123",
///                    "slTriggerPxType":"mark",
///                    "tpTriggerPx":"123",
///                    "tpTriggerPxType":"mark",
///                    "closeFraction":"0.6"
///                },
///                {
///                    "algoId":"123",
///                    "slTriggerPx":"123",
///                    "slTriggerPxType":"mark",
///                    "tpTriggerPx":"123",
///                    "tpTriggerPxType":"mark",
///                    "closeFraction":"0.4"
///                }
///            ]
///        }
///    ]
///}

pub type OkxPositions = OkxMessage<OkxPosition>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkxPosition {
    #[serde(
        rename = "cTime",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub c_time: DateTime<Utc>,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub u_time: i64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub p_time: i64,
    pub inst_id: String,
    pub pos_id: String,
    pub pos_side: Side,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub pos: f64,
    pub ccy: String,
}

impl From<(ExchangeId, Instrument, OkxPositions)> for MarketIter<Position> {
    fn from((exchange_id, instrument, positions): (ExchangeId, Instrument, OkxPositions)) -> Self {
        positions
            .data
            .into_iter()
            .map(|pos| {
                Ok(MarketEvent {
                    exchange_time: pos.c_time,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: Position {
                        ccy: pos.ccy,
                        c_time: 0,
                        inst_id: pos.inst_id,
                        pos_side: pos.pos_side,
                        pos: pos.pos,
                    },
                })
            })
            .collect()
    }
}

/// 获取账户信息，首次订阅按照订阅维度推送数据，此外，当下单、撤单、成交等事件触发时
/// {
/// "op": "subscribe",
/// "args": [{
///     "channel": "account",
///     "ccy": "BTC"
/// }]
/// }
///
/// {
///"arg": {
///    "channel": "account",
///    "uid": "44*********584"
///},
///"data": [{
///    "adjEq": "55444.12216906034",
///    "borrowFroz": "0",
///    "details": [{
///        "availBal": "4734.371190691436",
///        "availEq": "4734.371190691435",
///        "borrowFroz": "0",
///        "cashBal": "4750.426970691436",
///        "ccy": "USDT",
///        "coinUsdPrice": "0.99927",
///        "crossLiab": "0",
///        "disEq": "4889.379316336831",
///        "eq": "4892.951170691435",
///        "eqUsd": "4889.379316336831",
///        "fixedBal": "0",
///        "frozenBal": "158.57998",
///        "imr": "",
///        "interest": "0",
///        "isoEq": "0",
///        "isoLiab": "0",
///        "isoUpl": "0",
///        "liab": "0",
///        "maxLoan": "0",
///        "mgnRatio": "",
///        "mmr": "",
///        "notionalLever": "",
///        "ordFrozen": "0",
///        "spotInUseAmt": "",
///        "spotIsoBal": "0",
///        "stgyEq": "150",
///        "twap": "0",
///        "uTime": "1705564213903",
///        "upl": "-7.475800000000003",
///        "uplLiab": "0"
///    }],
///    "imr": "8.5737166146",
///    "isoEq": "0",
///    "mgnRatio": "143705.65988369548",
///    "mmr": "0.342948664584",
///    "notionalUsd": "85.737166146",
///    "ordFroz": "0",
///    "totalEq": "55868.06403501676",
///    "uTime": "1705564223311",
///    "upl": "-7.470342666000003"
///}]
///}

pub type OkxAccounts = OkxMessage<OkxAccount>;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkxAccount {
    #[serde(
        rename = "uTime",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub u_time: DateTime<Utc>,

    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub imr: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mmr: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub total_eq: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub upl: f64,
}

impl From<(ExchangeId, Instrument, OkxAccounts)> for MarketIter<Account> {
    fn from((exchange_id, instrument, accounts): (ExchangeId, Instrument, OkxAccounts)) -> Self {
        accounts
            .data
            .into_iter()
            .map(|account| {
                Ok(MarketEvent {
                    exchange_time: account.u_time,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: Account {
                        total_eq: account.total_eq,
                        imr: account.imr,
                        mmr: account.mmr,
                        u_time: 0,
                    },
                })
            })
            .collect()
    }
}
