// INSECURE CODE : {Vulnerable to Arbitrary CPI}

#[program]
pub mod insecure_program {
    use super::*;

    pub fn insecure_transfer(ctx: Context<InsecureAccounts>, amount: u64) -> Result<()> {
        // vulnerability 1: Raw invoke without program ID checks
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),  // No verification of actual program ID 
            &ctx.accounts.source.key(),
            &ctx.accounts.destination.key(),
            &ctx.accounts.authority.key(),
            &[],
            amount,
        )?;

        // Vulnerability 2: using invoke directly without Anchor safeGuards
        invoke(
            &transfer_ix,
            &[
                ctx.accounts.source.to_account_info(),
                ctx.accounts.destination.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
        );

        // Vulnerability 3: Accepting arbitrary program account
        process_metadata(&ctx.accounts.metadata_program)?;

        Ok(())

    }
}


#[derive(Accounts)]
pub struct InsecureAccounts<'info> {
    // vUlnerability: 4 --> Using UncheckedAccount instead of program type
    token_program: UncheckedAccount<'info>,  // No program validation
    source: UncheckedAccount<'info>,
    destination: UncheckedAccount<'info>,
    authority: UncheckedAccount<'info>,

    // Vulnerability 5 : Allowing untrusted program input
    #[account(mut)]
    metadata_program: UncheckedAccount<'info>, // could be Malicious
}