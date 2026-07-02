use crate::error::AmmError;
use anchor_lang::prelude::*;

pub fn calculate_swap_output(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_bps: u16,
) -> Result<u64> {
    let amount_in_with_fee = (amount_in as u128)
        .checked_mul((10000 - fee_bps) as u128)
        .ok_or(AmmError::Overflow)?;

    let denominator = (reserve_in as u128)
        .checked_mul(10000)
        .ok_or(AmmError::Overflow)?
        .checked_add(amount_in_with_fee)
        .ok_or(AmmError::Overflow)?;

    let numerator = amount_in_with_fee
        .checked_mul(reserve_out as u128)
        .ok_or(AmmError::Overflow)?;

    let amount_out = numerator
        .checked_div(denominator)
        .ok_or(AmmError::Overflow)?;

    Ok(amount_out as u64)
}

pub fn calculate_lp_tokens(
    amount_a: u64,
    amount_b: u64,
    reserve_a: u64,
    reserve_b: u64,
    lp_supply: u64,
) -> Result<u64> {
    if lp_supply == 0 {
        // Initial liquidity: sqrt(a * b)
        let amount_a_128 = amount_a as u128;
        let amount_b_128 = amount_b as u128;
        let liquidity = amount_a_128
            .checked_mul(amount_b_128)
            .ok_or(AmmError::Overflow)?;
            
        Ok(integer_sqrt(liquidity) as u64)
    } else {
        // Proportional liquidity
        let amount_a_128 = amount_a as u128;
        let reserve_a_128 = reserve_a as u128;
        let lp_supply_128 = lp_supply as u128;

        let liquidity_a = amount_a_128
            .checked_mul(lp_supply_128)
            .ok_or(AmmError::Overflow)?
            .checked_div(reserve_a_128)
            .ok_or(AmmError::Overflow)?;

        let amount_b_128 = amount_b as u128;
        let reserve_b_128 = reserve_b as u128;

        let liquidity_b = amount_b_128
            .checked_mul(lp_supply_128)
            .ok_or(AmmError::Overflow)?
            .checked_div(reserve_b_128)
            .ok_or(AmmError::Overflow)?;

        Ok(std::cmp::min(liquidity_a, liquidity_b) as u64)
    }
}

pub fn calculate_withdraw_amount(
    lp_amount: u64,
    reserve: u64,
    lp_supply: u64,
) -> Result<u64> {
    let lp_amount_128 = lp_amount as u128;
    let reserve_128 = reserve as u128;
    let lp_supply_128 = lp_supply as u128;

    let amount = lp_amount_128
        .checked_mul(reserve_128)
        .ok_or(AmmError::Overflow)?
        .checked_div(lp_supply_128)
        .ok_or(AmmError::Overflow)?;

    Ok(amount as u64)
}

fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
