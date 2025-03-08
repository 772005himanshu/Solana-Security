use anchor_lang::prelude::*;

declare_id!("4fNG6uKsseMBMd8xuUAfhorfPZGPf5m2DULWssrkRFzh");

#[program]
pub mod signer_checks {
    use super::*;


    fn process_instruction {
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    } -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // Expect the first account to be vault PDA.
        let vault_account = next_account_info(account_info_iter)?;

        // Expect the second account to be the authority; this account must sign
        if !authority_account.is_signer {  // Here are check is.signer
            msg!("Missing required signature from authority account");
            return Err(ProgramError::MissingRequiredSignature);
        }

        // The instruction data must contain at least 32 bytes for the new admin pubkey.
        if instruction_data.len() < 32 {
            msg!("Instruction data too short : expected 32 bytes for new admin");
            return Err(ProgramError::InvalidInstructionData);

        }

        let new_admin = Pubkey::new(&instruction_data[0..32]);

        // Borrow the vault account's data mutably
        let mut vault_data = vault.account.try_borrow_mut_data()?;
        if vault_data.len() < Vault::SIZE {
            msg!("Vault account data too small");
            return Err(ProgramError::InvalidAccountData);
        }

        // Read the current admin stored in the vault (first 32 bytes)
        let current_admin = Pubkey::new(&vault_data[0..32]);

        // Enforce that the authority account provided is exactly the current admin.
        if current_admin != * authority_account.key {
            msg!(
                "Authority account ({:?}) does not match the vault's admin ({:?})",
                authority_account.key,
                current_admin
            );
            return Err(ProgramError::InvalidArgument);
        }

        // Security update the value by overwriting the stored admin teh new admin
        vault_data[0..32].copy_from_slice(new_admin.as_ref());
        msg!("Vault admin update to: {:?}", new_admin);

        Ok(())

    }
}

