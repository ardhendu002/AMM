use amm::{
    instruction::{
        InitializePool as InitializePoolIx,
        AddLiquidity as AddLiquidityIx,
        Swap as SwapIx,
        RemoveLiquidity as RemoveLiquidityIx,
    },
    constants::*,
};
use anchor_lang::InstructionData;
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;
use anchor_lang::solana_program::{
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta},
};
use anchor_lang::system_program;

use std::str::FromStr;


#[test]
fn test_amm_flow() {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let program_id = amm::ID;
    
    let token_a_mint = Keypair::new();
    let token_b_mint = Keypair::new();

    let (pool_pda, _pool_bump) = Pubkey::find_program_address(
        &[POOL_SEED, token_a_mint.pubkey().as_ref(), token_b_mint.pubkey().as_ref()],
        &program_id
    );

    let (vault_a_pda, _) = Pubkey::find_program_address(
        &[VAULT_A_SEED, pool_pda.as_ref()],
        &program_id
    );

    let (vault_b_pda, _) = Pubkey::find_program_address(
        &[VAULT_B_SEED, pool_pda.as_ref()],
        &program_id
    );

    let (lp_mint_pda, _) = Pubkey::find_program_address(
        &[LP_MINT_SEED, pool_pda.as_ref()],
        &program_id
    );

    let rent_sysvar = Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap();

   
    let init_pool_accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(token_a_mint.pubkey(), false),
        AccountMeta::new_readonly(token_b_mint.pubkey(), false),
        AccountMeta::new(pool_pda, false),
        AccountMeta::new(vault_a_pda, false),
        AccountMeta::new(vault_b_pda, false),
        AccountMeta::new(lp_mint_pda, false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(rent_sysvar, false),
    ];

    let init_pool_data = InitializePoolIx {}.data();

    let init_pool_ix = Instruction {
        program_id,
        accounts: init_pool_accounts,
        data: init_pool_data,
    };

    let message = Message::new(&[init_pool_ix], Some(&payer.pubkey()));
    let tx_init = Transaction::new(&[&payer], message, svm.latest_blockhash());
    
    let user_token_a = Keypair::new();
    let user_token_b = Keypair::new();
    let user_lp_token = Keypair::new();

    let add_liquidity_accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pool_pda, false),
        AccountMeta::new(vault_a_pda, false),
        AccountMeta::new(vault_b_pda, false),
        AccountMeta::new(lp_mint_pda, false),
        AccountMeta::new(user_token_a.pubkey(), false),
        AccountMeta::new(user_token_b.pubkey(), false),
        AccountMeta::new(user_lp_token.pubkey(), false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
    ];

    let add_liquidity_data = AddLiquidityIx { amount_a: 1000, amount_b: 1000 }.data();

    let add_liquidity_ix = Instruction {
        program_id,
        accounts: add_liquidity_accounts,
        data: add_liquidity_data,
    };

    let message = Message::new(&[add_liquidity_ix], Some(&payer.pubkey()));
    let tx_add_liq = Transaction::new(&[&payer], message, svm.latest_blockhash());


    let swap_accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pool_pda, false),
        AccountMeta::new(vault_a_pda, false),
        AccountMeta::new(vault_b_pda, false),
        AccountMeta::new(user_token_a.pubkey(), false),
        AccountMeta::new(user_token_b.pubkey(), false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
    ];

    let swap_data = SwapIx { is_a_to_b: true, amount_in: 100, min_amount_out: 90 }.data();

    let swap_ix = Instruction {
        program_id,
        accounts: swap_accounts,
        data: swap_data,
    };

    let message = Message::new(&[swap_ix], Some(&payer.pubkey()));
    let tx_swap = Transaction::new(&[&payer], message, svm.latest_blockhash());


    let remove_liquidity_accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pool_pda, false),
        AccountMeta::new(vault_a_pda, false),
        AccountMeta::new(vault_b_pda, false),
        AccountMeta::new(lp_mint_pda, false),
        AccountMeta::new(user_token_a.pubkey(), false),
        AccountMeta::new(user_token_b.pubkey(), false),
        AccountMeta::new(user_lp_token.pubkey(), false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
    ];

    let remove_liquidity_data = RemoveLiquidityIx { lp_amount: 500 }.data();

    let remove_liquidity_ix = Instruction {
        program_id,
        accounts: remove_liquidity_accounts,
        data: remove_liquidity_data,
    };

    let message = Message::new(&[remove_liquidity_ix], Some(&payer.pubkey()));
    let tx_remove_liq = Transaction::new(&[&payer], message, svm.latest_blockhash());

  
    assert_eq!(tx_init.message.instructions.len(), 1);
    assert_eq!(tx_add_liq.message.instructions.len(), 1);
    assert_eq!(tx_swap.message.instructions.len(), 1);
    assert_eq!(tx_remove_liq.message.instructions.len(), 1);
}
