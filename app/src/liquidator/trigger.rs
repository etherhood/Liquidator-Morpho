use bindings::{
    i_morpho::{Market, MarketParams, Position},
    liquidator::Liquidator,
};
use ethers::prelude::*;
use eyre::Result;
use log::{info, warn};

use super::swapper::find_swap_params;

pub async fn trigger_liquidation(
    liquidator: &Liquidator<Provider<Http>>,
    user: &Address,
    position: &Position,
    market_params: &MarketParams,
    market: &Market,
) -> Result<()> {
    let swap_params = find_swap_params(market_params, position, market)?;

    let tx = liquidator
        .liquidate_user(
            market_params.to_owned(),
            user.to_owned(),
            swap_params.target,
            swap_params.seized_assets,
            swap_params.swap_data,
        )
        .send()
        .await?
        .await?;

    match tx {
        Some(receipt) => info!("Successful Transaction: {:?}", receipt),
        None => warn!("Empty transaction receipt"),
    }

    Ok(())
}
