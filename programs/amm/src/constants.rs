use anchor_lang::prelude::*;

#[constant]
pub const POOL_SEED: &[u8] = b"pool";
#[constant]
pub const VAULT_A_SEED: &[u8] = b"vault_a";
#[constant]
pub const VAULT_B_SEED: &[u8] = b"vault_b";
#[constant]
pub const LP_MINT_SEED: &[u8] = b"lp_mint";

pub const FEE_BPS: u16 = 30; // 0.3%
pub const BPS_DENOMINATOR: u16 = 10000;
