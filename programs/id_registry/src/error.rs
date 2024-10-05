use anchor_lang::error_code;

#[error_code]
pub enum IdRegistryError {
    #[msg("Unauthorized Admin")]
    UnauthorizedAdmin,
    #[msg("Unauthorized Owner")]
    UnauthorizedOwner,
    #[msg("Gateway is frozen")]
    GatewayFrozen,
    #[msg("Cannot Use Same Recovery Address as the Custody Adress ")]
    SameRecoveryAndCustodyAddress,
    CustomError,
    #[msg("Invalid Gateway")]
    UnauthorizedGateway,
    #[msg("Invalid Custody")]
    UnauthorizedCustody,
    #[msg("Total Number of Id exceeded")]
    OverflowError,
    #[msg("Cannot set the same recovery address")]
    CannotSetSameRecovery,
    #[msg("Cannot transfer to same custody")]
    CannotTransferToSameCustody,
    #[msg("Unauthorized recovery account")]
    UnauthorizedRecoveryAccount,
    #[msg("Cannot recover to same custody")]
    CannotRecoverToSameCustody,
}
