use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult
};
use solana_program_test::{tokio, ProgramTest, InvokeContext};
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use coinflip::flip::process_instruction;
use coinflip::flip::GameData;


#[tokio::test]
async fn test_flip() {
    // Create a program test environment.
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "flip", // Name of the program to be tested
        program_id, // program id
        Some(process_instruction), // processor
    );

    // Setup
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create accounts
    let game_account_keypair = Keypair::new();
    let game_account_pubkey = game_account_keypair.pubkey();
    let user_account_keypair = Keypair::new();
    let user_account_pubkey = user_account_keypair.pubkey();

    // Fund the game account with SOL
    let transfer_amount = 1_000_000;  // Amount in lamports
    let transfer_instruction = system_instruction::transfer(&payer.pubkey(), &game_account_pubkey, transfer_amount);
    let mut transaction = Transaction::new_with_payer(&[transfer_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.expect("transfer failed");

    // Create game data
    let game_data = GameData {
        is_initialized: true,
        bet_amount: 100,
    };

    // Create AccountInfo structures for program processing
    let mut game_account_lamports = 1_000_000;
    let mut user_account_lamports = 1_000_000;

    // Create user account data
    let mut user_account_data = Account::new(user_account_lamports, 0, &program_id);

    // Invoke the program
    let result = process_instruction(
        &program_id,
        &[
            AccountInfo::new(
                &game_account_pubkey,
                true,
                false,
                &mut game_account_lamports,
                &mut game_data.try_to_vec().unwrap(),
                &program_id,
                false,
                0,
            ),
            AccountInfo::new(
                &user_account_pubkey,
                true,
                false,
                &mut user_account_lamports,
                &mut user_account_data.data,
                &program_id,
                false,
                0,
            ),
        ],
        &[],
    );

    assert!(result.is_ok());
}
