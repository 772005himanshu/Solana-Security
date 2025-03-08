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
    space: u64,
    bump: u8,
) -> ProgramResult {
    // Create the `create account` instruction.
    let ix = system_instruction::create_account(
        payer.key,
        pda_account.key,
        lamports,
        space,
        program_id,
    );

    // Combine teh Provided seeds with the bumps seed
    // First, create a slice holding the bump as a one_element byte slice

    let bump_slice = &[bump]; // it derived from teh upper bump that is not safe
    // Then, build a vector of all seed slices (copies of teh provided seeds plus the bump)
    let mut seeds_with_bump: Vec<&[u8]> = seeds.to_vec();
    seeds_with_bump.push(bump_slice);

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
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    // Build the seeds array by appending the bump seed.

    let bump_slice = &[bump];
    let mut seeds_with_bump: Vec<&[u8]> = seeds.to_vec();

    seeds_with_bump.push(bump_slice);

    // Compute the PDA using the given seeds and program_id.
    let computed_pda = Pubkey::create_program_address(seeds_with_bump.as_slice(), program_id)?;  // @audit here is the issue

    // compare teh computed PDA  with the provided PDA
    if computed_pda == *provided_pda {
        msg!("PDA validation successful.");
        Ok(())
    }

    else{
        msg!("PDA validation failed, Expected: {}, Provided: {}", computed_pda,provided_pda);
        Err(ProgramError::InvalidArgument);
        Ok(())
    }
}
