use crate::IdRegistryError;
use crate::WidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<TransferWithRecovery>) -> Result<()> {
    let TransferWithRecovery {
        wid_account,
        new_custody,
        new_recovery,
        signer,
    } = ctx.accounts;
    require!(
        signer.key() == wid_account.custody,
        IdRegistryError::UnauthorizedCustody
    );
    require!(
        new_custody.key() != wid_account.custody,
        IdRegistryError::CannotTransferToSameCustody
    );
    require!(
        new_recovery.key() != new_custody.key(),
        IdRegistryError::SameRecoveryAndCustodyAddress
    );
    wid_account.custody = new_custody.key();
    wid_account.recovery = new_recovery.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct TransferWithRecovery<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: New Custody Account
    pub new_custody: AccountInfo<'info>,
    /// CHECK: New Recovery Account
    pub new_recovery: AccountInfo<'info>,
}
