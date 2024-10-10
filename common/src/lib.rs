use anchor_lang::{prelude::*, solana_program::sysvar::instructions};
pub mod admin {
    use anchor_lang::declare_id;
    declare_id!("2QnJ1bG8NV3ZyWbbnXfG4xtDjXgSR9SfmRDioGDX74fM");
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct KeyRegistryGateway {
    pub id_registry_program: Pubkey,
    pub key_gateway_program: Pubkey,
    pub key_gateway_frozen: bool,
    pub max_keys_per_id: u16,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SignedKeyRequestMetadata {
    pub signer_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub deadline: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct KeyData {
    pub key_type: u8,
    pub value: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct KeyAccount {
    pub wid: u64,
    pub parent_key_id: u16,
    pub key_id: u16,
    pub is_admin: bool,
    pub key: KeyData,
    pub flags: Vec<bool>,
}
