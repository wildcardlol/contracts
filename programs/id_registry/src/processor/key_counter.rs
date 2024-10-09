use crate::IdRegistryError;
use crate::WidAccount;
use anchor_lang::prelude::*;
use common::KeyRegistryGateway;

pub fn handler(ctx: Context<IncreaseWidKeyCounter>) -> Result<()> {
    ctx.accounts.enforce_call_from_key_gateway()?;
    let IncreaseWidKeyCounter { wid_account, .. } = ctx.accounts;
    wid_account.key_counter = wid_account
        .key_counter
        .checked_add(1)
        .ok_or(IdRegistryError::OverflowError)?;
    // todo: emit event
    Ok(())
}

#[derive(Accounts)]
pub struct IncreaseWidKeyCounter<'info> {
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: Manually deserialized due to circular dependencies issue
    pub key_gateway_state: UncheckedAccount<'info>,
    /// CHECK: Sysvar: used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> IncreaseWidKeyCounter<'info> {
    fn enforce_call_from_key_gateway(&self) -> Result<()> {
        let ix = anchor_lang::solana_program::sysvar::instructions::get_instruction_relative(
            0,
            &self.instruction_sysvar,
        )?;
        let data = self.key_gateway_state.try_borrow_data()?;
        let mut account_data = &data[8..];
        let key_gateway_state = KeyRegistryGateway::deserialize(&mut account_data)
            .map_err(|_| ErrorCode::AccountDidNotDeserialize)?;
        // Enforces CPI
        require!(
            ix.program_id == key_gateway_state.key_gateway_program.key(),
            IdRegistryError::UnauthorizedGateway
        );
        Ok(())
    }
}
