use anchor_lang::{prelude::*, system_program::CreateNonceAccountWithSeedBumps};

use solana_program::{
    account_info::AccountInfo,
    entry_point::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    program::invoked_signed,
    program_error::ProgramError,
};

declare_id!("FnGsJt3T2HAykbM24JbahuJjr9WVieJgPJJxdfgn6MsS");

pub fn create_pda_account(
    payer: &AccountInfo,
    pda_account: &AccountInfo,
    system_program: &AccountInfo,
    program_id: &Pubkey,
    seeds: &[&[u8]],
    lamports: u64,
    space: u64
) -> ProgramResult {

    // Use the find_program_address to compute the PDA and canonical bump

    let (expected_pda, canonical_bump) = Pubkey::find_program_address(seeds, program_id);

    // Verify that the PDA provided matches the Expected PDA
    if expected_pda != * pda_account.key {
        msg!("Provided PDA does not match the expected PDA derived from the seeds");
        return Err(ProgramError::InvalidArgument);
    }


    let ix = system_instruction::create_account(
        payer.key,
        pda_account.key,
        lamports,
        space,
        program_id,
    );


    // Prepare the seeds vector for invoke_signed by appending the canonical bump
    let mut seeds_with_bump: Vec<&[u8]> = seeds.to_vec();
    seeds_with_bump.push(&[canonical_bump]);

    // The invoke_signed call require a slice seeds slices
    invoke_signed(
        &ix,
        &[payer.clone(),pda_account.clone(), system_program.clone()],
        // We pass our combined seeds as a  signal signer seeds array,
        &[seeds_with_bump.as_slice()],
    )
}


// Validates that a provided PDA matches the one computed from the given seeds and bump

// # Arguments
// * `provided_pda` - The PDA account passed into the instruction
// * `program_id` - The current program's ID
// `seeds` - A slice of seed byte slice used to derive the PDA
// `bump` - The bump seed that should have been used

// # Returns
// * `Ok(())` if the PDA matches the provided PDA
// * `Err(ProgramError::InvalidArgument)` if the computed PDA doest not matches

pub fn valiadate_pda_usage(
    provided_pda: &Pubkey,
    program_id: &Pubkey,
    seeds: &[&[u8]]
) -> ProgramResult {
    // Use the find_program_address to compute the expected PDA
    let (expected_pda, _canonical_bump) = Pubkey::find_program_address(seeds,program_id);


    // compare teh computed PDA  with the provided PDA
    if expected_pda == *provided_pda {
        msg!("PDA validation successful.");
        Ok(())
    }

    else{
        msg!("PDA validation failed, Expected: {}, Provided: {}",expected_pda,provided_pda);
        Err(ProgramError::InvalidArgument);
        Ok(())
    }
}
