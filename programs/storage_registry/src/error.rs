use anchor_lang::prelude::*;
#[error_code]
pub enum StorageRegistryError {
    #[msg("Contract is deprecated")]
    ContractDeprecated,
    #[msg("Exceeds capacity")]
    ExceedsCapacity,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid payment")]
    InvalidPayment,
    #[msg("Invalid deprecation timestamp")]
    InvalidDeprecationTimestamp,
    #[msg("Unauthorized admin")]
    UnauthorizedAdmin,
}
