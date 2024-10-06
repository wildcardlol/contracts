use anchor_lang::prelude::*;

declare_id!("95wVTaZEyDkvUZ8VryNMaMLUGnJRTWPyw2Vmk5jFdAgW");
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
pub mod admin {
    use anchor_lang::declare_id;
    declare_id!("DgSSToixmDerJbnzhZHZyepkpgN3iAtyLaofgP3jmbc6");
}

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
    pub fn freeze_gateway(ctx: Context<FreezeGateway>) -> Result<()> {
        processor::freeze_gateway::handler(ctx)?;
        Ok(())
    }

    /// GATEWAY MANAGED
    pub fn register(ctx: Context<Register>) -> Result<()> {
        processor::register::handler(ctx)?;
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
