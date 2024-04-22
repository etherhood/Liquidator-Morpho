use app::{
    db::{
        event_processor::ProcessEvent,
        health_checker::HealthCheck,
        morpho_db::{FileManager, MorphoDB, MorphoDBImpl},
    },
    liquidator::trigger::trigger_liquidation,
    oracles::price_fetcher::PriceFetcher,
};
use bindings::{
    i_morpho::{
        AccrueInterestFilter, BorrowFilter, CreateMarketFilter, IMorpho, IMorphoEvents,
        LiquidateFilter, RepayFilter, SupplyCollateralFilter, SupplyFilter,
        WithdrawCollateralFilter, WithdrawFilter,
    },
    liquidator::Liquidator,
};
use dotenv::dotenv;
use ethers::{
    abi::RawLog,
    prelude::{Middleware, *},
    providers::{Http, Provider, ProviderExt, StreamExt, Ws},
    types::{Address, U64},
};
use eyre::Result;
use log::{error, info, warn};
use std::sync::Arc;

const MORPHO_ADDRESS: &str = "0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb";
const BLOCK_START: u64 = 18883124;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = Config::build()?;

    let provider = Provider::<Ws>::connect(config.wss_rpc_url.to_owned()).await?;
    let http_provider = Provider::<Http>::try_connect(&config.http_rpc_url).await?;

    let client = Arc::new(provider);

    let http_client = Arc::new(http_provider);

    let mut db: MorphoDB = MorphoDB::load_memory_db(&config.file_name)?;

    let last_block = sync_to_lastest_block(&config, &mut db, http_client.clone()).await?;
    info!("Last block: {last_block}");

    let result = subscribe(&config, &client, &mut db, last_block.into(), &http_client).await;

    match result {
        Ok(_) => info!("Listening completed"),
        Err(err) => error!("Error in listening: {}", err),
    }

    Ok(())
}

async fn subscribe(
    config: &Config,
    ws_client: &Arc<Provider<Ws>>,
    db: &mut MorphoDB,
    block_number: U64,
    client: &Arc<Provider<Http>>,
) -> Result<()> {
    let morpho = IMorpho::new(MORPHO_ADDRESS.parse::<Address>()?, ws_client.clone());

    let morpho_events = morpho.events().from_block(block_number);

    let mut morpho_events_stream = morpho_events.stream().await?;
    let mut new_block_stream = ws_client.subscribe_blocks().await?;

    info!("Started listening to events");

    info!("Started listening to new blocks");

    loop {
        tokio::select! {
            event = morpho_events_stream.next() => {
                if let Some(Ok(event)) = event {
                    let result = process_event(event, db);
                    match result {
                        Ok(_) => info!("Successfully processed events"),
                        Err(e) => error!("Error while processing event: {:?}", e)
                    }
                }
            },
            block = new_block_stream.next() => {
                info!("New block received: {:?}", block.unwrap().number.unwrap());
                let result = process_new_block(config, db, client).await;
                match result {
                    Ok(_) => info!("Successfully processed block"),
                    Err(e) => error!("Error while processing block: {:?}", e)
                }
            }
            else => break
        };
    }

    Ok(())
}

fn process_event(event: IMorphoEvents, db: &mut MorphoDB) -> Result<()> {
    match event {
        IMorphoEvents::BorrowFilter(f) => {
            info!("Borrowed by {}", f.on_behalf);
            f.process(db)
        }
        IMorphoEvents::LiquidateFilter(f) => {
            info!("Liquidated {}", f.borrower);
            f.process(db)
        }
        IMorphoEvents::RepayFilter(f) => {
            info!("Repaid for {}", f.on_behalf);
            f.process(db)
        }
        IMorphoEvents::SupplyCollateralFilter(f) => {
            info!("Supplied collateral for {}", f.on_behalf);
            f.process(db)
        }
        IMorphoEvents::WithdrawCollateralFilter(f) => {
            info!("Withdrawn collateral for {}", f.on_behalf);
            f.process(db)
        }
        IMorphoEvents::CreateMarketFilter(f) => {
            info!("Market created {:?}", f.id);
            f.process(db)
        }
        IMorphoEvents::AccrueInterestFilter(f) => {
            info!("Interest Accrued {:?}", f.id);
            f.process(db)
        }
        IMorphoEvents::SupplyFilter(f) => {
            info!("Supplied {:?}", f.id);
            f.process(db)
        }
        IMorphoEvents::WithdrawFilter(f) => {
            info!("Withdrawn {:?}", f.id);
            f.process(db)
        }
        _ => info!("Unchecked events"),
    }

    Ok(())
}

