use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Invalid token pair provided")]
    InvalidTokenPair,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Insufficient liquidity in the pool")]
    InsufficientLiquidity,
    #[msg("Mathematical overflow occurred")]
    Overflow,
    #[msg("Invalid amount provided")]
    InvalidAmount,
}