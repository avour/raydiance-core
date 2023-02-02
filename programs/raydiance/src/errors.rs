use anchor_lang::prelude::*;

#[error_code]
pub enum RadianceError {

    #[msg("Not enough token balance")]
    InvalidTokenBalance,

    #[msg("Math Error")]
    MathError,

    #[msg("")]
    InvalidPublicKey
}
