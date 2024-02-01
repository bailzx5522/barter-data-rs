use crate::{
    error::DataError,
    subscription::{
        account::Account,
        balance::Balance,
        book::{OrderBook, OrderBookL1},
        candle::Candle,
        greek::Greek,
        liquidation::Liquidation,
        mark_price::MarkPrice,
        option_summary::OptionSummary,
        pong::Pong,
        position::Position,
        trade::PublicTrade,
        SubKind,
    },
};
use barter_integration::model::{instrument::Instrument, Exchange};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Convenient new type containing a collection of [`MarketEvent<T>`](MarketEvent)s.
#[derive(Debug)]
pub struct MarketIter<T>(pub Vec<Result<MarketEvent<T>, DataError>>);

impl<T> FromIterator<Result<MarketEvent<T>, DataError>> for MarketIter<T> {
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = Result<MarketEvent<T>, DataError>>,
    {
        Self(iter.into_iter().collect())
    }
}

/// Normalised Barter [`MarketEvent<T>`](Self) wrapping the `T` data variant in metadata.
///
/// Note: `T` can be an enum such as the [`DataKind`] if required.
///
/// See [`crate::subscription`] for all existing Barter Market event variants.
///
/// ### Examples
/// - [`MarketEvent<PublicTrade>`](crate::subscription::trade::PublicTrade)
/// - [`MarketEvent<OrderBookL1>`](crate::subscription::book::OrderBookL1)
/// - [`MarketEvent<DataKind>`](DataKind)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct MarketEvent<T> {
    pub exchange_time: DateTime<Utc>,
    pub received_time: DateTime<Utc>,
    pub exchange: Exchange,
    pub instrument: Instrument,
    pub kind: T,
}

/// Available kinds of normalised Barter [`MarketEvent<T>`](MarketEvent).
///
/// ### Notes
/// - [`Self`] is only used as the [`MarketEvent<DataKind>`](MarketEvent) `Output` when combining
///   several [`Streams<SubKind::Event>`](crate::streams::Streams) using the
///   [`MultiStreamBuilder<Output>`](crate::streams::builder::multi::MultiStreamBuilder).
/// - [`Self`] is purposefully not supported in any
///   [`Subscription`](crate::subscription::Subscription)s directly, it is only used to
///   make ergonomic [`Streams`](crate::streams::Streams) containing many
///   [`MarketEvent<T>`](MarketEvent) kinds.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub enum DataKind {
    Pong(Pong),
    Trade(PublicTrade),
    OrderBookL1(OrderBookL1),
    OrderBook(OrderBook),
    Candle(Candle),
    Liquidation(Liquidation),

    MarkPrice(MarkPrice),
    OptionSummary(OptionSummary),

    Account(Account),
    Balance(Balance),
    Greek(Greek),
    Position(Position),
}

impl DataKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataKind::Trade(_) => "trade",
            DataKind::OrderBook(_) => "orderbook",
            DataKind::MarkPrice(_) => "mark",
            DataKind::Account(_) => "account",
            DataKind::Position(_) => "positions",
            DataKind::Balance(_) => "balance_and_position",
            DataKind::OptionSummary(_) => "opt-summary",
            _ => "",
        }
    }
}

impl From<MarketEvent<PublicTrade>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<PublicTrade>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Trade(event.kind),
        }
    }
}

impl From<MarketEvent<Pong>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<Pong>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Pong(event.kind),
        }
    }
}

impl From<MarketEvent<OrderBookL1>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<OrderBookL1>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::OrderBookL1(event.kind),
        }
    }
}

impl From<MarketEvent<OrderBook>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<OrderBook>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::OrderBook(event.kind),
        }
    }
}

impl From<MarketEvent<Candle>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<Candle>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Candle(event.kind),
        }
    }
}

impl From<MarketEvent<Liquidation>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<Liquidation>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Liquidation(event.kind),
        }
    }
}

impl From<MarketEvent<MarkPrice>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<MarkPrice>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::MarkPrice(event.kind),
        }
    }
}

impl From<MarketEvent<OptionSummary>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<OptionSummary>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::OptionSummary(event.kind),
        }
    }
}

impl From<MarketEvent<Balance>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<Balance>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Balance(event.kind),
        }
    }
}

impl From<MarketEvent<Position>> for MarketEvent<DataKind> {
    fn from(event: MarketEvent<Position>) -> Self {
        Self {
            exchange_time: event.exchange_time,
            received_time: event.received_time,
            exchange: event.exchange,
            instrument: event.instrument,
            kind: DataKind::Position(event.kind),
        }
    }
}
