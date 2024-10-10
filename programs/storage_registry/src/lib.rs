use anchor_lang::prelude::*;

declare_id!("BbMwSGQAArCGSmk25TDBcP3jSyMK2Mu1TKZQeiNXF52x");
pub mod error;
pub mod state;
pub use error::*;
pub mod constants;
pub mod processor;
#[program]
pub mod storage_registry {

    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        usd_unit_price: u64,
        sol_usd_price: u64,
        max_units: u64,
        price_feed_address: Pubkey,
    ) -> Result<()> {
        let storage_registry = &mut ctx.accounts.storage_registry;
        storage_registry.admin = ctx.accounts.admin.key();
        storage_registry.usd_unit_price = usd_unit_price;
        storage_registry.sol_usd_price = sol_usd_price;
        storage_registry.max_units = max_units;
        storage_registry.price_feed_address = price_feed_address;
        storage_registry.rented_units = 0;
        storage_registry.deprecation_timestamp = Clock::get()?.unix_timestamp + 365 * 24 * 60 * 60; // 1 year from now
        storage_registry.vault = ctx.accounts.vault.key();
        Ok(())
    }
    pub fn rent(ctx: Context<RentUnits>, wid: u64, units: u64) -> Result<()> {
        let storage_registry = &mut ctx.accounts.storage_registry;
        let clock = Clock::get()?;

        require!(
            clock.unix_timestamp < storage_registry.deprecation_timestamp,
            StorageRegistryError::ContractDeprecated
        );

        require!(units > 0, StorageRegistryError::InvalidAmount);
        require!(
            storage_registry.rented_units + units <= storage_registry.max_units,
            StorageRegistryError::ExceedsCapacity
        );
        let price_in_usd = storage_registry.usd_unit_price * units;
        let price_in_sol = storage_registry.sol_usd_price * price_in_usd;

        storage_registry.rented_units += units;

        // Transfer SOL from user to vault
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &storage_registry.vault,
            price_in_sol,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        // Emit a custom event (since we're skipping formal event definitions)
        msg!(
            "Rent: payer={}, wid={}, units={}",
            ctx.accounts.user.key(),
            wid,
            units
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + StorageRegistry::INIT_SPACE)]
    pub storage_registry: Account<'info, StorageRegistry>,
    #[account(mut, constraint = admin.key() == common::admin::ID @ StorageRegistryError::UnauthorizedAdmin)]
    pub admin: Signer<'info>,
    /// CHECK: This is the vault account, we don't need to check it
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RentUnits<'info> {
    #[account(mut)]
    pub storage_registry: Account<'info, StorageRegistry>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the vault account
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct StorageRegistry {
    pub admin: Pubkey,
    pub usd_unit_price: u64,
    pub sol_usd_price: u64,
    pub max_units: u64,
    pub price_feed_address: Pubkey,
    pub rented_units: u64,
    pub deprecation_timestamp: i64,
    pub vault: Pubkey,
}
