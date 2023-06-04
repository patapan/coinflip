use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, msg,
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

    // Fund the user's account
    let initial_user_balance = 1_000_000;  // Amount in lamports
    let transfer_instruction = system_instruction::transfer(&payer.pubkey(), &user_account_pubkey, initial_user_balance);
    let mut transaction = Transaction::new_with_payer(&[transfer_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.expect("transfer failed");


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
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
        ],
        data: game_data.try_to_vec().unwrap(),  // serialize your instruction data
    };

    // Sign and execute the transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &game_account_keypair, &user_account_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check that the balance of the user account increased by the bet amount
    let expected_user_balance_pos = initial_user_balance + 2 * game_data.bet_amount as u64;  // Assuming `bet_amount` is in lamports
    // Check that the balance of the user account increased by the bet amount
    let expected_user_balance_neg = initial_user_balance - 2 * game_data.bet_amount as u64;  // Assuming `bet_amount` is in lamports
    

    let user_balance = banks_client.get_balance(user_account_pubkey).await.expect("Error retrieving user balance");
    println!("user balance: {}", user_balance);
    assert!((user_balance == expected_user_balance_pos) || (user_balance == expected_user_balance_neg)) ;


}
