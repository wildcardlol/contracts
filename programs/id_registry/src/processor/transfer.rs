use crate::IdRegistryError;
use crate::WcidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Transfer>) -> Result<()> {
    let Transfer {
        wcid_account,
        new_custody,
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
    wcid_account.custody = new_custody.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wcid_account: Account<'info, WcidAccount>,
    /// CHECK: New Custody Account
    pub new_custody: AccountInfo<'info>,
}
