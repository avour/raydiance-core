use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LendingPool {
    /// Interest rates are automatically determined by the program
    pub base_interest_rate: u64,
    pub quote_interest_rate: u64,

    // This is where LP tokens will be deposited to this pool
    pub collateral_vault: Pubkey,

    pub safety_margin: u64,
    pub liquidation_incentive: u64,

    pub borrowable_base_mint: Pubkey,
    pub borrowable_quote_mint: Pubkey,

    pub base_radiance_mint: Pubkey,
    pub quote_radiance_mint: Pubkey,

    // The Mint of the lp token
    pub lp_mint: Pubkey,
}

impl LendingPool {
    pub const SIZE: usize = 8 + // discriminator
    8 + // pub interest_rate: Pubkey,
    32 + // pub collateral_vault: Pubkey,
    8 + // pub safety_margin,
    8 + // pub liquidation_incentive,
    32 + // pub borrowable_base_mint,
    32 + // pub borrowable_quote_mint,
    32 + // pub base_radiance_mint,
    32 + // pub quote_radiance_mint,
    32; // pub lp_mint: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserCollateralConfig {
    pub user: Pubkey,

    /// amout of collateral the user has deposited
    /// in the vailt
    pub collateral_deposited: u64,

    // amount of base_mint the user has borrowed
    pub base_borrowed_amount: u64,

    // amount of quote_mint the user has borrowed
    pub quote_borrowed_amount: u64,

    pub collateral_needed: u64,
}

impl UserCollateralConfig {
    pub const SIZE: usize = 8 + // discriminator
    32 + // pub user,
    8 + // pub collateral_deposited,
    8 + // pub base_borrowed_amount,
    8 + // pub quote_borrowed_amount,
    8; // pub collateral_needed,
}
