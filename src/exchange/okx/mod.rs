use self::ticker::{OkxPong, OkxPongs};
use self::{
    balance::OkxBalances, channel::OkxChannel, mark::OkxMarkPrices, market::OkxMarket,
    option_summary::OkxOptionSummaries, subscription::OkxSubResponse, ticker::OkxOrderBookL1,
    trade::OkxTrades,
};
use crate::{
    exchange::{Connector, ExchangeId, ExchangeSub, PingInterval, StreamSelector},
    subscriber::{
        validator::WebSocketSubValidator, WebSocketSubscriber, WebSocketSubscriberWithLogin,
    },
    subscription::{
        balance::Balances,
        book::OrderBooksL1,
        mark_price::MarkPrices,
        option_summary::OptionSummaries,
        pong::{Pong, Pongs},
        trade::PublicTrades,
    },
    transformer::stateless::StatelessTransformer,
    ExchangeWsStream, Identifier,
};
use barter_integration::{
    error::SocketError, model::SubscriptionId, protocol::websocket::WsMessage,
};
use barter_macro::{DeExchange, SerExchange};
use serde_json::json;
use std::time::Duration;
use url::Url;

/// Defines the type that translates a Barter [`Subscription`](crate::subscription::Subscription)
/// into an exchange [`Connector`] specific channel used for generating [`Connector::requests`].
pub mod channel;
pub mod mark;
pub mod option_summary;
pub mod ticker;

/// Defines the type that translates a Barter [`Subscription`](crate::subscription::Subscription)
/// into an exchange [`Connector`] specific market used for generating [`Connector::requests`].
pub mod market;

/// [`Subscription`](crate::subscription::Subscription) response type and response
/// [`Validator`](barter_integration::Validator) for [`Okx`].
pub mod subscription;

/// Public trade types for [`Okx`].
pub mod trade;

/// Private types for [`Okx`].
pub mod balance;

/// [`Okx`] server base url.
///
/// See docs: <https://www.okx.com/docs-v5/en/#overview-api-resources-and-support>
pub const BASE_URL_OKX: &str = "wss://wsaws.okx.com:8443/ws/v5/public";
pub const PRIVATE_URL_OKX: &str = "wss://wsaws.okx.com:8443/ws/v5/private";

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
pub struct Okx;

impl Connector for Okx {
    const ID: ExchangeId = ExchangeId::Okx;
    type Channel = OkxChannel;
    type Market = OkxMarket;
    type Subscriber = WebSocketSubscriberWithLogin;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = OkxSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_OKX).map_err(SocketError::UrlParse)
    }

    fn private_url() -> Result<Url, SocketError> {
        Url::parse(PRIVATE_URL_OKX).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(PING_INTERVAL_OKX),
            ping: || WsMessage::text("ping"),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        vec![WsMessage::Text(
            json!({
                "op": "subscribe",
                "args": &exchange_subs,
            })
            .to_string(),
        )]
    }
}

impl StreamSelector<PublicTrades> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, OkxTrades>>;
}

impl StreamSelector<OrderBooksL1> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL1, OkxOrderBookL1>>;
}
impl StreamSelector<MarkPrices> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, MarkPrices, OkxMarkPrices>>;
}
impl StreamSelector<OptionSummaries> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OptionSummaries, OkxOptionSummaries>>;
}

impl StreamSelector<Balances> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Balances, OkxBalances>>;
}

impl StreamSelector<Pongs> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Pongs, OkxPongs>>;
}
