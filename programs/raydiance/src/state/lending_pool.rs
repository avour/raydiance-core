use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LendingPool {
    pub interest_rate: u64,
    // token_b: Pubkey,
    // token_a: Pubkey,

    // This is where LP tokens will be deposited to this pool
    pub colleteral_vault: Pubkey,

    // The Mint of the lp token
    pub lp_mint: Pubkey,

    pub safety_margin: u64,
    pub liquidation_incentive: u64,
}


impl LendingPool {
    pub const SIZE: usize = 8 + // discriminator
    8 + // pub interest_rate: Pubkey,
    32 + // pub colleteral_vault: Pubkey,
    32; // pub lp_mint: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserColleteralConfig {
    pub user: Pubkey,

    /// amout of colleteral the user has deposited
    /// in the vailt
    pub amount: u64,

    pub collateral_needed: u64,
}

impl UserColleteralConfig {
    pub const SIZE: usize = 8 + // discriminator
    32 + // pub user,
    8 + // pub amount,
    8;  // pub collateral_needed,

}
