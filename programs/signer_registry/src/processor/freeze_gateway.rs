use anchor_lang::prelude::*;

use crate::KeyRegistryError;
use crate::KeyRegistryGateway;

pub fn handler(ctx: Context<FreezeGateway>) -> Result<()> {
    let FreezeGateway {
        key_gateway_state,
        owner,
        ..
    } = ctx.accounts;
    require!(
        owner.key() == key_gateway_state.owner,
        KeyRegistryError::UnauthorizedOwner
    );
    require!(
        key_gateway_state.key_gateway_frozen == false,
        KeyRegistryError::GatewayFrozen
    );
    key_gateway_state.key_gateway_frozen = true;
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct FreezeGateway<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
}
