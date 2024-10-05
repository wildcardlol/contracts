use crate::IdRegistryError;
use crate::WcidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<TransferWithRecovery>) -> Result<()> {
    let TransferWithRecovery {
        wcid_account,
        new_custody,
        new_recovery,
        signer,
    } = ctx.accounts;
    require!(
        signer.key() == wcid_account.custody,
        IdRegistryError::UnauthorizedCustody
    );
    require!(
        new_custody.key() != wcid_account.custody,
        IdRegistryError::CannotTransferToSameCustody
    );
    require!(
        new_recovery.key() != new_custody.key(),
        IdRegistryError::SameRecoveryAndCustodyAddress
    );
    wcid_account.custody = new_custody.key();
    wcid_account.recovery = new_recovery.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct TransferWithRecovery<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wcid_account: Account<'info, WcidAccount>,
    /// CHECK: New Custody Account
    pub new_custody: AccountInfo<'info>,
    /// CHECK: New Recovery Account
    pub new_recovery: AccountInfo<'info>,
}
