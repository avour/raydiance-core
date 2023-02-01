pub mod errors;


use anchor_lang::prelude::*;
use anchor_spl::{
    // dex::{self, serum_dex::state::Market},
    token::{Mint, MintTo, Token, TokenAccount, Transfer, Burn}, associated_token::AssociatedToken,
};
use crate::errors::*;

declare_id!("2SnZowMwuMLoBeAeCFEWhjE12QoGd6ex8gR15TYhWyF5");

mod open_serum {
    #[cfg(not(feature = "devnet"))]
    #[cfg(not(feature = "localnet"))]
    anchor_lang::declare_id!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
    #[cfg(feature = "devnet")]
    anchor_lang::declare_id!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj");
    #[cfg(feature = "localnet")]
    anchor_lang::declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}

#[program]
pub mod raydiance {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        let lending_pool = &mut ctx.accounts.lending_pool;
        // state.escrow_wallet = ctx.accounts.escrow_wallet_state.key().clone();

        lending_pool.lp_vault = ctx.accounts.lp_vault.key().clone();
        lending_pool.lp_mint = ctx.accounts.lp_mint.key().clone();
        lending_pool.radiance_mint = ctx.accounts.radiance_mint.key().clone();
        // let mut _market =
        //     Market::load(&ctx.accounts.serum_market, &ctx.accounts.dex_program.key()).unwrap();
        // _market.

        Ok(())
    }


    pub fn deposit(ctx: Context<Deposit>, input: DepositInput) -> Result<()> {
        // let lending_pool = &mut ctx.accounts.lending_pool;

        let lending_pool_bump = *ctx.bumps.get("lending_pool").unwrap();
        let seeds = &[
            b"lending_pool".as_ref(),
            ctx.accounts.serum_market.key.as_ref(),
            ctx.accounts.lending_pool.lp_mint.as_ref(),
            &[lending_pool_bump]
       ];       
       let signer_seeds = &[&seeds[..]];

        //// Transfer LP token to vault
        msg!("Processing Transaction");
        let transfer_instruction = Transfer {
            from: ctx.accounts.user_lp_token_account.to_account_info(),
            to: ctx.accounts.lp_vault.to_account_info(),
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
        // msg!("Minting Equivalent Radiance Token to user account");
        // anchor_spl::token::mint_to(
        //     ctx.accounts.radiance_mint_to_context().with_signer(signer_seeds),
        //     input.amount,
        // )?;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64)-> Result<()> {

        // TODO: check custom token balance =< withdraw token
        let user_radiance_balance = ctx.accounts.user_radiance_token_account.amount;
        require!(amount <= user_radiance_balance, LendingPoolError::InvalidTokenBalance);

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
        let transfer_instruction = Transfer{
            from: ctx.accounts.lp_vault.to_account_info(),
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
        msg!("Minting Equivalent Radiance Token to user account");
        anchor_spl::token::burn(
            ctx.accounts.radiance_burn_to_context().with_signer(signer_seeds),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = user,
        space = LendingPool::LEN, 
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        init,
        payer = user,
        seeds=[b"lp_vault".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub lp_vault: Account<'info, TokenAccount>,

    // Mint of radiance token issued to lp stakers, when the make a deposit
    #[account(init, payer = user, mint::decimals = 9, mint::authority = lending_pool)]
    pub radiance_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub lp_mint: Account<'info, Mint>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, 
        // owner = open_serum::ID,
    )]
    pub serum_market: UncheckedAccount<'info>,

    // /// The Serum program, the is the program that owns the market
    // pub dex_program: Program<'info, dex::Dex>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct LendingPool {
    pub interest_rate: u64,
    pub collateral_needed: u64,
    // token_b: Pubkey,
    // token_a: Pubkey,

    // This is where LP tokens will be deposited to this pool
    pub lp_vault: Pubkey,

    // The Mint of the lp token
    pub lp_mint: Pubkey,

    /// 
    pub radiance_mint: Pubkey,
}

impl LendingPool {
    pub const LEN: usize = 8 + // discriminator
    8 + // pub interest_rate: Pubkey,
    8 + // pub collateral_needed: Pubkey,
    32 + // pub lp_vault: Pubkey,
    32 + // pub lp_mint: Pubkey,
    32; // pub radiance_mint: Pubkey,
}


#[derive(Accounts)]
#[instruction(input_data: DepositInput)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        has_one = lp_mint,
        has_one = lp_vault,
        has_one = radiance_mint,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds=[b"lp_vault".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub lp_vault: Account<'info, TokenAccount>,

    // Mint of radiance token issued to lp stakers, when they make a deposit
    #[account(mut)]
    pub radiance_mint: Account<'info, Mint>,

    /// user account for radiance mint token
    #[account(
        mut,
        constraint=user_radiance_token_account.owner == user.key(),
        constraint=user_radiance_token_account.mint == radiance_mint.key()
    )]
    pub user_radiance_token_account: Account<'info, TokenAccount>,

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
}


impl<'info> Deposit<'info>  {

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
pub struct DepositInput {
    amount: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        has_one = lp_mint,
        has_one = radiance_mint,
        has_one = lp_vault,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds=[b"lp_vault".as_ref(), serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    pub lp_vault: Account<'info, TokenAccount>,

    // Mint of radiance token issued to lp stakers, when they make a deposit
    #[account(mut)]
    pub radiance_mint: Account<'info, Mint>,

    /// user account for radiance mint token
    #[account(
        mut,
        constraint=user_radiance_token_account.owner == user.key(),
        constraint=user_radiance_token_account.mint == radiance_mint.key()
    )]
    pub user_radiance_token_account: Account<'info, TokenAccount>,
    
    // we need to be able to verify that this user trying to
    // withdraw has a token in our vault
    #[account(mut)]
    pub user: Signer<'info>,

    lp_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
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


impl<'info> Withdraw<'info> {

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