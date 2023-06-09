use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey, 
    program_error::ProgramError, 
    sysvar::{Sysvar, clock::Clock},
    system_instruction,
};

use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameData {
    pub is_initialized: bool,
    pub bet_amount: u64,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let game_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    let game_data = GameData::try_from_slice(&instruction_data)?;

    if !game_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    let clock = Clock::get()?;
    let game_result = (clock.unix_timestamp as u64) % 2;

    // win amount
    let bet_amount_float = game_data.bet_amount as f64; // Convert to f64
    let result = bet_amount_float * 0.95; // Now you can do the multiplication
    let winnings = result.round() as u64; 

    // Create a `transfer` instruction
    let transfer_instruction = if game_result == 0 {
        system_instruction::transfer(game_account.key, user_account.key, winnings)
    } else {
        system_instruction::transfer(user_account.key, game_account.key, game_data.bet_amount)
    };
    
    
    // Perform a CPI to the System Program
    solana_program::program::invoke(
        &transfer_instruction,
        &[user_account.clone(), game_account.clone(), system_program_account.clone()],
    )?;

    if game_result == 0 {
        msg!("Heads! You've won!");
    } else {
        msg!("Tails! You've lost!");
    }

    Ok(())
}
