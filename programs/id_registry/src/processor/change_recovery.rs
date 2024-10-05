use crate::IdRegistryError;
use crate::WcidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<ChangeRecovery>) -> Result<()> {
    let ChangeRecovery {
        wcid_account,
        new_recovery,
        signer,
    } = ctx.accounts;
    require!(
        signer.key() == wcid_account.custody,
        IdRegistryError::UnauthorizedCustody,
    );
    require!(
        new_recovery.key() != wcid_account.recovery,
        IdRegistryError::CannotSetSameRecovery
    );
    wcid_account.recovery = new_recovery.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct ChangeRecovery<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wcid_account: Account<'info, WcidAccount>,
    /// CHECK: New Recovery Account
    pub new_recovery: AccountInfo<'info>,
}
