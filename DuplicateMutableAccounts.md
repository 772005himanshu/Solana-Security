## Duplicate Mutable Accounts

### The Vulnerability
Duplicate mutable accounts refers to a scenario where the same account is passed more than once as mutable parameter to an instruction. This occurs when an instruction require two mutable accounts of the same type. A malicious actor could pass in the same account twice, causing the account to be mutated in unintended ways(e.g overwriting data). The severity of this vulerability varies based on the specific scenario.

### Example scenario

Consider a program designed to rewardusers based on their participateion in a certain on-chain activity. The program has an instruction to update the balance of two accounts: a reward account and a bonus account , A user should receive a standard reward in one account and a potential bonus in another account on specific predetermined criteria:

```Rust 
pub fn distribution_rewards(ctx: Context<DistributionRewards>, reward_amount: u64, bonus_amount: u64) -> Result<()> {
    let reward_account = &mut ctx.accounts.reward_account;
    let bonus_reward = &mut ctx.accounts.bonus_account;

    // Increment the reward and bonus accounts seperately
    reward_account.balance += reward_amount;
    bonus_account.balance += bonus_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct DistributionRewards<'info> {
    #[account(mut)]
    reward_account: Account<'info,RewardAccount>,
    #[account(mut)]
    bonus_account: Account<'info,RewardAccount>,
}

#[account]
pub struct RewardAccount {
    pub balance: u64,
}

```

If a malicious actor passes teh same account for `reward_account` and `bonus_account`, the account's balance will be incorrectly updated twice.

### Recommendation Mitigation

To mitigate this issue, add a check within the instruction logic to verify that the public keys of the two accounts are not identical.

```Rust
pub fn distribution_reward(ctx: Context<DistributionRewards>, reward_amount: u64, bonus_amount: u64) -> Result<()> {
    if ctx.accounts.reward_account.key() == ctx.accounts.bonus_account.key() {
        return Err(ProgramError::InvalidArgument.into())
    }

    let reward_account = &mut ctx.accounts.reward_account;
    let bonus_reward = &mut ctx.accounts.bonus_account;

    reward_account.balance += reward_amount;
    bonus_account.balance += bonus_amount;

    Ok(())
}

```

We can use the anchor constraints to add a more explicit check on the account. This can be done using the `#[account]` attribute and the `constraint` keyword.

```Rust 

#[derive(Accounts)]
pub struct DistributionRewards<'info> {
    #[account(
        mut,
        constraint = reward_account.key() != bonus_account.key(),
    )]
    pub reward_account : Account<'info,RewardAccount>,
    #[account(mut)]
    pub bonus_account: Account<'info,RewardAccount>,
}

#[account]
pub struct RewardAccount {
    pub balance: u64,
}


```