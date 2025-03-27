# Solana Program Security

## The Attacker Mindset in Exploiting Solana Programs

#### Solana's Programming Model

                 A Program on Solana
    
    Program Account                            Data Account
    - Stores the program's code            - Stores data
      and nothing else              <---     e.g message = "helloworld"
    - Is executable                        - Is not executable
    - Owned by the BPF Loader              - Owned by the program Account
  
`Solana's programming model` shapes the security landscape of application built on its network. On Solana, accounts act as containers for data, similar to files on a computer. We can separate accounts into two general types: exectable and non-executable . Executable accounts or program, are accounts capable of running code`[derive(Accounts)]`. Non-executable accounts are used for data storage without the ability `[account]`

- They interact with data stored in other accounts, passed by reference during transaction.


## Potential Attack Vectors

- Logic Bugs
- Data Validation Flaws
- Rust-Specific Issues
- Access Control Vulnerabilities
- Arithmetic and Precision Errors
- Cross-Program Invocation(CPI) Issues
- Program Derived Address(PDAs) Misuse


## Account Data Matching

### The Vulnerability
Account data matching is a vulnerability that arises when developers fail to check the data stored on an account matches an expected set of values. Without proper data validation checks, a program may inadvertently operate with incorrect or maliciously substituted accounts


### Example Scenario

```Rust
pub fn update_admin_settings(ctx: Context<UpdateAdminSettings>, new_settings : AdminSettings) -> Result<()> {
    ctx.accounts.config_data.settings = new_settings;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateAdminSettings<'info> {
    #[account(mut)]
    pub config_data: Account<'info, ConfigData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct ConfigData {
    admin: Pubkey,
    settings: AdminSettings
}
```

### Recommended Mitigation

```Rust
pub fn update_admin_settings(ctx: Context<UpdateAdminSettings>, new_settings: AdminSettings) -> Result<()> {
    if(ctx.accounts.admin.key() != ctx.accounts.config_data.admin){
        return Err(ProgramError::Unauthorized);
    }

    ctx.account.config_data.settings = new_settings;

    Ok(())
}

#[derive(Account)]
pub struct UpdateAdminSettings<'info> {
    #[account(
        mut,
        constraint = config_data.admin == admin.key()  // has_one and constraints
    )]
    pub config_data: Account<'info,ConfigData>,
    pub admin: Signer<'info>,
}

```

## Account Data Reallocation

### The Vulnerability

The `realloc` function provided by the `AccountInfo` struct introduces a nuanced vulnerability related to memory management. This function allows for reallocating an account's data size, useful for dynamic data handling within programs. Improper use of `realloc` can lead to unintended consequences, including wasting compute unit or potentially exposing stale data.

The `realloc` method has two parameters:

- `new_len`: a `usize` that specifies the new length of the accounts data
- `zero-init`: a bool that determine whether the memory space should be zero-initialized

```Rust

pub fn realloc(
    &self,
    new_len: usize,
    zero_init: bool
) -> Result<(),ProgramError>{

}

```
Memory Allocated for account data is already zero-initialized at the program entry point. This means the new memory space is already zeroed out when data is reallocated to a large size within a single Tx. Re-zeroing memory is unnecessary and result in additional compute unit consumption. 
Conversely realloacting to a smaller size and then back to large size one within the same tx could expose to stale data if zero_init == false.

```Rust 
pub fn modify_todo_list(ctx: Context<ModifyTodoList>, modifications: Vec<TodoModification>) -> ProgramResult {
    for modification in modifications {
        match modification {
            TodoModification::Add(entry) => {
                // Add logic
            }

            TodoModification::Remove(index) => {
                // Here we need the realloc function
            }

            TodoModification::Edit(index, new_entry) {
                // Edit Logic
            },
        }
    }

    // Reallocation logic to adjust the data size based  on Modification

    let required_data_len = calculate_required_data_len(&modifications);

    ctx.accounts.todo_list_data.realloc(required_data_len, false)?;

    Ok(())
}

#[derive(Accounts)]
pub struct ModifyTodoList<'info> {
    #[account(mut)]
    todo_list_data: AccountInfo<'info>,
    // Other relevant accounts
}
``` 


