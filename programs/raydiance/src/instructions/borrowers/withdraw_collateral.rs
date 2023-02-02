
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer}, associated_token::AssociatedToken};
use crate::{state::{LendingPool, UserCollateralConfig}, errors::RadianceError};

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
        has_one = lp_mint,
        has_one = collateral_vault,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds=[b"collateral_vault".as_ref(), serum_market.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// This is an account to store the configuration
    /// for the user collateral in the pool
    #[account(
        mut,
        seeds = [b"user_collateral_config".as_ref(), user.key().as_ref(), serum_market.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_collecteral_config: Account<'info, UserCollateralConfig>,

    // we need to be able to verify that this user trying to
    // withdraw has a token in our vault
    #[account(mut)]
    pub user: Signer<'info>,

    lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = user,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, 
        // owner = open_serum::ID
    )]
    pub serum_market: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn handler(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
    let user_collecteral_config = &mut ctx.accounts.user_collecteral_config;

    // TODO: check withdraw amount <= collateral balance
    require!(amount <= user_collecteral_config.collateral_deposited, RadianceError::InvalidTokenBalance);

    // let user_radiance_balance = ctx.accounts.user_radiance_token_account.amount;
    // require!(amount <= user_radiance_balance, RadianceError::InvalidTokenBalance);

    let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
    let seeds = &[
        b"lending_pool".as_ref(),
        ctx.accounts.serum_market.key.as_ref(),
        ctx.accounts.lending_pool.lp_mint.as_ref(),
        &[lending_pool_bump]
    ];       
    let signer_seeds = &[&seeds[..]];
    

    msg!("Transfer Initiated");
    // Perform the actual transfer
    let transfer_instruction = Transfer {
        from: ctx.accounts.collateral_vault.to_account_info(),
        to: ctx.accounts.user_lp_token_account.to_account_info(),
        authority: ctx.accounts.lending_pool.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );
    anchor_spl::token::transfer(cpi_ctx, amount)?;
    msg!("Transfer sent");

    // TODO: burn token minted to user
    user_collecteral_config.collateral_deposited = user_collecteral_config
        .collateral_deposited.checked_sub(amount)
        .ok_or(RadianceError::MathError)?;

    // TODO: recompute collateral needed based on amount remaining

    Ok(())
}