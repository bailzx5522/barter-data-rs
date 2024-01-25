use self::{
    channel::OkxChannel, mark::OkxMarkPrices, market::OkxMarket,
    option_summary::OkxOptionSummaries, subscription::OkxSubResponse, ticker::OkxOrderBookL1,
    trade::OkxTrades,
};
use crate::{
    exchange::{Connector, ExchangeId, ExchangeSub, PingInterval, StreamSelector},
    subscriber::{
        validator::WebSocketSubValidator, WebSocketSubscriber, WebSocketSubscriberWithLogin,
    },
    subscription::{
        book::OrderBooksL1, mark_price::MarkPrices, option_summary::OptionSummaries,
        trade::PublicTrades,
    },
    transformer::stateless::StatelessTransformer,
    ExchangeWsStream,
};
use barter_integration::{error::SocketError, protocol::websocket::WsMessage};
use barter_macro::{DeExchange, SerExchange};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use url::Url;

use super::Login;

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

/// [`Okx`] server base url.
///
/// See docs: <https://www.okx.com/docs-v5/en/#overview-api-resources-and-support>
pub const BASE_URL_OKX: &str = "wss://wsaws.okx.com:8443/ws/v5/public";

/// [`Okx`] server [`PingInterval`] duration.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-connect>
pub const PING_INTERVAL_OKX: Duration = Duration::from_secs(29);

/// [`Okx`] exchange.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange)]
pub struct Okx;

impl Okx {
    //     // pub fn new(access_key: &str, secret_key: &str, passphrase: &str) -> Self {
    //     //     Self {
    //     //         access_key: String::from(access_key),
    //     //         secret_key: String::from(secret_key),
    //     //         passphrase: String::from(passphrase),
    //     //     }
    //     // }
    //     pub fn signature(&self) -> (&str, String) {
    //         // let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
    //         // let sign_message = match url.query() {
    //         //     Some(query) => format!(
    //         //         "{}{}{}?{}{}",
    //         //         timestamp,
    //         //         method.as_str(),
    //         //         url.path(),
    //         //         query,
    //         //         body
    //         //     ),
    //         //     None => format!("{}{}{}{}", timestamp, method.as_str(), url.path(), body),
    //         // };

    //         // let signature = encode(hmac::sign(&signed_key, sign_message.as_bytes()).as_ref());
    //         (self.access_key.as_str(), String::from("123"))
    //     }
    fn login_request(&self) {
        print!("-------------- login request");
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginArgs {
    api_key: String,
    passphrase: String,
    timestamp: String,
    sign: String,
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
