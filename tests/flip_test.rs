use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError, 
    program_pack::Pack, 
    sysvar::{Sysvar, clock::Clock},
};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use coinflip::flip::process_instruction;
use coinflip::flip::GameData;
use spl_token::state::Account as TokenAccount;

#[tokio::test]
async fn test_flip() {
    // Create a program test environment.
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "flip", // Name of the program to be tested
        program_id, // program id
        None, // processor
    );

    // Setup
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create accounts
    let game_account_keypair = Keypair::new();
    let game_account_pubkey = game_account_keypair.pubkey();
    let user_account_keypair = Keypair::new();
    let user_account_pubkey = user_account_keypair.pubkey();

    // Fund the game account
    let game_account_balance = banks_client
        .get_balance(game_account_pubkey)
        .await
        .expect("get_balance");
    banks_client
        .transfer(
            &payer,
            &game_account_pubkey,
            game_account_balance + 1_000_000,
            recent_blockhash,
        )
        .await
        .expect("transfer failed");

    // Create game data
    let game_data = GameData {
        is_initialized: true,
        bet_amount: 100,
    };

    // Invoke the program
    let result = process_instruction(
        &program_id,
        &[
            AccountInfo::new(
                &game_account_pubkey,
                true,
                false,
                &mut game_data.try_to_vec().unwrap(),
                &mut Account::new(1_000_000, 0, &program_id),
                &program_id,
                false,
                0,
            ),
            AccountInfo::new(
                &user_account_pubkey,
                true,
                false,
                &mut TokenAccount::unpack(&[0; spl_token::state::Account::get_packed_len()]).unwrap().amount.to_le_bytes(),
                &mut Account::new(1_000_000, 0, &program_id),
                &program_id,
                false,
                0,
            ),
        ],
        &[],
    );

    assert!(result.is_ok());
}
