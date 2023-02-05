pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("2SnZowMwuMLoBeAeCFEWhjE12QoGd6ex8gR15TYhWyF5");

mod open_serum {
    #[cfg(not(feature = "devnet"))]
    #[cfg(not(feature = "localnet"))]
    anchor_lang::declare_id!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
    #[cfg(feature = "devnet")]
    anchor_lang::declare_id!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj");
    #[cfg(feature = "localnet")]
    anchor_lang::declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}

#[program]
pub mod raydiance {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, input: CreatePoolInput) -> Result<()> {
        instructions::create_pool::handler(ctx, input)
    }

    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        input: DepositCollateralInput,
    ) -> Result<()> {
        instructions::deposit_collateral::handler(ctx, input)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, input: WithdrawCollateralInput) -> Result<()> {
        instructions::withdraw_collateral::handler(ctx, input)
    }

    pub fn borrow(ctx: Context<Borrow>, input: BorrowInput) -> Result<()> {
        instructions::borrow::handler(ctx, input)
    }

    pub fn repay_loan(ctx: Context<RepayLoan>, input: RepayLoanInput) -> Result<()> {
        instructions::repay_loan::handler(ctx, input)
    }

    pub fn supply_borrowable(ctx: Context<SupplyBorrowable>, input: SupplyBorrowableInput) -> Result<()> {
        instructions::supply_borrowable::handler(ctx, input)
    }

    pub fn withdraw_borrowable(ctx: Context<WithdrawBorrowable>, input: WithdrawBorrowableInput) -> Result<()> {
        instructions::withdraw_borrowable::handler(ctx, input)
    }

}

/// Also known as the "base" currency. For a given A/B market,
/// this is the vault for the A mint.
/// Base mint, for a SOL/USDC pool this is SOL
// pub coin_vault: AccountInfo<'info>,
/// Also known as the "quote" currency. For a given A/B market,
/// this is the vault for the B mint.
/// Quote mint, for a SOL/USDC pool this is USDC
// pub pc_vault: AccountInfo<'info>,

/// check total supply of token
///         let mint = Mint::unpack_unchecked(&mint_account.data).unwrap();
/// assert_eq!(mint.supply, 2000 - 42);
/// solana config set --url devnet

const NOTHING: u8 = 4;
