use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct KeyRegistryGateway {
    pub id_registry_program: Pubkey,
    pub key_gateway_program: Pubkey,
    pub key_gateway_frozen: bool,
    pub max_keys_per_id: u16,
    pub max_flags: u8,
    pub owner: Pubkey,
}
#[account]
#[derive(InitSpace)]
pub struct KeyAccount {
    pub wid: u64,
    pub parent_key_id: u16,
    pub key_id: u16,
    pub is_admin: bool,
    pub key: KeyData,
    #[max_len(0)] // seperately allocated based on gateway state
    pub flags: Vec<bool>, // e.g, [true,true] DM's and frames etc
}
#[account]
#[derive(InitSpace)]
pub struct KeyData {
    pub key_type: u8,
    #[max_len(256)]
    pub value: Vec<u8>,
}
