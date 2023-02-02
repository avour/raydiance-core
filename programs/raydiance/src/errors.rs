use anchor_lang::prelude::*;

#[error_code]
pub enum RadianceError {

    #[msg("Not enough token balance")]
    InvalidTokenBalance,

    #[msg("Math Error")]
    MathError,

    #[msg("PublicKey Do not match")]
    InvalidPublicKey,

    #[msg("Not enough liquidity in the pool")]
    IlliquidPool,
}
