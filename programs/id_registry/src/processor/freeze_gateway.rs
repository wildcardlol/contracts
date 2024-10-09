use anchor_lang::prelude::*;

use crate::IdRegistryError;
use crate::IdRegistryGateway;

pub fn handler(ctx: Context<FreezeGateway>) -> Result<()> {
    let FreezeGateway {
        registry_gateway,
        owner,
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
    registry_gateway.id_gateway_frozen = true;
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct FreezeGateway<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
}
