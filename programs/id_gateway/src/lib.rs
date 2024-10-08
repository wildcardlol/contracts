use anchor_lang::prelude::*;
use id_registry::{program::IdRegistry, IdRegistryGateway, WidAccount};

declare_id!("FEW1NtVZjD5u3ChatB9gCBKyvnbFibP7pn8ed81UWYqZ");

#[program]
pub mod id_gateway {
    use super::*;

    pub fn register(ctx: Context<Register>) -> Result<()> {
        let Register {
            custody_account,
            instruction_sysvar,
            payer,
            recovery_account,
            registry_gateway,
            system_program,
            registry_program,
            wid_account,
        } = ctx.accounts;
        id_registry::cpi::register(CpiContext::new(
            registry_program.to_account_info(),
            id_registry::cpi::accounts::Register {
                payer: payer.to_account_info(),
                custody_account: custody_account.to_account_info(),
                instruction_sysvar: instruction_sysvar.to_account_info(),
                recovery_account: recovery_account.to_account_info(),
                registry_gateway: registry_gateway.to_account_info(),
                system_program: system_program.to_account_info(),
                wid_account: wid_account.to_account_info(),
            },
        ))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub registry_gateway: Account<'info, IdRegistryGateway>,
    /// CHECK: Will be checked in Id registry program
    pub custody_account: AccountInfo<'info>,
    /// CHECK: Will be checked in Id registry program
    pub recovery_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: Sysvar: used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
    /// Will be created in Id registry program
    pub wid_account: Account<'info, WidAccount>,
    pub registry_program: Program<'info, IdRegistry>,
}
