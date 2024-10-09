use crate::{KeyAccount, KeyData, KeyRegistryError, KeyRegistryGateway, KEY_STATE_SEED};
use anchor_lang::prelude::*;
use id_registry::{
    cpi::{accounts::IncreaseWidKeyCounter, increase_wid_key_counter},
    program::IdRegistry,
    WidAccount,
};
pub fn handler(ctx: Context<Add>, key: KeyData, flags: Vec<bool>, is_admin: bool) -> Result<()> {
    ctx.accounts.enforce_key_gateway()?;
    let Add {
        wid_account,
        custody,
        key_account,
        ..
    } = ctx.accounts;
    require!(
        wid_account.custody == custody.key(),
        KeyRegistryError::UnauthorizedCustody
    );
    // todo: Validate Key
    key_account.key = key;
    key_account.wid = wid_account.wid;
    key_account.key_id = wid_account
        .key_counter
        .checked_add(1)
        .ok_or(KeyRegistryError::OverflowError)?;
    key_account.is_admin = is_admin;
    key_account.flags = flags;
    ctx.accounts.increase_wid_key_counter_ctx()?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(key: KeyData, flags: Vec<u8>)]
pub struct Add<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub custody: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 2 + 1 + (1 + key.value.len()) + flags.len(),
        seeds = [
            KEY_STATE_SEED,
            wid_account.wid.to_le_bytes().as_ref(),
            (wid_account.key_counter + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub key_account: Account<'info, KeyAccount>,
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub registry_program: Program<'info, IdRegistry>,
    pub system_program: Program<'info, System>,
    /// CHECK: Sysvar: Used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> Add<'info> {
    pub fn enforce_key_gateway(&self) -> Result<()> {
        let ix = anchor_lang::solana_program::sysvar::instructions::get_instruction_relative(
            0,
            &self.instruction_sysvar,
        )?;
        require!(
            ix.program_id == self.key_gateway_state.key_gateway_program,
            KeyRegistryError::UnauthorizedGateway
        );
        Ok(())
    }
    pub fn increase_wid_key_counter_ctx(&self) -> Result<()> {
        increase_wid_key_counter(CpiContext::new(
            self.registry_program.to_account_info(),
            IncreaseWidKeyCounter {
                wid_account: self.wid_account.to_account_info(),
                instruction_sysvar: self.instruction_sysvar.to_account_info(),
                key_gateway_state: self.key_gateway_state.to_account_info(),
            },
        ))?;
        Ok(())
    }
}
