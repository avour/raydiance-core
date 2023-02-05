use anchor_lang::prelude::*;

// #[account]
// #[derive(Default)]
// pub struct BorrowablePool {

//     /// this is automatically determined by the program
//     pub interest_rate: u64,

// }


// impl BorrowablePool {
//     pub const SIZE: usize = 8 + // discriminator
//     8 + // pub interest_rate: Pubkey,
//     32 + // pub collateral_vault: Pubkey,
//     8 + // pub safety_margin,
//     8 + // pub liquidation_incentive,
//     32 + // pub borrowable_base_mint,
//     32 + // pub borrowable_quote_mint,
//     32 + // pub base_radiance_mint,
//     32 + // pub quote_radiance_mint,
//     32; // pub lp_mint: Pubkey,
// }

// #[account]
// #[derive(Default)]
// pub struct UserCollateralConfis {
//     pub user: Pubkey,

//     /// amout of collateral the user has deposited
//     /// in the vailt
//     pub amount: u64,

//     pub collateral_needed: u64,
// }


// impl UserCollateralConfis {
//     pub const SIZE: usize = 8 + // discriminator
//     32 + // pub user,
//     8 + // pub amount,
//     8;  // pub collateral_needed,

// }

