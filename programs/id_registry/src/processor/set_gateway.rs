use anchor_lang::prelude::*;

use crate::IdRegistryError;
use crate::IdRegistryGateway;

pub fn handler(ctx: Context<SetGateway>) -> Result<()> {
    let SetGateway {
        registry_gateway,
        owner,
        new_gateway_program,
        ..
    } = ctx.accounts;
    require!(
        registry_gateway.owner == owner.key(),
        IdRegistryError::UnauthorizedOwner
    );
    require!(
        registry_gateway.id_gateway_frozen == false,
        IdRegistryError::GatewayFrozen
    );
    registry_gateway.id_gateway_program = new_gateway_program.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct SetGateway<'info> {
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Gateway account responsible for registring accounts
    #[account(constraint = new_gateway_program.executable == true @ IdRegistryError::KeyRegistryIsNotProgram)]
    pub new_gateway_program: AccountInfo<'info>,
    pub owner: Signer<'info>,
}
