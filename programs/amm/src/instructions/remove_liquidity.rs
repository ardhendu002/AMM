use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, Burn};
use crate::state::PoolState;
use crate::constants::POOL_SEED;
use crate::error::AmmError;
use crate::math::constant_product::calculate_withdraw_amount;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
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

pub fn handler(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
    require!(lp_amount > 0, AmmError::InvalidAmount);

    let pool = &mut ctx.accounts.pool;

    let amount_a = calculate_withdraw_amount(
        lp_amount,
        pool.reserve_a,
        ctx.accounts.lp_mint.supply,
    )?;

    let amount_b = calculate_withdraw_amount(
        lp_amount,
        pool.reserve_b,
        ctx.accounts.lp_mint.supply,
    )?;

    require!(amount_a > 0 && amount_b > 0, AmmError::InsufficientLiquidity);

    // Burn LP tokens
    let cpi_accounts_burn = Burn {
        mint: ctx.accounts.lp_mint.to_account_info(),
        from: ctx.accounts.user_lp_token.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program_burn = ctx.accounts.token_program.key();
    let cpi_ctx_burn = CpiContext::new(cpi_program_burn, cpi_accounts_burn);
    token::burn(cpi_ctx_burn, lp_amount)?;

    // Withdraw tokens
    let seeds = &[
        POOL_SEED,
        pool.token_a_mint.as_ref(),
        pool.token_b_mint.as_ref(),
        &[pool.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_out_a = Transfer {
        from: ctx.accounts.vault_a.to_account_info(),
        to: ctx.accounts.user_token_a.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program_out_a = ctx.accounts.token_program.key();
    let cpi_ctx_out_a = CpiContext::new_with_signer(cpi_program_out_a, cpi_accounts_out_a, signer_seeds);
    token::transfer(cpi_ctx_out_a, amount_a)?;

    let cpi_accounts_out_b = Transfer {
        from: ctx.accounts.vault_b.to_account_info(),
        to: ctx.accounts.user_token_b.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program_out_b = ctx.accounts.token_program.key();
    let cpi_ctx_out_b = CpiContext::new_with_signer(cpi_program_out_b, cpi_accounts_out_b, signer_seeds);
    token::transfer(cpi_ctx_out_b, amount_b)?;

    // Update reserves
    pool.reserve_a = pool.reserve_a.checked_sub(amount_a).ok_or(AmmError::Overflow)?;
    pool.reserve_b = pool.reserve_b.checked_sub(amount_b).ok_or(AmmError::Overflow)?;

    Ok(())
}
