use crate::{KeyAccount, KeyData, KeyRegistryError, KeyRegistryGateway, KEY_STATE_SEED};
use anchor_lang::prelude::*;
use common::SignedKeyRequestMetadata;
use id_registry::{
    cpi::{accounts::IncreaseWidKeyCounter, increase_wid_key_counter},
    program::IdRegistry,
    WidAccount,
};
pub fn handler(
    ctx: Context<AddViaAdmin>,
    key: KeyData,
    metadata: SignedKeyRequestMetadata,
    flags: Vec<bool>,
    is_admin: bool,
) -> Result<()> {
    ctx.accounts.enforce_key_gateway()?;
    ctx.accounts.validate_metadata_cpi(metadata)?;
    let AddViaAdmin {
        wid_account,
        key_account,
        id_registry_program,
        key_gateway_state,
        instruction_sysvar,
        parent_key_account,
        ..
    } = ctx.accounts;

    require!(
        flags.len() <= key_gateway_state.default_flags.len() as usize,
        KeyRegistryError::FlagsLengthExceeded
    );
    require!(
        key.key_type as usize >= key_gateway_state.validators.len(),
        KeyRegistryError::InvalidKeyType
    );
    require!(
        key.value.len() <= 256,
        KeyRegistryError::KeyValueLengthExceeded
    );

    increase_wid_key_counter(CpiContext::new(
        id_registry_program.to_account_info(),
        IncreaseWidKeyCounter {
            wid_account: wid_account.to_account_info(),
            instruction_sysvar: instruction_sysvar.to_account_info(),
            key_gateway_state: key_gateway_state.to_account_info(),
        },
    ))?;
    key_account.set_inner_admin(
        flags,
        is_admin,
        key,
        wid_account.key_counter + 1,
        parent_key_account,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct AddViaAdmin<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub parent_key_account: Account<'info, KeyAccount>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    #[account(
        init,
        payer = payer,
        space = 8 + KeyAccount::INIT_SPACE + key_gateway_state.default_flags.len() as usize,
        seeds = [
            KEY_STATE_SEED,
            wid_account.wid.to_le_bytes().as_ref(),
            wid_account.key_counter.checked_add(1).ok_or(KeyRegistryError::OverflowError)?.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub key_account: Account<'info, KeyAccount>,
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub id_registry_program: Program<'info, IdRegistry>,
    pub system_program: Program<'info, System>,
    /// CHECK: Its dynamic
    #[account(constraint = validator_program.executable == true @ KeyRegistryError::ValidatorKeyIsNotProgram)]
    pub validator_program: UncheckedAccount<'info>,
    /// CHECK: Sysvar: Used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> AddViaAdmin<'info> {
    pub fn enforce_key_gateway(&self) -> Result<()> {
        let ix = anchor_lang::solana_program::sysvar::instructions::get_instruction_relative(
            0,
            &self.instruction_sysvar,
        )?;
        require!(
            ix.program_id == self.key_gateway_state.key_gateway_program,
            KeyRegistryError::UnauthorizedGateway
        );
        Ok(())
    }
    pub fn validate_metadata_cpi(&self, metadata: SignedKeyRequestMetadata) -> Result<()> {
        let key_type = self.parent_key_account.key.key_type;
        let validators = &self.key_gateway_state.validators;
        require!(
            validators[key_type as usize] == self.validator_program.key(),
            KeyRegistryError::InvalidValidatorProgram
        );
        signed_key_request_validator::cpi::validate(
            CpiContext::new(
                self.validator_program.to_account_info(),
                signed_key_request_validator::cpi::accounts::Validate {
                    parent_key_account: self.parent_key_account.to_account_info(),
                    instruction_sysvar: self.instruction_sysvar.to_account_info(),
                    key_gateway_state: self.key_gateway_state.to_account_info(),
                },
            ),
            metadata,
        )?;
        Ok(())
    }
}
