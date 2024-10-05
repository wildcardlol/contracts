use crate::admin;
use crate::IdRegistryError;
use crate::IdRegistryGateway;
use crate::GATEWAY_STATE_SEED;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<InitializeGateway>) -> Result<()> {
    let InitializeGateway {
        registry_gateway,
        owner,
        id_gateway,
        ..
    } = ctx.accounts;
    registry_gateway.gateway_frozen = false;
    registry_gateway.id_counter = 0;
    registry_gateway.id_gateway = id_gateway.key();
    registry_gateway.owner = owner.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGateway<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 1 + 8 + 32,
        seeds = [GATEWAY_STATE_SEED],
        bump
    )]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Gateway account responsible for registring accounts
    pub id_gateway: AccountInfo<'info>,
    #[account(mut, constraint = owner.key() == admin::ID @ IdRegistryError::CustomError)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
