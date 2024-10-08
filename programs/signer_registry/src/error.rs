use anchor_lang::error_code;

#[error_code]
pub enum KeyRegistryError {
    #[msg("Unauthorized Admin")]
    UnauthorizedAdmin,
    #[msg("Unauthorized Owner")]
    UnauthorizedOwner,
    #[msg("Gateway is frozen")]
    GatewayFrozen,
    CustomError,
    #[msg("Invalid Gateway")]
    UnauthorizedGateway,
    #[msg("Invalid Custody")]
    UnauthorizedCustody,
    #[msg("Total Number of Id exceeded")]
    OverflowError,
    #[msg("Unauthorized gateway program Id")]
    GatewayIsNotProgram,
}
