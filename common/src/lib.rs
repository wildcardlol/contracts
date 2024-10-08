use anchor_lang::prelude::*;
pub mod admin {
    use anchor_lang::declare_id;
    declare_id!("FYYbupvWvzjXSjC9PA178CytPXMUqGWvbL98Jx9HEzR5");
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct KeyRegistryGateway {
    pub id_registry_program: Pubkey,
    pub key_gateway_program: Pubkey,
    pub gateway_frozen: bool,
    pub max_keys_per_id: u16,
    pub owner: Pubkey,
}
