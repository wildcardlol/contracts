use anchor_lang::prelude::*;

use crate::KeyRegistryError;
use crate::KeyRegistryGateway;

pub fn handler(ctx: Context<AddValidator>, validator_program: Pubkey) -> Result<()> {
    let AddValidator {
        key_gateway_state,
        owner,
        ..
    } = ctx.accounts;
    require!(
        key_gateway_state.owner == owner.key(),
        KeyRegistryError::UnauthorizedOwner
    );
    key_gateway_state.validators.push(validator_program);
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct AddValidator<'info> {
    #[account(mut)]
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub owner: Signer<'info>,
}
