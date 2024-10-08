use crate::IdRegistryError;
use crate::WidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Transfer>) -> Result<()> {
    let Transfer {
        wid_account,
        new_custody,
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
    wid_account.custody = new_custody.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: New Custody Account
    pub new_custody: AccountInfo<'info>,
}
