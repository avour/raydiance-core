
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token}, dex::{serum_dex::state::Market, Dex}};
use crate::{state::LendingPool, errors::RadianceError};
use safe_transmute::{to_bytes::transmute_to_bytes};
use std::convert::identity;

use super::lenders::BorrowableType;

#[derive(Accounts)]
#[instruction(input: CreatePoolInput)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = user,
        space = LendingPool::SIZE, 
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    /// Vault where all lp collateral are stored
    #[account(
        init,
        payer = user,
        seeds=[b"collateral_vault".as_ref(), serum_market.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// Vault where all base mint are stored
    #[account(
        init,
        payer = user,
        seeds=[b"borrowable_vault".as_ref(), BorrowableType::BASE.to_string().as_bytes(), serum_market.key().as_ref()],
        bump,
        token::mint=borrowable_base_mint,
        token::authority=lending_pool,
    )]
    pub borrowable_base_vault: Account<'info, TokenAccount>,

    /// Vault where all quote mint are stored
    #[account(
        init,
        payer = user,
        seeds=[b"borrowable_vault".as_ref(), BorrowableType::QUOTE.to_string().as_bytes(), serum_market.key().as_ref()],
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

    // Mint of radiance token issued to lenders, when the make a base token deposit
    #[account(init, payer = user, mint::decimals = 9, mint::authority = lending_pool)]
    pub base_radiance_mint: Account<'info, Mint>,

    // Mint of radiance token issued to lenders, when the make a quote token deposit
    #[account(init, payer = user, mint::decimals = 9, mint::authority = lending_pool)]
    pub quote_radiance_mint: Account<'info, Mint>,
    
    
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

impl<'info> CreatePool<'info> {

    fn validate(&self) -> Result<()> {
        let market =
            Market::load(&self.serum_market, &self.dex_program.key()).unwrap();
        let base_mint = Pubkey::new(&transmute_to_bytes(&identity(market.coin_mint)));
        let quote_mint = Pubkey::new(&transmute_to_bytes(&identity(market.pc_mint)));
        

        require_keys_eq!(base_mint, self.borrowable_base_mint.key(), RadianceError::InvalidPublicKey);
        require_keys_eq!(quote_mint, self.borrowable_quote_mint.key(), RadianceError::InvalidPublicKey);

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct CreatePoolInput {
    safety_margin: u64,
    liquidation_incentive: u64,
}

impl CreatePoolInput {
    fn validate(&self) -> Result<()> {
        // TODO: validate the safety margin etc..
        todo!()
    }
}

pub fn handler(ctx: Context<CreatePool>, input: CreatePoolInput ) -> Result<()> {
    // check that this mints are for the market
    input.validate()?;
    ctx.accounts.validate()?;

    let lending_pool = &mut ctx.accounts.lending_pool;

    lending_pool.borrowable_base_mint = ctx.accounts.borrowable_base_mint.key();
    lending_pool.borrowable_quote_mint = ctx.accounts.borrowable_quote_mint.key();
    lending_pool.base_radiance_mint = ctx.accounts.base_radiance_mint.key();
    lending_pool.quote_radiance_mint = ctx.accounts.quote_radiance_mint.key();

    lending_pool.safety_margin = input.safety_margin;
    lending_pool.liquidation_incentive = input.liquidation_incentive;

    lending_pool.collateral_vault = ctx.accounts.collateral_vault.key().clone();
    lending_pool.lp_mint = ctx.accounts.lp_mint.key().clone();



    Ok(())
}
