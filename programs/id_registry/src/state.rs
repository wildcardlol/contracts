use anchor_lang::prelude::*;

#[account]
pub struct IdRegistryGateway {
    pub id_gateway: Pubkey,
    pub gateway_frozen: bool,
    pub id_counter: u64,
    pub owner: Pubkey,
}
#[account]
pub struct WcidAccount {
    pub wcid: u64,
    pub custody: Pubkey,
    pub recovery: Pubkey,
}
