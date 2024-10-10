use anchor_lang::prelude::*;

#[error_code]
pub enum ValidatorError {
    Custom,
    InvalidKey,
    DeadlineExpired,
    UnauthorizedGateway,
    InvalidSignature,
}
