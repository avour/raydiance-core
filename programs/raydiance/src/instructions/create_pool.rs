
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token}, dex::serum_dex::state::Market};
use crate::state::LendingPool;
use safe_transmute::to_bytes::transmute_to_bytes;
use std::convert::identity;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = user,
        space = LendingPool::SIZE, 
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        init,
        payer = user,
        seeds=[b"colleteral_vault".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub colleteral_vault: Account<'info, TokenAccount>,

    // Mint of radiance token issued to lp stakers, when the make a deposit
    #[account(init, payer = user, mint::decimals = 9, mint::authority = lending_pool)]
    pub radiance_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub lp_mint: Account<'info, Mint>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, 
        // owner = open_serum::ID,
    )]
    pub serum_market: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreatePool>) -> Result<()> {
    let lending_pool = &mut ctx.accounts.lending_pool;
    // state.escrow_wallet = ctx.accounts.escrow_wallet_state.key().clone();

    lending_pool.colleteral_vault = ctx.accounts.colleteral_vault.key().clone();
    lending_pool.lp_mint = ctx.accounts.lp_mint.key().clone();

    let mut market =
        Market::load(&ctx.accounts.serum_market, &ctx.accounts.dex_program.key()).unwrap();
    let coin_mint = Pubkey::new(&transmute_to_bytes(&identity(market.coin_mint)));

    Ok(())
}
