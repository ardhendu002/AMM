pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod math;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("BHXF68PTvS1KXaix4HywUqP9DJWMjaNkSLZt1KSuTswZ");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        instructions::initialize_pool::handler(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        instructions::add_liquidity::handler(ctx, amount_a, amount_b)
    }

    pub fn swap(ctx: Context<Swap>, is_a_to_b: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        instructions::swap::handler(ctx, is_a_to_b, amount_in, min_amount_out)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
        instructions::remove_liquidity::handler(ctx, lp_amount)
    }
}
