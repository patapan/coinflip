use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey, 
    program_error::ProgramError, program_pack::Pack, sysvar::{Sysvar, clock::Clock}
};

use spl_token::{
    instruction::transfer, state::Account as TokenAccount, 
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
    let coin_flip_result = (clock.unix_timestamp as u64) % 2;

    if coin_flip_result == 0 {
        msg!("Heads! You've won!");
        game_data.bet_amount = 0;
        let user_token_account_data = TokenAccount::unpack(&user_account.data.borrow())?;
        let new_balance = user_token_account_data.amount + 2 * game_data.bet_amount;
        user_token_account_data.amount = new_balance;
    } else {
        msg!("Tails! You've lost!");
        game_data.bet_amount = 0;
    }
    game_data.serialize(&mut &mut game_account.data.borrow_mut()[..])?;
    Ok(())
}

