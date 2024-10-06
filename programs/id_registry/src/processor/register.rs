use crate::IdRegistryError;
use crate::IdRegistryGateway;
use crate::RegisterEvent;
use crate::WcidAccount;
use crate::WCID_STATE_SEED;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Register>) -> Result<()> {
    let Register {
        registry_gateway,
        id_gateway,
        custody_account,
        recovery_account,
        wcid_account,
        ..
    } = ctx.accounts;
    require!(
        registry_gateway.id_gateway == id_gateway.key(),
        IdRegistryError::UnauthorizedGateway
    );
    let wcid = registry_gateway
        .id_counter
        .checked_add(1)
        .ok_or(IdRegistryError::OverflowError)?;

    registry_gateway.id_counter = wcid;
    wcid_account.wcid = wcid;
    wcid_account.custody = custody_account.key();
    wcid_account.recovery = recovery_account.key();
    emit!(RegisterEvent {
        custody: custody_account.key(),
        recovery: recovery_account.key(),
        wcid,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub id_gateway: Signer<'info>,
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Custody Account
    pub custody_account: AccountInfo<'info>,
    #[account(
        constraint =
        custody_account.key() != recovery_account.key() @ IdRegistryError::SameRecoveryAndCustodyAddress
    )]
    /// CHECK: Recovery Account
    pub recovery_account: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 32 + 32,
        seeds = [WCID_STATE_SEED, (registry_gateway.id_counter + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub wcid_account: Account<'info, WcidAccount>,
    pub system_program: Program<'info, System>,
}