iN THIS Scenario, the modify_todo_list function might reallocate `to_do_list_data` multiple times to accomodate the size required by the modification. If the data size is reduced to remove a to-do entry and then increased again to add new entrirs within teh same tx and setting realloc `zero_init` to `false` could expose stale data.

## Recommendation Mitigation:

For thus issue setting realloc `zero_init` parameter prudently is crucial:

- Set `zero_inti` to `true` when increasing the data size after a prior decrease within the same tx call. This ensures that any new memory space is zero_initialized ,preventing stale data from being exposed.
- Set `zero_init` to `false` when incresaing the data size without a prior decrease in the same transaction call since the memory will already be zero_initialized

Instead of reallocating data to meet specific size requirement , developer use ALTs(Address LookUp Tables) . ALTs are much more powerful for the dynamic account interactions without the need for freequent memory resizing


## Account Reloading

### The Vulnerability

Account reloading is a vulnerability that arises when developers fails to update deserialized accounts after performing a CPI . Anchor does not automatically refresh the state of deserialized accounts after a CPI . This leads to Scenario where program logic operates on stale data, lead to scenorios where program logic operates on stale data, leading error or incorrect calculations.

### Example Scenario

```Rust
pub fn update_rewards(ctx: Context<UpdateStakingRewards>, amount: u64) -> Result<()> {
    let staking_seeds = &[b"stake", ctx.accounts.staker.key().as_ref(), &[ctx.accounts.staking_account.bump]];

    let cpi_accounts = UpdateRewards {
        staking_account : ctx.accounts.staking_account.to_account_info(),
    };

    let cpi_program = ctx.accounts.rewards_distribution_program.to_account_info();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, staking_seeds);

    rewards_distribution::cpi::update_rewards(cpi_ctx, amount)?;  // We for continue update the rewards we have to call this again and again that is not fiesable for the program

    // Attempt to log the "updated" reward balance
    msg!("Updated Reward balance: {}", ctx.accounts.staking_account.rewards)

    // Logic that use the stale ctx.accounts.staking_account.rewards

    Ok(())
}

#[derive(Account)]
pub struct UpdateStakingRewards<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake", staker.key().as_ref()],
        bump,
    )]

    pub staking_account: Account<'info,StakingAccount>,
    pub rewards_distribution_program: Program<'info, RewardsDistribution>,
}


#[account]
pub struct StakingAccount {
    pub amount: u64,
    pub address: Pubkey,
    pub rewards: u64,
    pub bump: u8,

}
```

In this example. the `update_rewards` function attempts to update the rewards for a user's staking account through a CPI call to a rewards distribution program. Initially , the program logs `ctx.accounts.staking_account.rewards` (i.e the rewards balance) after CPI and then continues onto logic that uses the stale `ctx.accounts.staking_account.rewards` data. The issue is that the staking accounts state is not automatically updated post-CPi which is why the data stale 

## Recommendation

To mitigate this issue, explicitly call Anchor's `reload` methods to reload a given account storage . Reloading an account post CPI will accurately reflect its state.

```diff

pub fn update_rewards(ctx: Context<UpdateStakingRewards>, amount: u64) -> Result<()> {
    let staking_seeds = &[b"stake", ctx.accounts.staker.key().as_ref(), &[ctx.accounts.staking_account.bump]];

    let cpi_accounts = UpdateRewards {
        staking_account : ctx.accounts.staking_account.to_account_info(),
    };

    let cpi_program = ctx.accounts.rewards_distribution_program.to_account_info();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, staking_seeds);

    rewards_distribution::cpi::update_rewards(cpi_ctx, amount)?;  // We for continue update the rewards we have to call this again and again that is not fiesable for the program

+   ctx.accounts.staking_account.reload()?;

    // Attempt to log the "updated" reward balance
    msg!("Updated Reward balance: {}", ctx.accounts.staking_account.rewards)

    // Logic that use the stale ctx.accounts.staking_account.rewards

    Ok(())
}

#[derive(Account)]
pub struct UpdateStakingRewards<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake", staker.key().as_ref()],
        bump,
    )]

    pub staking_account: Account<'info,StakingAccount>,
    pub rewards_distribution_program: Program<'info, RewardsDistribution>,
}


#[account]
pub struct StakingAccount {
    pub amount: u64,
    pub address: Pubkey,
    pub rewards: u64,
    pub bump: u8,

}
```

