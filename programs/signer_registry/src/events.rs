use anchor_lang::prelude::*;

#[event]
pub struct RegisterEvent {
    pub wid: u64,
    pub custody: Pubkey,
    pub recovery: Pubkey,
}
