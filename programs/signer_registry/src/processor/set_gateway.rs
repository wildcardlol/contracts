use anchor_lang::prelude::*;

use crate::KeyRegistryError;
use crate::KeyRegistryGateway;

pub fn handler(ctx: Context<SetGateway>) -> Result<()> {
    let SetGateway {
        key_gateway_state,
        owner,
        new_key_gateway_program,
        ..
    } = ctx.accounts;
    require!(
        key_gateway_state.owner == owner.key(),
        KeyRegistryError::UnauthorizedOwner
    );
    require!(
        key_gateway_state.key_gateway_frozen == false,
        KeyRegistryError::GatewayFrozen
    );
    key_gateway_state.key_gateway_program = new_key_gateway_program.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct SetGateway<'info> {
    #[account(mut)]
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    /// CHECK: Gateway account responsible for registring accounts
    #[account(constraint= new_key_gateway_program.executable == true @ KeyRegistryError::GatewayIsNotProgram)]
    pub new_key_gateway_program: AccountInfo<'info>,
    pub owner: Signer<'info>,
}
