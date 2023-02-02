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

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool::handler(ctx)
    }

    pub fn deposit_colleteral(
        ctx: Context<DepositColleteral>,
        input: DepositColleteralInput,
    ) -> Result<()> {
        instructions::deposit_colleteral::handler(ctx, input)
    }

    pub fn withdraw_colleteral(ctx: Context<WithdrawColleteral>, amount: u64) -> Result<()> {
        instructions::withdraw_colleteral::handler(ctx, amount)
    }

    // pub fn borrow_(ctx: Context<WithdrawColleteral>) -> Result<()> {
    //     /// 𝐵𝑂𝑅𝑅𝑂𝑊_𝐹𝐸� into account
    //     ///
    //     let mut market =
    //         anchor_spl::dex::serum_dex::state::Market::load(&ctx.accounts.serum_market, &ctx.accounts.dex_program.key()).unwrap();
    //     let coin_mint = Pubkey::new(&safe_transmute::transmute_to_bytes(&std::convert::identity(market.coin_mint)));

    //     Ok(())
    // }
    // pub fn compute_colleteral() {
    // }

    

    // supply_borrowable_base
    // supply_borrowable_mint
}

/// Also known as the "base" currency. For a given A/B market,
/// this is the vault for the A mint.
/// Base mint, for a SOL/USDC pool this is SOL
// pub coin_vault: AccountInfo<'info>,
/// Also known as the "quote" currency. For a given A/B market,
/// this is the vault for the B mint.
/// Quote mint, for a SOL/USDC pool this is USDC
// pub pc_vault: AccountInfo<'info>,
const NOTHING: u8 = 4;
