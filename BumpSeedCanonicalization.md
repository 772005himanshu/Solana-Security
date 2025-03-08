# Bump Seed Canonicalization

### The Vulnerability

Bump seed canonicalization refers to using the highest valid bump seed (i.e canonical bump) when deriving PDAs. Using the canonical bump is a deterministic and secure way to find an address given a set of seeds. Failing to use the canonical bump can lead to vulnerability, such as malicious actors creating or manipulating that compromise the program logic or data integrity

```Rust 
pub fn create_profile(ctx: Context<CreateProfile>, user_id: u64, attributes: Vec<u8> , bump: u8) -> Result<()> {
    // Explicitly derive the PDA using create_program_address and a user-provider bump
    let seeds: &[&[u8]] = &[b"profile", &user_id.to_le_bytes(), &[bump]];

    let (derived_address, _bump) = Pubkey::create_program_address(seeds, &ctx.program_id)?;

    if derived_address != ctx.accounts.profile.key(){
        return Err(ProgramError::InvalidSeeds);
    }

    let profile_pda = &mut ctx.accounts.profile;
    profile_pda.user_id = user_id;
    profile_pda.attributes = attributes;

    Ok(())
}


#[derive(Accounts)]
pub struct CreateProfile<'info>{
    #[account(mut)]
    pub user: Signer<'info>
    // The profiles account, expected to be  a PDa derived With the user_id and a user-provide bump seed
    #[account(mut)]
    pub profile: Account<'info.UserProfile>,
    pub system_program: Program<'info, System>,

} 

#[account]
pb struct UserProfile {
    pub user_id: u64,
    pub attributes: Vec<u8>,
}
```

What is the error in the Upper code ?? 
In this scenario, the program derives a `UserProfile` PDA using `create_program_address` with seeds that include a user-provided bump. Using a user-provided bump is problematic it fails to ensure te use of the canonical bump. This would allow a malicious actor to create multiple PDA with different bumps for teh same user ID.


## Recommendation Mitigation

```Rust
pub fn create_profile(ctx: Context<CreateProfile>, user_id: u64, attributes: Vec<u8>) -> Result<()> {
    let seeds : &[&[u8]] = &[b"profile", &user_id.to_le_bytes()];

    let (derived_address, bump) = Pubkey::find_program_address(seeds, &ctx.program_id)?;

    if(derived_address != ctx.accounts.profile.key()) {
        return Err(ProgramError::InvalidSeeds);
    }

    let profile_pda = &mut ctx.accounts.profile;

    profile_pda.user_id = user_id;
    profile_pda.attributes = attributes;
    profile_pda.bump = bump;

    Ok(())
}

#[derive(Accounts)]
#[instruction(user_id: u8)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 1024 + 1,
        seeds = [b"profile", user_id.to_le_bytes().as_ref()],
        bump
    )]
    pub profile: Account<'info,UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>
    pub system_program : Program<'info,System>,
}

#[account]
pub struct UserProfile{
    pub user_id: u8,
    pub attributes: Vec<u8>,
    pub bump: u8,
}
```

Here, `find_program_address` is used to derive the PDA with the canonical bump seed to ensure a deterministic and secure PDA creation. The canonical bump is stored in teh UserProfile account allowing to verify in subsequent operations. We prefer `find_program_address` over `create_program_address` because the latter create a valid PDA without searching for a bump seed.