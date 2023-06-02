use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey, 
    program_error::ProgramError, 
    program_pack::Pack, 
    sysvar::{Sysvar, clock::Clock},
};

use solana_sdk::account::Account;

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
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let game_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;

    let mut game_data = GameData::try_from_slice(&game_account.data.borrow())?;

    if !game_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    let clock = Clock::get()?;
    let coin_process_instruction_result = (clock.unix_timestamp as u64) % 2;

    if coin_process_instruction_result == 0 {
        msg!("Heads! You've won!");
        game_data.bet_amount = 0;
        let new_balance = **user_account.lamports.borrow() + 2 * game_data.bet_amount;
        **user_account.lamports.borrow_mut() = new_balance;
    } else {
        msg!("Tails! You've lost!");
        game_data.bet_amount = 0;
    }
    game_data.serialize(&mut &mut game_account.data.borrow_mut()[..])?;
    Ok(())
}
