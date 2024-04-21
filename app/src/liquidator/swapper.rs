use bindings::i_morpho::{Market, MarketParams, Position};
use ethers::prelude::*;
use eyre::{eyre, Result};

pub struct SwapParams {
    pub target: Address,
    pub swap_data: Bytes,
    pub seized_assets: U256,
}

pub fn find_swap_params(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
) -> Result<SwapParams> {
    let seized_assets = calculate_seized_assets(market_params, position, market);

    match seized_assets {
        Ok(amount) => {
            // @todo Calculate amount to liquidated
            // Call 1inch/solver APIs to fetch swap path and contract
            let result = SwapParams {
                target: Address::random(),
                swap_data: Bytes::new(),
                seized_assets: amount,
            };
            Ok(result)
        }
        Err(e) => Err(eyre!("Error in calculating seized amount: {}", e)),
    }
}

fn calculate_seized_assets(
    _market_params: &MarketParams,
    _position: &Position,
    _market: &Market,
) -> Result<U256> {
    Ok(U256::zero())
}
