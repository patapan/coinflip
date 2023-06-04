use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction}
};
use solana_program_test::{tokio, ProgramTest};
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
        "coinflip", // Name of the program to be tested
        program_id, // program id
        None
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

    // Prepare the instruction for the program
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(game_account_pubkey, true),
            AccountMeta::new(user_account_pubkey, true),
        ],
        data: game_data.try_to_vec().unwrap(),  // serialize your instruction data
    };

    // Sign and execute the transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &game_account_keypair, &user_account_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Add your assertion here

}
