use core::fmt;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo, Transfer};

use crate::state::LendingPool;

#[derive(Accounts)]
#[instruction(input: SupplyBorrowableInput)]
pub struct SupplyBorrowable<'info> {

    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), input.pool_id.to_le_bytes().as_ref()],
        bump,
        constraint = 
            (input.mint_type == BorrowableType::Base &&
            lending_pool.borrowable_base_mint == borrowable_mint.key() &&
            lending_pool.base_radiance_mint == radiance_mint.key() ) ||
            (input.mint_type == BorrowableType::Quote &&
            lending_pool.borrowable_quote_mint == borrowable_mint.key() &&
            lending_pool.quote_radiance_mint == radiance_mint.key()),
        )]
    pub lending_pool: Box<Account<'info, LendingPool>>,

    /// program Vault where all borrowable of type input.mint_type are stored
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

    #[account(
        mut,
        constraint=user_borrowable_token_account.owner == user.key(),
        constraint=user_borrowable_token_account.mint == borrowable_mint.key()
    )]
    pub user_borrowable_token_account: Account<'info, TokenAccount>,

    // Mint of radiance token issued to lenders, when they make a deposit
    #[account(mut)]
    pub radiance_mint: Account<'info, Mint>,

    // user associated account for radiance mint
    #[account(
        mut,
        constraint=user_radiance_token_account.owner == user.key(),
        constraint=user_radiance_token_account.mint == radiance_mint.key()
    )]
    pub user_radiance_token_account: Account<'info, TokenAccount>,

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

impl<'info> SupplyBorrowable<'info> {
    fn radiance_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.radiance_mint.to_account_info(),
                to: self.user_radiance_token_account.to_account_info(),
                authority: self.lending_pool.to_account_info(),
            },
        )
    }
}


#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct SupplyBorrowableInput {
    pool_id: u64,
    amount: u64,
    mint_type: BorrowableType,
}


#[derive(AnchorDeserialize, AnchorSerialize, Debug, Copy, Clone, PartialEq)]
pub enum BorrowableType {
    Base,
    Quote,
}

pub fn handler(ctx: Context<SupplyBorrowable>, input: SupplyBorrowableInput) -> Result<()> {
    let pool_id = input.pool_id.to_le_bytes();
    let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
    let seeds = &[
        b"lending_pool".as_ref(),
        ctx.accounts.serum_market.key.as_ref(),
        pool_id.as_ref(),
        &[lending_pool_bump]
   ];       
   let signer_seeds = &[&seeds[..]];

    //// Transfer borrable token to vault
    msg!("Processing Transaction");
    let transfer_instruction = Transfer {
        from: ctx.accounts.user_borrowable_token_account.to_account_info(),
        to: ctx.accounts.borrowable_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    msg!("Sending User LP Token to vault");
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );
    anchor_spl::token::transfer(cpi_ctx, input.amount)?;
    msg!("LP Token Locked up");

    // TODO: mint custom token with amount
    msg!("Minting Equivalent Radiance Token to user account");
    anchor_spl::token::mint_to(
        ctx.accounts.radiance_mint_to_context().with_signer(signer_seeds),
        input.amount,
    )?;
    
    Ok(())
}
