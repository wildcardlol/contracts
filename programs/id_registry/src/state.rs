use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IdRegistryGateway {
    pub id_gateway_program: Pubkey,
    pub id_gateway_frozen: bool,
    pub id_counter: u64,
    pub owner: Pubkey,
}
#[account]
#[derive(InitSpace)]
pub struct WidAccount {
    pub wid: u64,
    pub custody: Pubkey,
    pub recovery: Pubkey,
    pub key_counter: u16,
}
