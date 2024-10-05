use anchor_lang::prelude::*;

#[event]
pub struct RegisterEvent {
    pub wcid: u64,
    pub custody: Pubkey,
    pub recovery: Pubkey,
}
