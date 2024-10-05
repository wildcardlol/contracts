use anchor_lang::prelude::*;

use crate::admin;
use crate::IdRegistryError;
use crate::IdRegistryGateway;

pub fn handler(ctx: Context<SetGateway>) -> Result<()> {
    let SetGateway {
        registry_gateway,
        owner,
        new_id_gateway,
        ..
    } = ctx.accounts;
    require!(
        registry_gateway.owner == owner.key(),
        IdRegistryError::UnauthorizedOwner
    );
    require!(
        registry_gateway.gateway_frozen == false,
        IdRegistryError::GatewayFrozen
    );
    registry_gateway.id_gateway = new_id_gateway.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct SetGateway<'info> {
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Gateway account responsible for registring accounts
    pub new_id_gateway: AccountInfo<'info>,
    #[account(constraint = owner.key() == admin::ID @ IdRegistryError::UnauthorizedAdmin)]
    pub owner: Signer<'info>,
}
