use crate::IdRegistryError;
use crate::WidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<ChangeRecovery>) -> Result<()> {
    let ChangeRecovery {
        wid_account,
        new_recovery,
        signer,
    } = ctx.accounts;
    require!(
        signer.key() == wid_account.custody,
        IdRegistryError::UnauthorizedCustody,
    );
    require!(
        new_recovery.key() != wid_account.recovery,
        IdRegistryError::CannotSetSameRecovery
    );
    wid_account.recovery = new_recovery.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct ChangeRecovery<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: New Recovery Account
    pub new_recovery: AccountInfo<'info>,
}
