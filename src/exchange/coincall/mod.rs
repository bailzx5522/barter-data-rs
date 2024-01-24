use self::{
    channel::CoincallChannel, market::CoincallMarket, subscription::CoincallSubResponse,
    trade::CoincallTrades,
};
use crate::{
    exchange::{Connector, ExchangeId, ExchangeSub, PingInterval, StreamSelector},
    subscriber::{validator::WebSocketSubValidator, WebSocketSubscriber},
    subscription::{
        book::OrderBooksL1, mark_price::MarkPrices, option_summary::OptionSummaries,
        trade::PublicTrades,
    },
    transformer::stateless::StatelessTransformer,
    ExchangeWsStream,
};
use barter_integration::{error::SocketError, protocol::websocket::WsMessage};
use barter_macro::{DeExchange, SerExchange};
use serde_json::json;
use std::time::Duration;
use url::Url;

/// Defines the type that translates a Barter [`Subscription`](crate::subscription::Subscription)
/// into an exchange [`Connector`] specific channel used for generating [`Connector::requests`].
pub mod channel;
// pub mod mark;
// pub mod option_summary;
// pub mod ticker;

/// Defines the type that translates a Barter [`Subscription`](crate::subscription::Subscription)
/// into an exchange [`Connector`] specific market used for generating [`Connector::requests`].
pub mod market;

/// [`Subscription`](crate::subscription::Subscription) response type and response
/// [`Validator`](barter_integration::Validator) for [`Okx`].
pub mod subscription;

/// Public trade types for [`Okx`].
pub mod trade;

/// [`Okx`] server base url.
///
/// See docs: <https://docs.coincall.com/#options-websocket>
pub const BASE_URL_OKX: &str = "wss://ws.coincall.com/options";

/// [`Okx`] server [`PingInterval`] duration.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-connect>
pub const PING_INTERVAL_OKX: Duration = Duration::from_secs(29);

/// [`Okx`] exchange.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api>
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange,
)]
pub struct Coincall;

impl Connector for Coincall {
    const ID: ExchangeId = ExchangeId::Coincall;
    type Channel = CoincallChannel;
    type Market = CoincallMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = CoincallSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_OKX).map_err(SocketError::UrlParse)
    }

    // { "action":"heartbeat" }
    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(PING_INTERVAL_OKX),
            ping: || {
                WsMessage::Text(
                    json!({
                        "action": "heartbeat",
                    })
                    .to_string(),
                )
            },
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        vec![WsMessage::Text(
            json!({
                "action": "subscribe",
                // "dataType": "",
                "args": &exchange_subs,
            })
            .to_string(),
        )]
    }
}

impl StreamSelector<PublicTrades> for Coincall {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, CoincallTrades>>;
}

// impl StreamSelector<OrderBooksL1> for Okx {
//     type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL1, OkxOrderBookL1>>;
// }
// impl StreamSelector<MarkPrices> for Okx {
//     type Stream = ExchangeWsStream<StatelessTransformer<Self, MarkPrices, OkxMarkPrices>>;
// }
// impl StreamSelector<OptionSummaries> for Okx {
//     type Stream = ExchangeWsStream<StatelessTransformer<Self, OptionSummaries, OkxOptionSummaries>>;
// }
