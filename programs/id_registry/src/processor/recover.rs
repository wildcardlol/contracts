use crate::IdRegistryError;
use crate::WidAccount;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Recover>) -> Result<()> {
    let Recover {
        wid_account,
        new_custody,
        signer,
    } = ctx.accounts;
    require!(
        signer.key() == wid_account.recovery,
        IdRegistryError::UnauthorizedRecoveryAccount
    );
    require!(
        new_custody.key() != wid_account.custody,
        IdRegistryError::CannotRecoverToSameCustody
    );
    wid_account.custody = new_custody.key();
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct Recover<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: New Custody Account
    pub new_custody: AccountInfo<'info>,
}
