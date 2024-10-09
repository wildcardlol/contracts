use crate::IdRegistryError;
use crate::IdRegistryGateway;
use crate::RegisterEvent;
use crate::WidAccount;
use crate::WID_STATE_SEED;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Register>) -> Result<()> {
    ctx.accounts.enforce_gateway()?; // Enforces CPI / gateway
    let Register {
        registry_gateway,
        custody_account,
        recovery_account,
        wid_account,
        ..
    } = ctx.accounts;
    let wid = registry_gateway
        .id_counter
        .checked_add(1)
        .ok_or(IdRegistryError::KeyOverflowError)?;

    registry_gateway.id_counter = wid;
    wid_account.wid = wid;
    wid_account.custody = custody_account.key();
    wid_account.recovery = recovery_account.key();
    wid_account.key_counter = 0;
    emit!(RegisterEvent {
        custody: custody_account.key(),
        recovery: recovery_account.key(),
        wid,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Custody Account
    pub custody_account: AccountInfo<'info>,
    #[account(
        constraint =
        custody_account.key() != recovery_account.key() @ IdRegistryError::SameRecoveryAndCustodyAddress
    )]
    /// CHECK: Recovery Account
    pub recovery_account: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + WidAccount::INIT_SPACE,
        seeds = [WID_STATE_SEED, (registry_gateway.id_counter + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub wid_account: Account<'info, WidAccount>,
    pub system_program: Program<'info, System>,
    /// CHECK: Sysvar: used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> Register<'info> {
    fn enforce_gateway(&self) -> Result<()> {
        let ix = anchor_lang::solana_program::sysvar::instructions::get_instruction_relative(
            0,
            &self.instruction_sysvar,
        )?;
        require!(
            ix.program_id == self.registry_gateway.id_gateway_program,
            IdRegistryError::UnauthorizedGateway
        );
        Ok(())
    }
}
