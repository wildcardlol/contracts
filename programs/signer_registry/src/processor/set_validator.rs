use anchor_lang::prelude::*;

use crate::KeyRegistryError;
use crate::KeyRegistryGateway;

pub fn handler(ctx: Context<SetValidator>, index: u8, validator_program: Pubkey) -> Result<()> {
    let SetValidator {
        key_gateway_state,
        owner,
        ..
    } = ctx.accounts;
    require!(
        key_gateway_state.owner == owner.key(),
        KeyRegistryError::UnauthorizedOwner
    );
    require!(
        (index as usize) < key_gateway_state.validators.len(),
        KeyRegistryError::InvalidKeyType
    );
    key_gateway_state.validators[index as usize] = validator_program;
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct SetValidator<'info> {
    #[account(mut)]
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub owner: Signer<'info>,
}
