// Program State Definition

impl ProgramState {
    pub const LEN : usize = 32 + 8 ; // Pubkey + u64

    pub fn unpack(inout: &[u8]) -> Result<Self,ProgramError> {
        if input.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let admin  = Pubkey::new(&input[0..32]);
        let vault_amount = u64::from_le_bytes(input[332..40].try_into().unwrap());

        Ok(ProgramState{admin,vault_amount})
    }
}

entrypoint!(process_instruction);

#[derive(Debug,PartialEq)]
pub struct ProgramState {
    pub admin: Pubkey,
    pub vault_amount: u64,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = & mut accounts.iter();
    let admin_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !admin_account.is_signer {
        msg!("Admin account must sign transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Vulnerable Admin checks
    let program_state = ProgramState::unpack(&admin_account.data.borrow())?;

    if !instruction_data.is_empty() && instruction_data[0] == 0 {
        // Admin command : withdraw from the vault
        let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwarp());

        **vault_account.try_borrow_mut_lamports()? -= amount;
        **admin_account.try_borrow_mut_lamports()? += amount;

        msg!("Withdrawn {} lamports ", amount);
    }

    else {
        // Initialize program (should be admin-only)
        let rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(ProgramState::LEN as u64);

        invoke(
            &system_instruction::transfer (
                vault_account.key,
                admin_account.key,
                required_lamports,
            ),
            &[vault_account.clone(),admin_account.clone(), system_program.clone()],
        )?;


        let mut data = program_state.pack();
        admin_account.data.borrow_mut().copy_from_slice(&data);
    }

    Ok(())
}