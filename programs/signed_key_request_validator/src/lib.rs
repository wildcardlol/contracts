use anchor_lang::prelude::*;

pub mod error;
use common::{KeyAccount, KeyRegistryGateway, SignedKeyRequestMetadata};
// use ed25519_dalek::{PublicKey, Signature, Verifier};
pub use error::*;
declare_id!("r4j2QWkASQPuh8yhsJ1bLBiM6k5shGoUWANUWG2inwS");

use ed25519_dalek::{ed25519::signature::Signature, PublicKey, Verifier};
#[program]
pub mod signed_key_request_validator {

    use super::*;

    pub fn validate(ctx: Context<Validate>, metadata: SignedKeyRequestMetadata) -> Result<()> {
        ctx.accounts.enforce_call_from_key_gateway()?;
        let Validate {
            parent_key_account, ..
        } = ctx.accounts;
        let parent_key_account =
            KeyAccount::deserialize(&mut &parent_key_account.try_borrow_data()?[8..])?;
        require!(
            parent_key_account.key.value == metadata.signer_key,
            ValidatorError::Custom
        );
        let key = metadata.signer_key;
        require!(key.len() == 32, ValidatorError::InvalidKey);
        require!(
            metadata.deadline > Clock::get()?.unix_timestamp as u64,
            ValidatorError::DeadlineExpired
        );
        let publick_key =
            PublicKey::from_bytes(&key).map_err(|_| ProgramError::InvalidInstructionData)?;
        let signature = Signature::from_bytes(&metadata.signature)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        publick_key
            .verify(&[1], &signature)
            .map_err(|_| ValidatorError::InvalidSignature)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Validate<'info> {
    pub parent_key_account: UncheckedAccount<'info>,
    /// CHECK: Manually deserialized due to circular dependencies issue
    pub key_gateway_state: UncheckedAccount<'info>,
    /// CHECK: Sysvar: used to enforce cpi
    #[account(address = anchor_lang::solana_program::sysvar::instructions::id())]
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> Validate<'info> {
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
            ValidatorError::UnauthorizedGateway
        );
        Ok(())
    }
}
