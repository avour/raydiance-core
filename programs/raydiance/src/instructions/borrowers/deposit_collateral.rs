
use crate::{
    errors::RadianceError,
    state::{LendingPool, UserCollateralConfig},
};
use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token, TokenAccount, Transfer}};

#[derive(Accounts)]
#[instruction(input_data: DepositCollateralInput)]
pub struct DepositCollateral<'info> { 
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

    /// This is an account to store the configuration for the user
    /// collateral in the pool
    /// NOTE: because of init_if_needed, constraint for user is checked on handler
    #[account(
        init_if_needed,
        space = UserCollateralConfig::SIZE,
        payer = user,
        seeds = [b"user_collateral_config".as_ref(), user.key().as_ref(), serum_market.key().as_ref()],
        bump,
        // constraint = user_collecteral_config.user == user.key()
    )]
    pub user_collecteral_config: Account<'info, UserCollateralConfig>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint=user_lp_token_account.owner == user.key(),
        constraint=user_lp_token_account.mint == lp_mint.key()
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

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
pub struct DepositCollateralInput {
    amount: u64,
}

pub fn handler(ctx: Context<DepositCollateral>, input: DepositCollateralInput) -> Result<()> {
    let user_collecteral_config = &mut ctx.accounts.user_collecteral_config;

    if user_collecteral_config.user == Pubkey::default() {
        user_collecteral_config.user = ctx.accounts.user.key().clone();
    } else {
        require_keys_eq!(user_collecteral_config.user, ctx.accounts.user.key(), RadianceError::InvalidPublicKey);
    }

    //// Transfer LP token to vault
    msg!("Processing Transaction ",);

    let transfer_instruction = Transfer {
        from: ctx.accounts.user_lp_token_account.to_account_info(),
        to: ctx.accounts.collateral_vault.to_account_info(),
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
    user_collecteral_config.collateral_deposited = user_collecteral_config
        .collateral_deposited
        .checked_add(input.amount)
        .ok_or(RadianceError::MathError)?;

    // TODO: compute collateral needed based on amount deposited

    Ok(())
}
