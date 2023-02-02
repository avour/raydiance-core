use crate::{
    errors::RadianceError,
    state::{LendingPool, UserCollateralConfig}, instructions::lenders::BorrowableType,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(input: RepayLoanInput)]
pub struct RepayLoan<'info> {

    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
        constraint = 
            input.mint_type == BorrowableType::BASE &&
            lending_pool.borrowable_base_mint == borrowable_mint.key(),
        constraint =
            input.mint_type == BorrowableType::QUOTE &&
            lending_pool.borrowable_quote_mint == borrowable_mint.key()
    )]
    pub lending_pool: Account<'info, LendingPool>,

    /// This is an account to store the configuration
    /// for the user collateral in the pool
    #[account(
        mut,
        seeds = [b"user_collateral_config".as_ref(), user.key().as_ref(), serum_market.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_collecteral_config: Account<'info, UserCollateralConfig>,

    /// Vault where all borrowable of type input mint_type are stored
    #[account(
        mut,
        seeds=[b"borrowable_vault".as_ref(), input.mint_type.to_string().as_bytes(),  serum_market.key().as_ref()],
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
pub struct RepayLoanInput {
    amount: u64,
    mint_type: BorrowableType,
}

pub fn handler(ctx: Context<RepayLoan>, input: RepayLoanInput) -> Result<()> {

    // check there is enough liquidity in the borrawable pool
    require!(input.amount <= ctx.accounts.borrowable_vault.amount, RadianceError::IlliquidPool);

    msg!("Transfer Initiated");
    // Perform the actual transfer
    let transfer_instruction = Transfer {
        from: ctx.accounts.user_borrowable_token_account.to_account_info(),
        to: ctx.accounts.borrowable_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );
    anchor_spl::token::transfer(cpi_ctx, input.amount)?;
    msg!("Transfer sent");

    // Document loan
    let user_collecteral_config = &mut ctx.accounts.user_collecteral_config;
    match input.mint_type {
        BorrowableType::BASE => {
            user_collecteral_config.base_borrowed_amount = user_collecteral_config
                .base_borrowed_amount
                .checked_sub(input.amount)
                .ok_or(RadianceError::MathError)?;
        },
        BorrowableType::QUOTE => {
            user_collecteral_config.base_borrowed_amount = user_collecteral_config
                .base_borrowed_amount
                .checked_sub(input.amount)
                .ok_or(RadianceError::MathError)?;
        },
    }

    Ok(())
}
