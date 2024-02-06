use anyhow::Error;
use chrono::{TimeZone, Utc};
use fehler::throws;

use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::{
        self,
        binance::{futures::BinanceFuturesUsd, spot::BinanceSpot},
        kraken::Kraken,
        okx::Okx,
    },
    streams::{builder::Signer, Streams},
    subscription::{
        balance::Balances,
        book::{OrderBooksL1, OrderBooksL2},
        option_summary::OptionSummaries,
        position::Position,
        trade::PublicTrades,
    },
};
use barter_integration::model::instrument::kind::{
    InstrumentKind, OptionContract, OptionExercise, OptionKind,
};
use dotenv::var;
use tokio_stream::StreamExt;
use tracing::info;
use tracing_subscriber::fmt::init;

#[rustfmt::skip]
#[throws(Error)]
#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Notes:
    // - MarketEvent<DataKind> could use a custom enumeration if more flexibility is required.
    // - Each call to StreamBuilder::subscribe() creates a separate WebSocket connection for those
    //   Subscriptions passed.

    // Initialise MarketEvent<DataKind> Streams for various exchanges

    let signer = Signer::new(
        &var("OKEX_KEY")?,
        &var("OKEX_SECRET")?,
        &var("OKEX_PASSPHRASE")?,
    );

    // let streams = Streams::<Balances>::builder()
    //     .signer(Some(signer))
    //     .subscribe([
    //         (Okx, "eth", "usd", InstrumentKind::Spot, Balances),
    //     ])
    //     .init()
    //     .await
    //     .unwrap();

    // tokio::spawn(async move {
    //     let mut private_stream = streams.join_map().await;
    //     while let Some((exchange, data)) = private_stream.next().await {
    //         info!("private: {exchange} {data:?}");
    //     }
    // });


    let streams: Streams<MarketEvent<DataKind>> = Streams::builder_multi()
    .add(Streams::<OptionSummaries>::builder()
        .subscribe([
            // (Okx, "btc", "usd", InstrumentKind::Spot, OptionSummaries),
            (Okx, "eth", "usd", InstrumentKind::Spot, OptionSummaries),

        ])
    )
    .add(Streams::<OrderBooksL1>::builder()
        .subscribe([
            // (Okx, "btc", "usd", InstrumentKind::Spot, OptionSummaries),
            // (Okx, "btc", "usd", InstrumentKind::Option(call_contract2(1708646400000)), OrderBooksL1),
            (Okx, "eth", "usd", InstrumentKind::Option(call_contract1(1708646400000)), OrderBooksL1),

        ])
    )
    // .add(Streams::<Balances>::builder()
    //     .signer(Some(signer))
    //     .subscribe([
    //         // (Okx, "btc", "usd", InstrumentKind::Spot, OptionSummaries),
    //         (Okx, "eth", "usd", InstrumentKind::Spot, Balances),
    //     ]))
 
        .init()
        .await
        .unwrap();


    let handler = tokio::spawn(async move {
        // Join all exchange Streams into a single tokio_stream::StreamMap
        // Notes:
        //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
        //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!
        let mut joined_stream = streams.join_map().await;

        while let Some((exchange, data)) = joined_stream.next().await {
            match data.kind {
                DataKind::Position(position) => {
                    info!("Exchange: {exchange}, position update: {position:?}");
                }
                DataKind::Balance(balance)=>{
                    info!("Exchange: {exchange}, position update: {balance:?}");
                }
                DataKind::MarkPrice(mark) => {
                    info!("Exchange: {exchange}, mark price update: {mark:?}");
                }
                DataKind::OrderBookL1(bbo) => {
                    info!("Exchange: {exchange},inst_id:{}, ask:{}, bid:{}",data.instrument, bbo.best_ask.price, bbo.best_bid.price );
                }

                DataKind::OptionSummary(opt) => {
                    // info!("Exchange: {exchange}, option update");
                }
                _ => {
                    info!("Other unknown data type");
                }
            }
        }
    });
    let _ = handler.await;
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        // .json()
        // Install this Tracing subscriber as global default
        .init()
}

fn call_contract1(ts: i64) -> OptionContract {
    OptionContract {
        kind: OptionKind::Put,
        exercise: OptionExercise::American,
        expiry: Utc.timestamp_millis_opt(ts).unwrap(),
        strike: rust_decimal_macros::dec!(2500),
    }
}

fn call_contract2(ts: i64) -> OptionContract {
    OptionContract {
        kind: OptionKind::Put,
        exercise: OptionExercise::American,
        expiry: Utc.timestamp_millis_opt(ts).unwrap(),
        strike: rust_decimal_macros::dec!(38000),
    }
}
