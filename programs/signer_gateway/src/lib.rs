use anchor_lang::prelude::*;
use id_registry::{program::IdRegistry, WidAccount};
use signer_registry::{program::SignerRegistry, KeyAccount, KeyData, KeyRegistryGateway};

declare_id!("BWPppCHLqGTWZa8AmD9kFNxa9qXxDu9EfgWjp7sWSsD9");

#[program]
pub mod signer_gateway {
    use signer_registry::KeyData;

    use super::*;

    pub fn add(ctx: Context<Add>, key: KeyData, flags: Vec<bool>, is_admin: bool) -> Result<()> {
        let Add {
            payer,
            custody,
            wid_account,
            key_account,
            key_gateway_state,
            id_registry_program,
            key_registry_program,
            system_program,
            instruction_sysvar,
        } = ctx.accounts;
        signer_registry::cpi::add(
            CpiContext::new(
                key_registry_program.to_account_info(),
                signer_registry::cpi::accounts::Add {
                    custody: custody.to_account_info(),
                    instruction_sysvar: instruction_sysvar.to_account_info(),
                    key_account: key_account.to_account_info(),
                    key_gateway_state: key_gateway_state.to_account_info(),
                    payer: payer.to_account_info(),
                    id_registry_program: id_registry_program.to_account_info(),
                    system_program: system_program.to_account_info(),
                    wid_account: wid_account.to_account_info(),
                },
            ),
            key,
            flags,
            is_admin,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub custody: Signer<'info>,
    /// Key counter will be incremented
    #[account(mut)]
    pub wid_account: Account<'info, WidAccount>,
    /// CHECK: Will be created in Key Registry Program
    #[account(mut)]
    pub key_account: UncheckedAccount<'info>,
    pub key_gateway_state: Account<'info, KeyRegistryGateway>,
    pub id_registry_program: Program<'info, IdRegistry>,
    pub key_registry_program: Program<'info, SignerRegistry>,
    pub system_program: Program<'info, System>,
    /// CHECK: Sysvar: Used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}
