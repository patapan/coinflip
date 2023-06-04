use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey, 
    program_error::ProgramError, 
    sysvar::{Sysvar, clock::Clock},
    system_instruction::transfer,
};

use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameData {
    pub is_initialized: bool,
    pub bet_amount: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let game_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    let mut game_data = GameData::try_from_slice(&instruction_data)?;

    if !game_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    let clock = Clock::get()?;
    let coin_process_instruction_result = (clock.unix_timestamp as u64) % 2;

    if coin_process_instruction_result == 0 {
        msg!("Heads! You've won!");

        // Calculate winnings
        let winnings = 2 * game_data.bet_amount;

        // Create a `transfer` instruction
        let transfer_instruction = transfer(
            game_account.key,
            user_account.key,
            winnings,
        );

        // Perform a CPI to the System Program
        solana_program::program::invoke(
            &transfer_instruction,
            &[game_account.clone(), user_account.clone(), system_program_account.clone()],
        )?;

    } else {
        msg!("Tails! You've lost!");

        // Create a `transfer` instruction
        let transfer_instruction = transfer(
            user_account.key,
            game_account.key,
            game_data.bet_amount,
        );

        // Perform a CPI to the System Program
        solana_program::program::invoke(
            &transfer_instruction,
            &[user_account.clone(), game_account.clone(), system_program_account.clone()],
        )?;
    }

    Ok(())
}
