// Secure Code {Fixed}

#[program]
pub mod secure_program {
    use super::*;

    pub fn secure_transfer(ctx: Context<SecureAccounts>) -> Result<()> {
        // Fix - 1 Use Anchor built CPI module
        anchor_spl::token::transfer(
            ctx.accounts.transfer_context(),
            amount,
        );

        // Fix 3 : Only allow trusted programs
        process_trusted_metadata()?; // Uses hardcoded trusted program ID

        Ok(())
    }
}


#[derive(Account)]
pub struct SecureAccounts<'info> {
    // FIX 2/4 Use Program type for automatic validation
    token_program: Program<'info, Token>,
    source: Account<'info,TokenAccount>,
    destination: Account<'info,TokenAccount>,
    authority: Signer<'info>,

    // FIX 5 : No Arbitrary program inputs allowed
    // (Metadata program removed from account)


}

// Helper showing secure CPI with explicit checks
fn process_trusted_metadata(ctx: &Context<SomeContext>) -> Result<()> {
    // FIX: 1: Explicit program ID Verification
    require!(
        ctx.accounts.metadata_program.key() == METADATA_PROGRAM_ID,
        ProgramError::IncorrectProgramId
    );

    // FIX 2: Use Anchor CPI builder
    let cpi_ctx = CpiContext::new(
        ctx.accounts.metadata_program.to_account_info(),
        MetadataInstruction::DoSomething {
            account: ctx.accounts.some_account.key(),
        },

    );

    metadata::do_something(cpi_ctx)?;
}