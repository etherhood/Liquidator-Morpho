use bindings::i_morpho::{Market, MarketParams, Position};
use ethers::prelude::*;
use eyre::{Ok, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{write, File},
    io::{BufReader, Read},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MorphoDB {
    pub last_block_sync: u64,
    pub market_config: HashMap<U256, MarketParams>,
    pub market: HashMap<U256, Market>,
    pub market_positions: HashMap<U256, HashMap<Address, Position>>,
}

pub trait MorphoDBImpl {
    fn add_market(&mut self, id: U256, market_params: MarketParams);
    fn update_position(&mut self, id: &U256, user: Address, position: Position);
    fn get_position(&self, id: &U256, user: &Address) -> Position;
    fn market_exists(&self, id: &U256) -> bool;
    fn log_position(&self, id: &U256, user: &Address);
    fn get_all_markets(&self) -> Vec<MarketParams>;
    fn get_all_users(&self, id: &U256) -> Vec<Address>;
    fn get_all_positions(&self, id: &U256) -> Vec<Position>;
    fn get_all_market_ids(&self) -> Vec<U256>;
    fn get_market_params(&self, id: &U256) -> MarketParams;
    fn get_market(&self, id: &U256) -> Market;
    fn update_market(&mut self, id: U256, market: Market);
}

impl MorphoDBImpl for MorphoDB {
    fn add_market(&mut self, id: U256, market_params: MarketParams) {
        self.market_config.insert(id, market_params);
        self.market_positions.insert(id, HashMap::new());
        self.update_market(id, Market::default());
    }

    fn update_position(&mut self, id: &U256, user: Address, position: Position) {
        self.market_positions.get_mut(id).unwrap().insert(user, position);
    }

    fn get_position(&self, id: &U256, user: &Address) -> Position {
        let position = self.market_positions.get(id).unwrap().get(user);

        if let Some(position) = position {
            position.to_owned()
        } else {
            Position::default()
        }
    }

    fn market_exists(&self, id: &U256) -> bool {
        self.market_config.contains_key(id)
    }

    fn log_position(&self, id: &U256, user: &Address) {
        let position = self.get_position(id, user);
        info!(
            "User : {} with collateral: {}  and debt shares: {}",
            user, position.collateral, position.borrow_shares
        );
    }

    fn get_all_markets(&self) -> Vec<MarketParams> {
        self.market_config.values().cloned().collect()
    }

    fn get_all_positions(&self, id: &U256) -> Vec<Position> {
        self.market_positions.get(id).unwrap().values().cloned().collect()
    }

    fn get_all_users(&self, id: &U256) -> Vec<Address> {
        self.market_positions.get(id).unwrap().keys().cloned().collect()
    }

    fn get_all_market_ids(&self) -> Vec<U256> {
        self.market_config.keys().cloned().collect()
    }

    fn get_market_params(&self, id: &U256) -> MarketParams {
        self.market_config.get(id).unwrap().clone()
    }

    fn get_market(&self, id: &U256) -> Market {
        if let Some(market) = self.market.get(id) {
            market.clone()
        } else {
            Market::default()
        }
    }

    fn update_market(&mut self, id: U256, market: Market) {
        self.market.insert(id, market);
    }
}

pub trait FileManager {
    fn read_with_bufreader_str(read_path: &str) -> Result<String>;

    fn write_memory_db(&self, file_name: &str) -> Result<()>;

    fn load_memory_db(file_name: &str) -> Result<MorphoDB>;
}

impl FileManager for MorphoDB {
    fn read_with_bufreader_str(read_path: &str) -> Result<String> {
        let file = File::open(read_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut file_contents = String::new();
        buf_reader.read_to_string(&mut file_contents)?;
        Ok(file_contents)
    }

    fn load_memory_db(file_name: &str) -> Result<MorphoDB> {
        let data_json = Self::read_with_bufreader_str(file_name)?;
        Ok(serde_json::from_str(&data_json)?)
    }

    fn write_memory_db(&self, file_name: &str) -> Result<()> {
        let data_json = serde_json::to_string(self)?;
        write(file_name, data_json.as_bytes())?;
        Ok(())
    }
}
