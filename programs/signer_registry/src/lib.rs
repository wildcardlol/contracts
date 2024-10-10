use anchor_lang::prelude::*;

declare_id!("GzHMQ5gBpzw5LR8pjGLQHqJWZiq7trJ5T73PYp2ESzqQ");
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
pub mod signer_registry {
    use super::*;

    /// ADMIN MANAGED
    pub fn initialize_gateway(
        ctx: Context<InitializeGateway>,
        max_keys_per_id: u16,
        default_flags: Vec<bool>,
        initial_validators: Vec<Pubkey>,
    ) -> Result<()> {
        processor::initialize_gateway::handler(
            ctx,
            max_keys_per_id,
            default_flags,
            initial_validators,
        )?;
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

    /// GATEWAY MANAGED AND CUSTODY
    pub fn add(ctx: Context<Add>, key: KeyData, flags: Vec<bool>, is_admin: bool) -> Result<()> {
        processor::add::handler(ctx, key, flags, is_admin)?;
        Ok(())
    }
}
