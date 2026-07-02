use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};
use crate::state::PoolState;
use crate::constants::POOL_SEED;
use crate::error::AmmError;
use crate::math::constant_product::calculate_lp_tokens;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            POOL_SEED,
            pool.token_a_mint.as_ref(),
            pool.token_b_mint.as_ref()
        ],
        bump = pool.bump
    )]
    pub pool: Account<'info, PoolState>,

    #[account(
        mut,
        address = pool.vault_a,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.vault_b,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.lp_mint,
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_a.mint == pool.token_a_mint
    )]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_b.mint == pool.token_b_mint
    )]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_lp_token.mint == lp_mint.key()
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    require!(amount_a > 0 && amount_b > 0, AmmError::InvalidAmount);

    let pool = &mut ctx.accounts.pool;

    let lp_tokens_to_mint = calculate_lp_tokens(
        amount_a,
        amount_b,
        pool.reserve_a,
        pool.reserve_b,
        ctx.accounts.lp_mint.supply,
    )?;

    require!(lp_tokens_to_mint > 0, AmmError::InsufficientLiquidity);

    // Transfer token A to vault A
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.user_token_a.to_account_info(),
        to: ctx.accounts.vault_a.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program_a = ctx.accounts.token_program.key();
    let cpi_ctx_a = CpiContext::new(cpi_program_a, cpi_accounts_a);
    token::transfer(cpi_ctx_a, amount_a)?;

    // Transfer token B to vault B
    let cpi_accounts_b = Transfer {
        from: ctx.accounts.user_token_b.to_account_info(),
        to: ctx.accounts.vault_b.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program_b = ctx.accounts.token_program.key();
    let cpi_ctx_b = CpiContext::new(cpi_program_b, cpi_accounts_b);
    token::transfer(cpi_ctx_b, amount_b)?;

    // Mint LP tokens
    let seeds = &[
        POOL_SEED,
        pool.token_a_mint.as_ref(),
        pool.token_b_mint.as_ref(),
        &[pool.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_mint = MintTo {
        mint: ctx.accounts.lp_mint.to_account_info(),
        to: ctx.accounts.user_lp_token.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program_mint = ctx.accounts.token_program.key();
    let cpi_ctx_mint = CpiContext::new_with_signer(cpi_program_mint, cpi_accounts_mint, signer_seeds);
    token::mint_to(cpi_ctx_mint, lp_tokens_to_mint)?;

    // Update reserves
    pool.reserve_a = pool.reserve_a.checked_add(amount_a).ok_or(AmmError::Overflow)?;
    pool.reserve_b = pool.reserve_b.checked_add(amount_b).ok_or(AmmError::Overflow)?;

    Ok(())
}
