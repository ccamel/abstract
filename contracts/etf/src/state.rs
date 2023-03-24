use abstract_os::objects::fee::Fee;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

/// State stores LP token address
/// BaseState is initialized in contract
#[cosmwasm_schema::cw_serde]
pub struct State {
    pub liquidity_token_addr: Addr,
    pub manager_addr: Addr,
}

pub const STATE: Item<State> = Item::new("\u{0}{5}state");
pub const FEE: Item<Fee> = Item::new("\u{0}{3}fee");
