use super::SubKind;
use barter_integration::model::Side;
use barter_macro::{DeSubKind, SerSubKind};
use serde::{Deserialize, Serialize};

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`PublicTrade`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, DeSubKind, SerSubKind)]
pub struct Pongs;

impl SubKind for Pongs {
    type Event = Pong;
}

/// Normalised Barter [`PublicTrade`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Pong {}
