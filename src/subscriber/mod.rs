use self::{
    mapper::{SubscriptionMapper, WebSocketSubMapper},
    validator::SubscriptionValidator,
};
use crate::{
    exchange::Connector,
    streams::builder::Signer,
    subscription::{Map, SubKind, Subscription, SubscriptionMeta},
    Identifier,
};
use async_trait::async_trait;
use barter_integration::{
    error::SocketError,
    model::instrument::Instrument,
    protocol::websocket::{connect, WebSocket},
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// [`SubscriptionMapper`](mapper::SubscriptionMapper) implementations defining how to map a
/// collection of Barter [`Subscription`]s into exchange specific [`SubscriptionMeta`].
pub mod mapper;

/// [`SubscriptionValidator`](validator::SubscriptionValidator) implementations defining how to
/// validate actioned [`Subscription`]s were successful.
pub mod validator;

/// Defines how to connect to a socket and subscribe to market data streams.
#[async_trait]
pub trait Subscriber {
    type SubMapper: SubscriptionMapper;

    async fn subscribe<Exchange, Kind>(
        signer: &Option<Signer>,
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> Result<(WebSocket, Map<Instrument>), SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

/// Standard [`Subscriber`] for [`WebSocket`]s suitable for most exchanges.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct WebSocketSubscriber;

#[async_trait]
impl Subscriber for WebSocketSubscriber {
    type SubMapper = WebSocketSubMapper;

    async fn subscribe<Exchange, Kind>(
        _signer: &Option<Signer>,
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> Result<(WebSocket, Map<Instrument>), SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        // Define variables for logging ergonomics
        let exchange = Exchange::ID;
        let url = Exchange::url()?;
        debug!(%exchange, %url, ?subscriptions, "subscribing to WebSocket");

        // Connect to exchange
        let mut websocket = connect(url).await?;
        debug!(%exchange, ?subscriptions, "connected to WebSocket");

        // Map &[Subscription<Exchange, Kind>] to SubscriptionMeta
        let SubscriptionMeta {
            instrument_map,
            subscriptions,
        } = Self::SubMapper::map::<Exchange, Kind>(subscriptions);

        // Send Subscriptions over WebSocket
        for subscription in subscriptions {
            debug!(%exchange, payload = ?subscription, "sending exchange subscription");
            websocket.send(subscription).await?;
        }

        // Validate Subscription responses
        let map =
            Exchange::SubValidator::validate::<Exchange, Kind>(instrument_map, &mut websocket)
                .await?;

        info!(%exchange, "subscribed to WebSocket");
        Ok((websocket, map))
    }
}

/// Standard [`Subscriber`] for [`WebSocket`]s suitable for most exchanges.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct WebSocketSubscriberWithLogin;

#[async_trait]
impl Subscriber for WebSocketSubscriberWithLogin {
    type SubMapper = WebSocketSubMapper;

    async fn subscribe<Exchange, Kind>(
        signer: &Option<Signer>,
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> Result<(WebSocket, Map<Instrument>), SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        // Define variables for logging ergonomics
        let mut is_private = false;
        let s = signer;
        match s {
            Some(s) => {
                is_private = true;
            }
            None => {}
        };
        let exchange = Exchange::ID;
        let mut url = Exchange::url()?;
        if is_private == true {
            url = Exchange::private_url()?;
        }
        debug!(%exchange, %url, ?subscriptions, "subscribing to WebSocket");

        // Connect to exchange
        let mut websocket = connect(url).await?;
        debug!(%exchange, ?subscriptions, "connected to WebSocket");

        // Map &[Subscription<Exchange, Kind>] to SubscriptionMeta
        let SubscriptionMeta {
            instrument_map,
            subscriptions,
        } = Self::SubMapper::map::<Exchange, Kind>(subscriptions);

        match signer {
            Some(s) => {
                s.login_request(&mut websocket).await?;
            }
            None => {}
        };

        // Send Subscriptions over WebSocket
        for subscription in subscriptions {
            debug!(%exchange, payload = ?subscription, "sending exchange subscription");
            websocket.send(subscription).await?;
        }

        // Validate Subscription responses
        let map =
            Exchange::SubValidator::validate::<Exchange, Kind>(instrument_map, &mut websocket)
                .await?;

        info!(%exchange, "subscribed to WebSocket");
        Ok((websocket, map))
    }
}
