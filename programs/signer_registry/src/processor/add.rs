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
        id_registry_program,
        key_gateway_state,
        instruction_sysvar,
        ..
    } = ctx.accounts;
    require!(
        wid_account.custody == custody.key(),
        KeyRegistryError::UnauthorizedCustody
    );
    require!(
        flags.len() <= key_gateway_state.max_flags as usize,
        KeyRegistryError::FlagsLengthExceeded
    );
    require!(
        key.value.len() <= 256,
        KeyRegistryError::KeyValueLengthExceeded
    );

    increase_wid_key_counter(CpiContext::new(
        id_registry_program.to_account_info(),
        IncreaseWidKeyCounter {
            wid_account: wid_account.to_account_info(),
            instruction_sysvar: instruction_sysvar.to_account_info(),
            key_gateway_state: key_gateway_state.to_account_info(),
        },
    ))?;
    key_account.key = key;
    key_account.parent_key_id = 0;
    key_account.wid = wid_account.wid;
    key_account.key_id = wid_account.key_counter + 1; // check is already done
    key_account.is_admin = is_admin;
    key_account.flags = flags;
    Ok(())
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub custody: Signer<'info>,
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    #[account(
        init,
        payer = payer,
        space = 8 + KeyAccount::INIT_SPACE + key_gateway_state.max_flags as usize,
        seeds = [
            KEY_STATE_SEED,
            wid_account.wid.to_le_bytes().as_ref(),
            wid_account.key_counter.checked_add(1).ok_or(KeyRegistryError::OverflowError)?.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub key_account: Account<'info, KeyAccount>,
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub id_registry_program: Program<'info, IdRegistry>,
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
            self.id_registry_program.to_account_info(),
            IncreaseWidKeyCounter {
                wid_account: self.wid_account.to_account_info(),
                instruction_sysvar: self.instruction_sysvar.to_account_info(),
                key_gateway_state: self.key_gateway_state.to_account_info(),
            },
        ))?;
        Ok(())
    }
}
