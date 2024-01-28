use super::Okx;
use crate::{
    subscription::{
        balance::Balances, book::OrderBooksL1, mark_price::MarkPrices, option_summary::OptionSummaries, trade::PublicTrades, Subscription
    },
    Identifier,
};
use serde::Serialize;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Okx`](super::Okx) channel to be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct OkxChannel(pub &'static str);

impl OkxChannel {
    /// [`Okx`] real-time trades channel.
    ///
    /// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel>
    pub const TRADES: Self = Self("trades");

    pub const ORDER_BOOK_L1: Self = Self("tickers");
    pub const MARK_PRICE: Self = Self("mark-price");
    pub const OPTION_SUMMARY: Self = Self("opt-summary");
    pub const BALANCE: Self = Self("balance_and_position");
}

impl Identifier<OkxChannel> for Subscription<Okx, PublicTrades> {
    fn id(&self) -> OkxChannel {
        OkxChannel::TRADES
    }
}

impl Identifier<OkxChannel> for Subscription<Okx, OrderBooksL1> {
    fn id(&self) -> OkxChannel {
        OkxChannel::ORDER_BOOK_L1
    }
}

impl Identifier<OkxChannel> for Subscription<Okx, MarkPrices> {
    fn id(&self) -> OkxChannel {
        OkxChannel::MARK_PRICE
    }
}

impl Identifier<OkxChannel> for Subscription<Okx, OptionSummaries> {
    fn id(&self) -> OkxChannel {
        OkxChannel::OPTION_SUMMARY
    }
}

impl Identifier<OkxChannel> for Subscription<Okx, Balances> {
    fn id(&self) -> OkxChannel {
        OkxChannel::BALANCE
    }
}

impl AsRef<str> for OkxChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
