use super::morpho_db::{MorphoDB, MorphoDBImpl};
use bindings::i_morpho::{
    AccrueInterestFilter, BorrowFilter, CreateMarketFilter, LiquidateFilter, RepayFilter,
    SupplyCollateralFilter, SupplyFilter, WithdrawCollateralFilter, WithdrawFilter,
};
use std::collections::HashMap;

pub trait ProcessEvent {
    fn process(self, db: &mut MorphoDB);
}

impl ProcessEvent for CreateMarketFilter {
    fn process(self, db: &mut MorphoDB) {
        db.market_config.insert(self.id.into(), self.market_params);
        db.market_positions.insert(self.id.into(), HashMap::new());
    }
}

impl ProcessEvent for BorrowFilter {
    fn process(self, db: &mut MorphoDB) {
        if !db.market_exists(&self.id.into()) {
            return;
        }

        let mut position = db.get_position(&self.id.into(), &self.on_behalf);

        position.borrow_shares += self.shares.as_u128();

        let mut market_info = db.get_market(&self.id.into());

        market_info.total_borrow_assets += self.assets.as_u128();
        market_info.total_borrow_shares += self.shares.as_u128();

        db.update_position(&self.id.into(), self.on_behalf, position);

        db.update_market(self.id.into(), market_info);
    }
}

impl ProcessEvent for SupplyCollateralFilter {
    fn process(self, db: &mut MorphoDB) {
        if !db.market_exists(&self.id.into()) {
            return;
        }

        let mut position = db.get_position(&self.id.into(), &self.on_behalf);

        position.collateral += self.assets.as_u128();

        db.update_position(&self.id.into(), self.on_behalf, position);
    }
}

impl ProcessEvent for RepayFilter {
    fn process(self, db: &mut MorphoDB) {
        if !db.market_exists(&self.id.into()) {
            return;
        }

        let mut position = db.get_position(&self.id.into(), &self.on_behalf);

        position.borrow_shares -= self.shares.as_u128();
        db.update_position(&self.id.into(), self.on_behalf, position);

        let mut market_info = db.get_market(&self.id.into());

        market_info.total_borrow_assets =
            if market_info.total_borrow_assets >= self.assets.as_u128() {
                market_info.total_borrow_assets - self.assets.as_u128()
            } else {
                0
            };

        market_info.total_borrow_shares -= self.shares.as_u128();

        db.update_market(self.id.into(), market_info);
    }
}

impl ProcessEvent for WithdrawCollateralFilter {
    fn process(self, db: &mut MorphoDB) {
        if !db.market_exists(&self.id.into()) {
            return;
        }

        let mut position = db.get_position(&self.id.into(), &self.on_behalf);

        position.collateral -= self.assets.as_u128();
        db.update_position(&self.id.into(), self.on_behalf, position);
    }
}

impl ProcessEvent for LiquidateFilter {
    fn process(self, db: &mut MorphoDB) {
        if !db.market_exists(&self.id.into()) {
            return;
        }

        let mut position = db.get_position(&self.id.into(), &self.borrower);

        position.collateral -= self.seized_assets.as_u128();
        position.borrow_shares -= self.bad_debt_shares.as_u128() + self.repaid_shares.as_u128();

        db.update_position(&self.id.into(), self.borrower, position);

        let mut market_info = db.get_market(&self.id.into());

        market_info.total_borrow_shares -=
            self.repaid_shares.as_u128() + self.bad_debt_shares.as_u128();
        market_info.total_borrow_assets =
            if market_info.total_borrow_assets >= self.repaid_assets.as_u128() {
                market_info.total_borrow_assets - self.repaid_assets.as_u128()
            } else {
                0
            };

        market_info.total_borrow_assets =
            if market_info.total_borrow_assets >= self.bad_debt_assets.as_u128() {
                market_info.total_borrow_assets - self.bad_debt_assets.as_u128()
            } else {
                0
            };

        db.update_market(self.id.into(), market_info);
    }
}

impl ProcessEvent for AccrueInterestFilter {
    fn process(self, db: &mut MorphoDB) {
        let mut market_info = db.get_market(&self.id.into());

        market_info.total_borrow_assets += self.interest.as_u128();
        market_info.total_supply_assets += self.interest.as_u128();
        market_info.total_supply_shares += self.fee_shares.as_u128();

        db.update_market(self.id.into(), market_info);
    }
}

impl ProcessEvent for SupplyFilter {
    fn process(self, db: &mut MorphoDB) {
        let mut market_info = db.get_market(&self.id.into());

        market_info.total_supply_assets += self.assets.as_u128();
        market_info.total_supply_shares += self.shares.as_u128();

        db.update_market(self.id.into(), market_info);
    }
}

impl ProcessEvent for WithdrawFilter {
    fn process(self, db: &mut MorphoDB) {
        let mut market_info = db.get_market(&self.id.into());

        market_info.total_supply_shares -= self.shares.as_u128();
        market_info.total_supply_assets -= self.assets.as_u128();

        db.update_market(self.id.into(), market_info);
    }
}
