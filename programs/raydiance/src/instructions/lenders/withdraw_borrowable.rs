
use crate::{
    errors::RadianceError,
    state::{LendingPool, UserCollateralConfig},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, Burn};

use super::supply_borrowable::BorrowableType;

#[derive(Accounts)]
#[instruction(input: WithdrawBorrowableInput)]
pub struct WithdrawBorrowable<'info> {

    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref()],
        bump,
        constraint = 
            input.mint_type == BorrowableType::BASE &&
            lending_pool.borrowable_base_mint == borrowable_mint.key() &&
            lending_pool.base_radiance_mint == radiance_mint.key(),
        constraint =
            input.mint_type == BorrowableType::QUOTE &&
            lending_pool.borrowable_quote_mint == borrowable_mint.key() &&
            lending_pool.quote_radiance_mint == radiance_mint.key(),
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
        seeds=[b"borrowable_vault".as_ref(), input.mint_type.to_string().as_bytes(), serum_market.key().as_ref()],
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

impl<'info> WithdrawBorrowable<'info> {

    fn radiance_burn_to_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.radiance_mint.to_account_info(),
                from: self.user_radiance_token_account.to_account_info(),
                authority: self.lending_pool.to_account_info(),
            },
        )
    }

}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct WithdrawBorrowableInput {
    amount: u64,
    mint_type: BorrowableType,
}

pub fn handler(ctx: Context<WithdrawBorrowable>, input: WithdrawBorrowableInput) -> Result<()> {
    // check that withdraw amount <= user token balance 
    let user_radiance_balance = ctx.accounts.user_radiance_token_account.amount;
    require!(input.amount <= user_radiance_balance, RadianceError::InvalidTokenBalance);

    /// TODO: check there is enough liquidity in the vault, if not,
    /// has to wait till enough liquidity is available

    let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
    let seeds = &[
        b"lending_pool".as_ref(),
        ctx.accounts.serum_market.key.as_ref(),
        &[lending_pool_bump]
   ];       
   let signer_seeds = &[&seeds[..]];

    msg!("Transfer Initiated");
    // Perform the actual transfer
    let transfer_instruction = Transfer{
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

    // TODO: burn token minted to user
    msg!("Minting Equivalent Radiance Token to user account");
    anchor_spl::token::burn(
        ctx.accounts.radiance_burn_to_context().with_signer(signer_seeds),
        input.amount,
    )?;
    
    Ok(())
}
