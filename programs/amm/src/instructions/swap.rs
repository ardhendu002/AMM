use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::PoolState;
use crate::constants::POOL_SEED;
use crate::error::AmmError;
use crate::math::constant_product::calculate_swap_output;

#[derive(Accounts)]
pub struct Swap<'info> {
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
        constraint = user_token_a.mint == pool.token_a_mint
    )]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_b.mint == pool.token_b_mint
    )]
    pub user_token_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Swap>, is_a_to_b: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
    require!(amount_in > 0, AmmError::InvalidAmount);

    let pool = &mut ctx.accounts.pool;

    let (reserve_in, reserve_out) = if is_a_to_b {
        (pool.reserve_a, pool.reserve_b)
    } else {
        (pool.reserve_b, pool.reserve_a)
    };

    let amount_out = calculate_swap_output(amount_in, reserve_in, reserve_out, pool.fee_bps)?;

    require!(amount_out >= min_amount_out, AmmError::SlippageExceeded);

    // Transfer input
    let cpi_accounts_in = Transfer {
        from: if is_a_to_b { ctx.accounts.user_token_a.to_account_info() } else { ctx.accounts.user_token_b.to_account_info() },
        to: if is_a_to_b { ctx.accounts.vault_a.to_account_info() } else { ctx.accounts.vault_b.to_account_info() },
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program_in = ctx.accounts.token_program.key();
    let cpi_ctx_in = CpiContext::new(cpi_program_in, cpi_accounts_in);
    token::transfer(cpi_ctx_in, amount_in)?;

    // Transfer output
    let seeds = &[
        POOL_SEED,
        pool.token_a_mint.as_ref(),
        pool.token_b_mint.as_ref(),
        &[pool.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_out = Transfer {
        from: if is_a_to_b { ctx.accounts.vault_b.to_account_info() } else { ctx.accounts.vault_a.to_account_info() },
        to: if is_a_to_b { ctx.accounts.user_token_b.to_account_info() } else { ctx.accounts.user_token_a.to_account_info() },
        authority: pool.to_account_info(),
    };
    let cpi_program_out = ctx.accounts.token_program.key();
    let cpi_ctx_out = CpiContext::new_with_signer(cpi_program_out, cpi_accounts_out, signer_seeds);
    token::transfer(cpi_ctx_out, amount_out)?;

    // Update reserves
    if is_a_to_b {
        pool.reserve_a = pool.reserve_a.checked_add(amount_in).ok_or(AmmError::Overflow)?;
        pool.reserve_b = pool.reserve_b.checked_sub(amount_out).ok_or(AmmError::Overflow)?;
    } else {
        pool.reserve_b = pool.reserve_b.checked_add(amount_in).ok_or(AmmError::Overflow)?;
        pool.reserve_a = pool.reserve_a.checked_sub(amount_out).ok_or(AmmError::Overflow)?;
    }

    Ok(())
}
