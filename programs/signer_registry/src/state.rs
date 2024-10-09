use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct KeyRegistryGateway {
    pub id_registry_program: Pubkey,
    pub key_gateway_program: Pubkey,
    pub key_gateway_frozen: bool,
    pub max_keys_per_id: u16,
    pub owner: Pubkey,
}
#[account]
pub struct KeyAccount {
    pub wid: u64,
    pub parent_key_id: u16,
    pub key_id: u16,
    pub is_admin: bool,
    pub key: KeyData,
    pub flags: Vec<bool>, // e.g, [1,2] DM's and frames
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct KeyData {
    pub key_type: u8,
    pub value: Vec<u8>,
}
