use core::fmt;

use crate::{
    errors::RadianceError,
    state::{LendingPool, UserColleteralConfig},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(input: SupplyBorrowableInput)]
pub struct SupplyBorrowable<'info> {

    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
        constraint =  input.mint_type == BorrowableType::BASE && lending_pool.borrowable_base_mint == borrowable_mint.key(),
        constraint =  input.mint_type == BorrowableType::QUOTE && lending_pool.borrowable_quote_mint == borrowable_mint.key()
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds=[b"borrowable_vault".as_ref(), input.mint_type.to_string().as_bytes(),  serum_market.key().as_ref(), borrowable_mint.key().as_ref()],
        bump,
        token::mint=borrowable_mint,
        // token::authority=lending_pool,
    )]
    pub borrowable_vault: Account<'info, TokenAccount>,


    pub borrowable_mint: Account<'info, Mint>,

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
pub struct SupplyBorrowableInput {
    amount: u64,
    mint_type: BorrowableType,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Copy, Clone, PartialEq)]
pub enum BorrowableType {
    BASE,
    QUOTE,
}

impl fmt::Display for BorrowableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn handler(ctx: Context<SupplyBorrowable>) -> Result<()> {
    Ok(())
}
