use crate::{
    errors::RadianceError,
    state::{LendingPool, UserCollateralConfig}, instructions::lenders::BorrowableType,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};


#[derive(Accounts)]
#[instruction(input: BorrowInput)]
pub struct Borrow<'info> {

    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        constraint = 
            (input.mint_type == BorrowableType::Base &&
            lending_pool.borrowable_base_mint == borrowable_mint.key()) || 
            (input.mint_type == BorrowableType::Quote &&
            lending_pool.borrowable_quote_mint == borrowable_mint.key()),
        )]
    pub lending_pool: Account<'info, LendingPool>,

    /// This is an account to store the configuration
    /// for the user collateral in the pool
    #[account(
        mut,
        seeds = [b"user_collateral_config".as_ref(), user.key().as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        has_one = user
    )]
    pub user_collecteral_config: Account<'info, UserCollateralConfig>,

    /// Vault where all borrowable of type input mint_type are stored
    #[account(
        mut,
        seeds=[b"borrowable_vault".as_ref(), serum_market.key().as_ref(), borrowable_mint.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        token::mint=borrowable_mint,
        token::authority=lending_pool,
    )]
    pub borrowable_vault: Account<'info, TokenAccount>,

    /// this is the base/quote mint depending on the input.mint_type
    pub borrowable_mint: Account<'info, Mint>,

    /// token account of user where we send token they borrows 
    #[account(
        mut,
        constraint=user_borrowable_token_account.owner == user.key(),
        constraint=user_borrowable_token_account.mint == borrowable_mint.key()
    )]
    pub user_borrowable_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut,
        //  owner = open_serum::ID,
        )]
    pub serum_market: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct BorrowInput {
    pool_id: u64,
    amount: u64,
    mint_type: BorrowableType,
}

pub fn handler(ctx: Context<Borrow>, input: BorrowInput) -> Result<()> {

    /// TODO: check there is enough collateral in the UserCollateralConfig for the required borrow amount 

    // check there is enough liquidity in the borrawable pool
    require!(input.amount <= ctx.accounts.borrowable_vault.amount, RadianceError::IlliquidPool);

    let pool_id = input.pool_id.to_le_bytes();
    let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
    let seeds = &[
        b"lending_pool".as_ref(),
        ctx.accounts.serum_market.key.as_ref(),
        pool_id.as_ref(),
        &[lending_pool_bump]
   ];       
   let signer_seeds = &[&seeds[..]];
   
    // Document loan
    let user_collecteral_config = &mut ctx.accounts.user_collecteral_config;
    match input.mint_type {
        BorrowableType::Base => {
            user_collecteral_config.base_borrowed_amount = user_collecteral_config
                .base_borrowed_amount
                .checked_add(input.amount)
                .ok_or(RadianceError::MathError)?;
        },
        BorrowableType::Quote => {
            user_collecteral_config.base_borrowed_amount = user_collecteral_config
                .base_borrowed_amount
                .checked_add(input.amount)
                .ok_or(RadianceError::MathError)?;
        },
    }

    msg!("Transfer Initiated");
    // Perform the actual transfer
    let transfer_instruction = Transfer {
        from: ctx.accounts.borrowable_vault.to_account_info(),
        to: ctx.accounts.user_borrowable_token_account.to_account_info(),
        authority: ctx.accounts.lending_pool.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );
    anchor_spl::token::transfer(cpi_ctx, input.amount)?;
    msg!("Transfer sent");

    Ok(())
}
