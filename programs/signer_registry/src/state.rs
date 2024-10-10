use anchor_lang::prelude::*;

use crate::KeyRegistryError;

#[account]
#[derive(InitSpace)]
pub struct KeyRegistryGateway {
    pub id_registry_program: Pubkey,
    pub key_gateway_program: Pubkey,
    pub key_gateway_frozen: bool,
    pub max_keys_per_id: u16,
    #[max_len(0)] // allocated while initializing
    pub default_flags: Vec<bool>,
    pub owner: Pubkey,
    #[max_len(0)] // allocated while initializing
    pub validators: Vec<Pubkey>,
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

impl KeyAccount {
    pub fn set_inner_custody(
        &mut self,
        flags: Vec<bool>,
        is_admin: bool,
        key: KeyData,
        key_id: u16,
        wid: u64,
    ) {
        self.flags = flags;
        self.is_admin = is_admin;
        self.key = key;
        self.key_id = key_id;
        self.wid = wid;
        self.parent_key_id = 0;
    }
    pub fn set_inner_admin(
        &mut self,
        flags: Vec<bool>,
        is_admin: bool,
        key: KeyData,
        key_id: u16,
        parent_key_account: &KeyAccount,
    ) -> Result<()> {
        for (i, &flag) in flags.iter().enumerate() {
            if flag {
                require!(
                    parent_key_account.flags[i] == true,
                    KeyRegistryError::InvalidFlagsSetByAdmin
                );
            }
        }
        self.flags = flags;
        self.is_admin = is_admin;
        self.key = key;
        self.key_id = key_id;
        self.parent_key_id = parent_key_account.key_id;
        self.wid = parent_key_account.wid;
        Ok(())
    }
}
