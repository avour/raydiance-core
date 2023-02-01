use anchor_lang::prelude::*;

#[error_code]
pub enum LendingPoolError {

    #[msg("Not enough token balance")]
    InvalidTokenBalance

}