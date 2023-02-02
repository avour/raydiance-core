
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token}, dex::{serum_dex::state::Market, Dex}};
use crate::{state::LendingPool, errors::RadianceError};
use safe_transmute::{to_bytes::transmute_to_bytes};
use std::convert::identity;

use super::supply_borrowable_base::BorrowableType;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = user,
        space = LendingPool::SIZE, 
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    /// Vault where all lp colleteral are stored
    #[account(
        init,
        payer = user,
        seeds=[b"colleteral_vault".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub colleteral_vault: Account<'info, TokenAccount>,

    /// Vault where all base mint are stored
    #[account(
        init,
        payer = user,
        seeds=[b"borrowable_base_vault".as_ref(), BorrowableType::BASE.to_string().as_bytes(), serum_market.key().as_ref()],
        bump,
        token::mint=borrowable_base_mint,
        token::authority=lending_pool,
    )]
    pub borrowable_base_vault: Account<'info, TokenAccount>,

    /// Vault where all quote mint are stored
    #[account(
        init,
        payer = user,
        seeds=[b"borrowable_quote_mint".as_ref(), BorrowableType::QUOTE.to_string().as_bytes(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=borrowable_quote_mint,
        token::authority=lending_pool,
    )]
    pub borrowable_quote_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// this is a liquidity pool token minted by the dex
    /// we need to do some check here, i don't
    pub lp_mint: Account<'info, Mint>,

    /// CHECK: in program that this mint are for the market
    /// Base mint, for a SOL/USDC pool this is SOL
    pub borrowable_base_mint: Account<'info, Mint>,
    /// Quote mint, for a SOL/USDC pool this is USDC
    pub borrowable_quote_mint: Account<'info, Mint>,

    // Mint of radiance token issued to lp stakers, when the make a deposit
    #[account(init, payer = user, mint::decimals = 9, mint::authority = lending_pool)]
    pub radiance_mint: Account<'info, Mint>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, 
        // owner = open_serum::ID,
    )]
    pub serum_market: UncheckedAccount<'info>,
    /// The Serum program, this is the program that owns the market
    pub dex_program: Program<'info, Dex>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreatePool>) -> Result<()> {
    let market =
        Market::load(&ctx.accounts.serum_market, &ctx.accounts.dex_program.key()).unwrap();
    let base_mint = Pubkey::new(&transmute_to_bytes(&identity(market.coin_mint)));
    let quote_mint = Pubkey::new(&transmute_to_bytes(&identity(market.pc_mint)));

    require_keys_eq!(base_mint, ctx.accounts.borrowable_base_mint.key(), RadianceError::InvalidPublicKey);
    require_keys_eq!(quote_mint, ctx.accounts.borrowable_quote_mint.key(), RadianceError::InvalidPublicKey);

    let lending_pool = &mut ctx.accounts.lending_pool;

    lending_pool.colleteral_vault = ctx.accounts.colleteral_vault.key().clone();
    lending_pool.lp_mint = ctx.accounts.lp_mint.key().clone();



    Ok(())
}
