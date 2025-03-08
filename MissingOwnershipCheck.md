# Missing OwnerShip Check

## The Vulnerability
We also do in solidity with the help of onlyOwner modifier on EVM chains . It crucial check in the web3 Security that the expected owns an account involved in the transaction or operation.
Accounts include on `owner` field, which indicates the program with the authority to write to the account's data. This field ensures that only authorized programs can modify an account's state.
This field


## Example Scenario
Consider a program function defined to allow admin-only withdrawals from a vault . The funtion takes in a configuration account (i.e config) and uses its `admin` field to check wheter the provided admin account's public key is the same as the one stored in the config account. However, it fails to verify the `config` account OwnerShip

``` Rust
pub fn  admin_token_withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Account Setup

    if config.admin != admin.pubkey() {
        return Err(ProgramError::InvalidAdminAccount)
    }

    // Transfer Funds Logic
}
```

Malicious Actor could exploit this by supplying a `config` account that they can control with a matching `admin` field, effectively tricking the program into executing the withdrawal

## Recommendation Mitigation

```Rust 
pub fn  admin_token_withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Account Setup

    if config.admin != admin.pubkey() {
        return Err(ProgramError::InvalidAdminAccount)
    }

    if config.owner != program_id {
        return Err(ProgramError::InvalidConfigAccount)
    }

    // Transfer Funds Logic
}

```

Anchor streamlines this check with the `Account` type . `Account<'info, T>` is wrapper around `AccountInfo`, which verifies program ownership and deserialize the underying data into T(i.e the specified account type). This allow developers to use `Account<'info,T>` to validate account ownership easily. Developer  can also use the `#[account]` attribute to add the Owner trait to a given account. This trait defines an address expected to own the account.
This trait defines an address expected to own the account. This trait defines an address expected to own the account
In addition, developers can use the owner constraint to define the program that should own a given  account if its different from the currently executing one. This useful when writing an instruction that expect an account to be PDA derived from a different program. The owner constraint is defined as `#[account(owner = <expr>)]`, where <expr> is an arbitrary expression


## Read-Only Account

