
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer}, associated_token::AssociatedToken};
use crate::{state::{LendingPool, UserCollateralConfig}, errors::RadianceError};


#[derive(Accounts)]
#[instruction(input: WithdrawCollateralInput)]
pub struct WithdrawCollateral<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        has_one = lp_mint,
        has_one = collateral_vault,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds=[b"collateral_vault".as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// This is an account to store the configuration
    /// for the user collateral in the pool
    #[account(
        mut,
        seeds = [b"user_collateral_config".as_ref(), user.key().as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
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

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct WithdrawCollateralInput {
    // a custom user id to address unique pools when testing
    /// TODO: take this out
    pool_id: u64,
    amount: u64,
}


pub fn handler(ctx: Context<WithdrawCollateral>, input: WithdrawCollateralInput) -> Result<()> {
    let user_collecteral_config = &mut ctx.accounts.user_collecteral_config;

    // TODO: check withdraw amount <= collateral balance
    require!(input.amount <= user_collecteral_config.collateral_deposited, RadianceError::InvalidTokenBalance);

    // let user_radiance_balance = ctx.accounts.user_radiance_token_account.amount;
    // require!(amount <= user_radiance_balance, RadianceError::InvalidTokenBalance);

    let pool_id = input.pool_id.to_le_bytes();
    let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
    let seeds = &[
        b"lending_pool".as_ref(),
        ctx.accounts.serum_market.key.as_ref(),
        pool_id.as_ref(),
        &[lending_pool_bump]
    ];       
    let signer_seeds = &[&seeds[..]];
    
    // change config account
    user_collecteral_config.collateral_deposited = user_collecteral_config
        .collateral_deposited.checked_sub(input.amount)
        .ok_or(RadianceError::MathError)?;


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
    anchor_spl::token::transfer(cpi_ctx, input.amount)?;
    msg!("Transfer sent");

    /// TODO: recompute collateral needed based on amount remaining

    Ok(())
}