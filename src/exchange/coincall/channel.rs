use super::Coincall;
use crate::{
    subscription::{
        book::OrderBooksL1, mark_price::MarkPrices, option_summary::OptionSummaries,
        trade::PublicTrades, Subscription,
    },
    Identifier,
};
use serde::Serialize;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Okx`](super::Okx) channel to be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct CoincallChannel(pub &'static str);

impl CoincallChannel {
    /// [`Okx`] real-time trades channel.
    ///
    /// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel>
    pub const TRADES: Self = Self("trades");

    pub const ORDER_BOOK_L1: Self = Self("tickers");
    pub const MARK_PRICE: Self = Self("mark-price");
    pub const OPTION_SUMMARY: Self = Self("opt-summary");
}

impl Identifier<CoincallChannel> for Subscription<Coincall, PublicTrades> {
    fn id(&self) -> CoincallChannel {
        CoincallChannel::TRADES
    }
}

impl Identifier<CoincallChannel> for Subscription<Coincall, OrderBooksL1> {
    fn id(&self) -> CoincallChannel {
        CoincallChannel::ORDER_BOOK_L1
    }
}

impl Identifier<CoincallChannel> for Subscription<Coincall, MarkPrices> {
    fn id(&self) -> CoincallChannel {
        CoincallChannel::MARK_PRICE
    }
}

impl Identifier<CoincallChannel> for Subscription<Coincall, OptionSummaries> {
    fn id(&self) -> CoincallChannel {
        CoincallChannel::OPTION_SUMMARY
    }
}

impl AsRef<str> for CoincallChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
