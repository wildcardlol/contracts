use crate::IdRegistryError;
use crate::IdRegistryGateway;
use crate::GATEWAY_STATE_SEED;
use anchor_lang::prelude::*;
use common::admin;

pub fn handler(ctx: Context<InitializeGateway>) -> Result<()> {
    let InitializeGateway {
        registry_gateway,
        owner,
        gateway_program,
        ..
    } = ctx.accounts;
    registry_gateway.id_gateway_frozen = false;
    registry_gateway.id_counter = 0;
    registry_gateway.owner = owner.key();
    registry_gateway.id_gateway_program = gateway_program.key();
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
    #[account(constraint = gateway_program.executable == true @ IdRegistryError::GatewayIsNotProgram)]
    pub gateway_program: AccountInfo<'info>,
    /// Initial Owner is always admin
    #[account(mut, constraint = owner.key() == admin::ID @ IdRegistryError::UnauthorizedAdmin)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
