use anchor_lang::prelude::*;


use solana_program::{
    account_info::{next_account_info,AccountInfo},
    entryPoint,
    entryPoint::ProgramResult,
    msg,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::rent::Rent,
};

use spl_associated_token_account::get_associated_token_address;
use spl_token;

entrypoint!(process_instruction);

declare_id!("DomRGVrPW7rRUpB57gXxZgiE3qAVRMzjiMgm7XHNnxTo");


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let token_program = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let user_authority = next_account_info(accounts_iter)?;
    let vault_authority = next_account_info(accounts_iter)?;
    let vault_token_account = next_account_info(accounts_iter)?;
    let user_token_account = next_account_info(accounts_iter)?;

    let instruction_data.len() < 8  {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    if !user_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive vault authority PDA
    if token_program.key() != &spl_token_id {
        msg!("Invalid token program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let expected_vault_ata = get_associated_token_address(
        vault_authority.key(),
        mint.key(),
    );

    if expected_vault_ata != *vault_token_account.key() {
        msg!("Invalid vault token Account");
        return Err(ProgramError::InvalidAccountAddress);
    }

    msg!("Transferring {} tokens", amount);

    let transfer_ix = spl_token::instruction::transfer(
        token_program.key(),
        vault_token_account.key(),
        user_token_account.key(),
        vault_authority.key(),
        &[],
        amount,
    )?;


    let seeds = &[b"vault", user_authority.key().as_ref(), &[bump]];

    let invoke_signed(
        &transfer_ix,
        &[
            vault_token_authority.clone(),
            user_token_account.clone(),
            vault_authority.clone(),
            token_authority.clone(),
        ],
        &[seeds],
    )?;


    Ok(())


    

}
