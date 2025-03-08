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

    if !user_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive vault authority PDA
    let (expected_vault_authority, bump) = Pubkey::find_program_account(
        &[b"vault", user_authority.as_ref()],
        program_id,
    );

    // Validate vault authority account
    if expected_vault_authority != *vault_authority.key {
        msg!("Invalid Vault Authority PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    // Derive expected vault token account ATA
    let expected_vault_ata = get_associated_token_address(
        vault_authority.key,
        mint.key,
    );

    // Validate Vault Token account
    if expected_vault_ata != *vault_token_account.key {
        msg!("Invalid Vault Token Account");
        return Err(ProgramError::InvalidAccountAddress);
    }

    msg!("Transfering {} tokens", amount);

    // Create transfer instruction 
    let transfer_ix = spl_token::instruction::transfer {
        token_program.key,
        vault_token_account.key,
        user_token_account.key,
        vault_authority.key,
        &[],
        amount,
    }?;

    // Create PDA seeds
    let seeds = &[
        b"vault",
        user_authority,as_ref(),
        &[bump],
    ];

    // Sign with PDA and execute transfer
    invoke_signer(
        &transfer_ix,
        &[
            vault_token_account.clone(),
            user_token_account.clone(),
            vault_authority.clone(),
            token_program.clone(),
        ]
        &[seeds],
    )?;

    Ok(())


    

}
