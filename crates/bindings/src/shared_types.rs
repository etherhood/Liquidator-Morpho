///`Authorization(address,address,bool,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Authorization {
    pub authorizer: ::ethers::core::types::Address,
    pub authorized: ::ethers::core::types::Address,
    pub is_authorized: bool,
    pub nonce: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
}
///`Market(uint128,uint128,uint128,uint128,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Market {
    pub total_supply_assets: u128,
    pub total_supply_shares: u128,
    pub total_borrow_assets: u128,
    pub total_borrow_shares: u128,
    pub last_update: u128,
    pub fee: u128,
}
///`MarketParams(address,address,address,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MarketParams {
    pub loan_token: ::ethers::core::types::Address,
    pub collateral_token: ::ethers::core::types::Address,
    pub oracle: ::ethers::core::types::Address,
    pub irm: ::ethers::core::types::Address,
    pub lltv: ::ethers::core::types::U256,
}
///`Signature(uint8,bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Signature {
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}
