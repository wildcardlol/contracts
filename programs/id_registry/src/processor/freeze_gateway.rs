use anchor_lang::prelude::*;

use crate::admin;
use crate::IdRegistryError;
use crate::IdRegistryGateway;

pub fn handler(ctx: Context<FreezeGateway>) -> Result<()> {
    let FreezeGateway {
        registry_gateway,
        owner,
        ..
    } = ctx.accounts;
    require!(
        owner.key() == registry_gateway.owner,
        IdRegistryError::UnauthorizedOwner
    );
    require!(
        registry_gateway.gateway_frozen == false,
        IdRegistryError::GatewayFrozen
    );
    registry_gateway.gateway_frozen = true;
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct FreezeGateway<'info> {
    #[account(constraint = owner.key() == admin::ID @ IdRegistryError::UnauthorizedAdmin)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
}
