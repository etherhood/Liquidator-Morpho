use bindings::i_morpho::{Market, Position};
use ethers::prelude::*;

pub trait HealthCheck {
    fn is_healthy(&self, market: &Market, lltv: &U256, price: &U256) -> bool;
}

const VIRTUAL_ASSETS: u128 = 1;
const VIRTUAL_SHARES: u128 = 1_000_000;

impl HealthCheck for Position {
    fn is_healthy(&self, market: &Market, lltv: &U256, price: &U256) -> bool {
        let max_borrow = (U256::from(self.collateral) * price / U256::exp10(36)).as_u128()
            * lltv.as_u128()
            / U256::exp10(18).as_u128();

        let borrowed_assets = self.borrow_shares * (market.total_borrow_assets + VIRTUAL_ASSETS)
            / (market.total_borrow_shares + VIRTUAL_SHARES);

        max_borrow >= borrowed_assets
    }
}