async fn process_new_block(
    config: &Config,
    db: &MorphoDB,
    client: &Arc<Provider<Http>>,
) -> Result<()> {
    let oracle_prices = db.get_all_markets().fetch_prices(client.clone()).await?;

    let market_ids = db.get_all_market_ids();

    let liquidator =
        Liquidator::new(config.liquidator_address.parse::<Address>()?, client.to_owned());

    for market_id in market_ids {
        let market_info = db.get_market(&market_id);
        let market_params = db.get_market_params(&market_id);
        let users = db.get_all_users(&market_id);
        let price = oracle_prices.get(&market_params.oracle);

        if let Some(price) = price {
            if price.is_zero() {
                continue;
            }

            for user in users {
                let position = db.get_position(&market_id, &user);
                if !position.is_healthy(&market_info, &market_params.lltv, price) {
                    warn!(
                        "Position can be liquidated for {:?} and market: {market_id:#032x}",
                        &user
                    );
                    info!("Triggering liquidation");

                    let result = trigger_liquidation(
                        &liquidator,
                        &user,
                        &position,
                        &market_params,
                        &market_info,
                    )
                    .await;
                    match result {
                        Ok(()) => info!("Successful liquidation"),
                        Err(e) => error!("Errored while liquidation: {}", e),
                    }
                }
            }
        } else {
            continue;
        };
    }

    Ok(())
}

async fn sync_to_lastest_block(
    config: &Config,
    db: &mut MorphoDB,
    client: Arc<Provider<Http>>,
) -> Result<u64> {
    let current_block: u64 = client.get_block_number().await.unwrap().try_into().unwrap();

    let mut start_block =
        if db.last_block_sync < BLOCK_START { BLOCK_START } else { db.last_block_sync };

    let mut logs: Vec<Log> = Vec::new();

    loop {
        let mut end_block = start_block + 99_000;

        if end_block > current_block {
            end_block = current_block;
        }

        info!("Syncing for blocks between {} and {}", start_block, end_block);

        let filter = Filter::new()
            .address(MORPHO_ADDRESS.parse::<Address>()?)
            .from_block(start_block)
            .to_block(end_block);

        let mut morpho_logs = client.get_logs(&filter).await?;

        logs.append(&mut morpho_logs);
        start_block += 99_000;
        if start_block > current_block {
            break;
        }
    }

    for log in logs.iter() {
        if log.topics.is_empty() {
            continue;
        }

        match log.topics[0] {
            x if x == CreateMarketFilter::signature() => {
                let event = <CreateMarketFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;

                if event.market_params.lltv.is_zero() {
                    continue;
                }
                event.process(db);
            }
            x if x == BorrowFilter::signature() => {
                let event =
                    <BorrowFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;
                event.process(db);
            }
            x if x == SupplyCollateralFilter::signature() => {
                let event = <SupplyCollateralFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;
                event.process(db);
            }
            x if x == RepayFilter::signature() => {
                let event =
                    <RepayFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;
                event.process(db);
            }
            x if x == WithdrawCollateralFilter::signature() => {
                let event = <WithdrawCollateralFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;
                event.process(db);
            }
            x if x == LiquidateFilter::signature() => {
                let event =
                    <LiquidateFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db);
            }
            x if x == SupplyFilter::signature() => {
                let event =
                    <SupplyFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db);
            }
            x if x == WithdrawFilter::signature() => {
                let event =
                    <WithdrawFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db);
            }
            x if x == AccrueInterestFilter::signature() => {
                let event = <AccrueInterestFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;

                event.process(db);
            }
            _ => {}
        }
    }

    db.last_block_sync = current_block;

    db.write_memory_db(&config.file_name)?;

    Ok(current_block)
}

struct Config {
    wss_rpc_url: String,
    http_rpc_url: String,
    file_name: String,
    liquidator_address: String,
}

impl Config {
    fn build() -> Result<Self> {
        Ok(Config {
            wss_rpc_url: get_from_config("WSS_RPC_URL".to_string())?,
            http_rpc_url: get_from_config("HTTP_RPC_URL".to_string())?,
            file_name: get_from_config("FILE_NAME".to_string())?,
            liquidator_address: get_from_config("LIQUIDATOR_ADDRESS".to_string())?,
        })
    }
}

fn get_from_config(key: String) -> Result<String> {
    Ok(std::env::var(key)?)
}
