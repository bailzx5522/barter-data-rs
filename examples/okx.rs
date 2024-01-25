use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::{
        binance::{futures::BinanceFuturesUsd, spot::BinanceSpot},
        kraken::Kraken,
        okx::Okx,
    },
    streams::Streams,
    subscription::{
        book::{OrderBooksL1, OrderBooksL2},
        option_summary::OptionSummaries,
        trade::PublicTrades,
    },
};
use barter_integration::model::instrument::kind::InstrumentKind;
use tokio_stream::StreamExt;
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Notes:
    // - MarketEvent<DataKind> could use a custom enumeration if more flexibility is required.
    // - Each call to StreamBuilder::subscribe() creates a separate WebSocket connection for those
    //   Subscriptions passed.

    // Initialise MarketEvent<DataKind> Streams for various exchanges
    let okx = Okx::new("","","");
    let streams: Streams<MarketEvent<DataKind>> = Streams::builder_multi()

    .add(Streams::<OptionSummaries>::builder()
        .subscribe([
    // (Okx, "btc", "usd", InstrumentKind::Spot, OptionSummaries),
    (okx, "eth", "usd", InstrumentKind::Spot, OptionSummaries),
    ])
    )
        .init()
        .await
        .unwrap();

    // Join all exchange Streams into a single tokio_stream::StreamMap
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!
    let mut joined_stream = streams.join_map().await;

    while let Some((exchange, data)) = joined_stream.next().await {
        info!("Exchange: {exchange}, MarketEvent<DataKind>: {data:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        .json()
        // Install this Tracing subscriber as global default
        .init()
}
