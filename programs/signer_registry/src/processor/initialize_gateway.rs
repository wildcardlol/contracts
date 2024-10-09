use crate::KeyRegistryError;
use crate::KeyRegistryGateway;
use crate::KEY_GATEWAY_SEED;
use anchor_lang::prelude::*;
use common::admin;

pub fn handler(ctx: Context<InitializeGateway>, max_keys_per_id: u16) -> Result<()> {
    let InitializeGateway {
        key_gateway_state,
        key_gateway_program,
        id_registry_program,
        owner,
        ..
    } = ctx.accounts;
    key_gateway_state.key_gateway_frozen = false;
    key_gateway_state.id_registry_program = id_registry_program.key();
    key_gateway_state.key_gateway_program = key_gateway_program.key();
    key_gateway_state.max_keys_per_id = max_keys_per_id;
    key_gateway_state.owner = owner.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGateway<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + KeyRegistryGateway::INIT_SPACE,
        seeds = [KEY_GATEWAY_SEED],
        bump
    )]
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    /// CHECK: Responsible for adding keys
    #[account(constraint= key_gateway_program.executable == true @ KeyRegistryError::GatewayIsNotProgram)]
    pub key_gateway_program: AccountInfo<'info>,
    /// CHECK: Registry Contract
    #[account(constraint= id_registry_program.executable == true @ KeyRegistryError::GatewayIsNotProgram)]
    pub id_registry_program: AccountInfo<'info>,
    #[account(mut, constraint = owner.key() == admin::ID @ KeyRegistryError::CustomError)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
