use anchor_lang::prelude::*;

declare_id!("CGjicJKZE9tYBxMup4nhmrzfbonb6uNcYUFx5KV8U7n5");
pub mod error;
pub use error::*;
pub mod state;
pub use state::*;
pub mod events;
pub use events::*;
pub mod processor;
pub use processor::*;
pub mod constants;
pub use constants::*;
#[program]
pub mod id_registry {
    use super::*;

    /// ADMIN MANAGED
    pub fn initialize_gateway(ctx: Context<InitializeGateway>) -> Result<()> {
        processor::initialize_gateway::handler(ctx)?;
        Ok(())
    }
    pub fn set_gateway(ctx: Context<SetGateway>) -> Result<()> {
        processor::set_gateway::handler(ctx)?;
        Ok(())
    }
    // Once frozen cannot be set again
    pub fn freeze_gateway(ctx: Context<FreezeGateway>) -> Result<()> {
        processor::freeze_gateway::handler(ctx)?;
        Ok(())
    }

    /// GATEWAY MANAGED
    pub fn register(ctx: Context<Register>) -> Result<()> {
        processor::register::handler(ctx)?;
        Ok(())
    }

    /// CALLED BY KEY REGISTRY -> KEY GATEWAY MANAGED
    pub fn increase_wid_key_counter(ctx: Context<IncreaseWidKeyCounter>) -> Result<()> {
        processor::key_counter::handler(ctx)?;
        Ok(())
    }

    /// CUSTODY MANAGED
    pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
        processor::transfer::handler(ctx)?;
        Ok(())
    }
    pub fn transfer_with_recovery(ctx: Context<TransferWithRecovery>) -> Result<()> {
        processor::transfer_with_recovery::handler(ctx)?;
        Ok(())
    }
    pub fn change_recovery(ctx: Context<ChangeRecovery>) -> Result<()> {
        processor::change_recovery::handler(ctx)?;
        Ok(())
    }

    /// RECOVERY MANAGED
    pub fn recover(ctx: Context<Recover>) -> Result<()> {
        processor::recover::handler(ctx)?;
        Ok(())
    }
}
