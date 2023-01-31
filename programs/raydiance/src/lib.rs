use anchor_lang::prelude::*;
use anchor_spl::{
    dex::{self, serum_dex::state::Market},
    token::{Mint, Token, TokenAccount, Transfer}, associated_token::AssociatedToken,
};
// use anchor_spl::

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

        lending_pool.lp_vault = ctx.accounts.lp_vault_state.key().clone();
        lending_pool.lp_mint = ctx.accounts.lp_mint.key().clone();
        // let mut _market =
        //     Market::load(&ctx.accounts.serum_market, &ctx.accounts.dex_program.key()).unwrap();
        // _market.

        Ok(())
    }


    pub fn deposit(ctx: Context<Deposit>, input: DepositInput) -> Result<()> {
        // let lending_pool = &mut ctx.accounts.lending_pool;

        // TODO: mint custom token with amount

        let lp_mint_pubkey = ctx.accounts.lp_mint.key();
        let inner = vec![
            b"lending_pool".as_ref(),
            ctx.accounts.serum_market.key.as_ref(),
            lp_mint_pubkey.as_ref(),
        ];
        let outer = vec![inner.as_slice()];
        let transfer_instruction = Transfer {
            from: ctx.accounts.user_lp_token_account.to_account_info(),
            to: ctx.accounts.lp_vault_state.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, input.amount_tokens)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64)-> Result<()> {

        // TODO: check custom token balance =< withdraw token
         
        let lp_mint_pubkey = ctx.accounts.lp_mint.key();
        let inner = vec![
            b"lending_pool".as_ref(),
            ctx.accounts.serum_market.key.as_ref(),
            lp_mint_pubkey.as_ref(),
        ];
        let outer = vec![inner.as_slice()];
    
        // Perform the actual transfer
        let transfer_instruction = Transfer{
            from: ctx.accounts.user_lp_token_account.to_account_info(),
            to: ctx.accounts.lp_vault_state.to_account_info(),
            // authority: ctx.accounts.user.to_account_info(),
            // from: escrow_wallet.to_account_info(),
            // to: destination_wallet,
            authority: ctx.accounts.lending_pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );
        anchor_spl::token::transfer(cpi_ctx, amount)?;
 
        // TODO: burn token minted to user

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = user,
        space = 8, 
        seeds = [b"lending_pool", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        init,
        payer = user,
        seeds=[b"lp_vault", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    lp_vault_state: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    lp_mint: Account<'info, Mint>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, owner = open_serum::ID)]
    pub serum_market: UncheckedAccount<'info>,

    // /// The Serum program, the is the program that owns the market
    // pub dex_program: Program<'info, dex::Dex>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct LendingPool {
    interest_rate: u64,
    collateral_needed: u64,
    // token_a: Pubkey,
    // token_b: Pubkey,

    // This is where LP tokens will be deposited to this pool
    lp_vault: Pubkey,

    // The Mint of the lp token
    lp_mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(input_data: DepositInput)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(mut)]
    pub user: Signer<'info>,

    lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint=user_lp_token_account.owner == user.key(),
        constraint=user_lp_token_account.mint == lp_mint.key()
    )]
    user_lp_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"lp_vault", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    lp_vault_state: Account<'info, TokenAccount>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, owner = open_serum::ID)]
    pub serum_market: UncheckedAccount<'info>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct DepositInput {
    amount_tokens: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"lending_pool", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        has_one = lp_mint,
        bump,
    )]
    pub lending_pool: Account<'info, LendingPool>,

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
    user_lp_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"lp_vault", serum_market.key().as_ref(), lp_mint.key().as_ref()],
        bump,
        token::mint=lp_mint,
        token::authority=lending_pool,
    )]
    lp_vault_state: Account<'info, TokenAccount>,

    /// CHECK: Checks are made when loading and interacting with the market
    #[account(mut, owner = open_serum::ID)]
    pub serum_market: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}
