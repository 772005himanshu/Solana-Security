
# Arbitrary CPI(Cross Program Innovation):

### The vulnerability:

Arbitrary CPIs occur when a program, invoke another program without verifying the target program's identity. This vulnerability exists because the solana runtime allows any program to call another program if the caller has the calle;s Program Id and Adreses to collee Interface . If a program performs CPIs based on user input without validating the callee's program Id,it could execute code in an attacker-controlled program

### Example Scenario

Consider a program that distribute awards to participant based on their contribution to a project . After distributing the rewards. the program records the details in seperate ledger program for auditing and tracking purposes

```Rust 
fn pub distribute_and_record_rewards(ctx: Context<DistributeAndRecord>, reward_amount: u64) -> ProgramResult {
    // Rewards Distribution logic 

    let instruction = custom_ledger_program::instruction::record_transaction(
        &ctx.accounts.ledger_program.key(),
        &ctx.accounts.rewards_account.key(),
        reward_account,
    )?;

    // Here we are making a CPI Call 
    invoke(
        &instruction,
        &[
            ctx.accounts.reward_account.clone(),
            ctx.accounts.ledger_program.clone(),
        ],

    );
}


#[derive(Accounts)]
pub struct DistributeAndRecord<'info> {
    reward_account: AccountInfo<'info>,
    ledger_program: AccountInfo<'info>,
}
```

An attacker could exploit this by passing a malicious program's ID as teh `ledger_program`, leading to unintended consequences.

### Recommendation Mitigation

To secure against this issue, developer can add a check that verifies the ledger program's identity before performing the CPI. This check would ensure that the CPI call is made to the intended program, preventing arbitrary CPIs

```diff
fn pub distribute_and_record_rewards(ctx: Context<DistributeAndRecord>, reward_amount: u64) -> ProgramResult {
    // Rewards Distribution logic 

+    if(ctx.accounts.ledger_program.key() != &custom_ledger_program::ID) {
+        return Err(ProgramError::IncorrectProgramId.into());
+    }

    let instruction = custom_ledger_program::instruction::record_transaction(
        &ctx.accounts.ledger_program.key(),
        &ctx.accounts.rewards_account.key(),
        reward_account,
    )?;

    // Here we are making a CPI Call 
    invoke(
        &instruction,
        &[
            ctx.accounts.reward_account.clone(),
            ctx.accounts.ledger_program.clone(),
        ],

    );
}


#[derive(Accounts)]
pub struct DistributeAndRecord<'info> {
    reward_account: AccountInfo<'info>,
    ledger_program: AccountInfo<'info>,
}
```


Alternatively, hardcoding the address can be a possible solution instead of having the user pass it in.



# Authority Transfer Functionality

### The Vulnerability

Solana programs often designation specific public keys as authorities for critical function, such as updating program parameters or withdrawing funds. The inability to transfer the owner ship to the another address can be risky. (In solidity `tx.origin` things may be vulnerability so we donot use it in the contract)

### Example Scenario

```Rust 
pub fn set_params(ctx: Context<SetParams>,/* parameters to be set */) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.current_admin.key(),
        ctx.accounts.global_admin.authority,
    );

    // .......
}
```
THe Authority is statically defined without the ability to update it to a new address

### recommendation Mitigation

A secure approach to mitiagating this issue is to create a two step process for transferring authority. This process would allow the current authority to nominate a new `pending_authority`, which must explicitly accept the role. Not only transfer the authority transfer functionality, but it would also protect against accidental transfer or malicious takeovers.

We have to define two functionality :

1. Nomination by the Current Authority: The current authority would nominate a new `pending_authority` by calling `nominate_new_authority` , which sets the `pending_authority` field in the program state.

2. Acceptance by New Authority: the nominated `pending_authority` calls `accept_authority` to take on their new role, transferring authority from thr current authority to `pending_authority`

```Rust
pub fn nominate_new_authority(ctx: Context<NominateAuthority>, new_authority: Pubkey ) -> Result<()> {
    let state = &mut ctx.accounts.state;

    require_keys_eq!(state.authority , ctx.accounts.current_authority.key());

    state.pending_authority = Some(new_authority);

    Ok(())
}


pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
    let state = &ctx.accounts.state;

    require_keys_eq!(Some(ctx.accounts.new_authority.key()) , state.pending_authority);

    state.authority = ctx.accounts.new_authority.key();

    state.pending_authority = None;
    Ok(())
}

#[derive(Accounts)]
pub struct NominateAuthority<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub state: Account<'info,ProgramState> // that give Authority and pending authority
    pub current_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        constraint = state.pending_authority == Some(new_authority.key())
    )]
    pub state: Account<'info,ProgramState>,
    pub new_authority: Signer<'info>,
}

#[account]
pub struct ProgramState{
    pub authority: Pubkey,
    pub pending_authority: Option<Pubkey>
}
```
