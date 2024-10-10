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
    #[msg("Total Number of Key exceeded")]
    OverflowError,
    #[msg("Total limit exceeded")]
    LimitExceeded,
    #[msg("Number of flags Exceeded")]
    FlagsLengthExceeded,
    #[msg("Key cannot be greater than 256 bytes")]
    KeyValueLengthExceeded,
    #[msg("Unauthorized gateway program Id")]
    GatewayIsNotProgram,
    #[msg("Validator is not a program")]
    ValidatorKeyIsNotProgram,
    #[msg("Invalid Validator program")]
    InvalidValidatorProgram,
    #[msg("Invalid key type")]
    InvalidKeyType,
    #[msg("Parent cannot set child flags to true if they themselves have it false")]
    InvalidFlagsSetByAdmin,
}
