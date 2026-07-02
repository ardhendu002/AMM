use anchor_lang::prelude::*;
use crate::state::PoolState;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + PoolState::INIT_SPACE,
    )]
    pub pool_state: Account<'info, PoolState>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool_state;

    pool.fee_bps = 30;
    pool.bump = 0;

    Ok(())
}