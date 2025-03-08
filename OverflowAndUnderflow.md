# Overflow And Underflow 

## The Vulnerability

An integer is a number without a fractional component. Rust stores integers as fixed-size variables . These variables are defined by their `signedness`(i.e signed or unsigned) and the amount of space they occupy in memory . For example, the `u8` type denotes an unsigned integer that occupies 8 bites of space. 

Rust includes checks for integer overflow and underflow when compiling in debug mode. These checks will cause the program to panic at runtime if such a condition is detected , Rust does not include checks that panic for integer overflow and underflows when compiling in release mode with the --release flag 


## Example Scenario
an Attacker can exploit this vulnerability by taking advantage of the silent overflow/undeeflow behaviuor in release mode, especially function that handle token balances. take the following example

```Rust
pub fn process_instruction(
    _program_id: & Pubkey,
    accounts: [&AccountInfo],
    _instruction_data: &[u8]

) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;

    let mut balance: u8 = account.data.borrow()[0]; 
    let token_to_subtract: u8 = 100;

    balance = balance - tokens_to_subtract;

    account.data.borrow_mut()[0] = balance;
    msg!("Updated balance to {}", balance);

    Ok(())

}
```

Upper function will underflow for example, a user with 10 tokens would underflow to a total balance of 165 tokens

## Recommended Mitigation

### overflow-checks

```Rust 
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: [&AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;

    let mut balance: u8 = account.data.borrow()[0];
    let tokens_to_subtract: u8 = 100;

    match balance.checked_sub(tokens_to_subtract) {
        Some(new_balance) => {
            account.data.borrow_mut()[0] = new_balance;
            msg!("Updated balance to {}", new_balance);
        },
        None => {
            return Err(ProgramError::InsufficientFunds)
        }
    }

    Ok(())
}
```

`checked_sub` is used to subtract `tokens_to_subtract` from `balance`. Thus, if `balance` is sufficient to cover the subtraction, checked_sub will return `Some(new_balance)`. The program continues to update the account's balance safely and logs it. However, if the subtraction would result in an `underflow`, `checked_sub`returns `None`, which we can handle by returning an error.


### Checked Math Macro
using library `Checked Math` macro . The issue with `checked_*` arithmetic function is the loss of mathematical notation.Instead,cumbersome methods like `a.checked_add(b).unwarp()` must be used instead of `a + b`. For example , if we want to write `(x*y) + z` using the checked arithmetic functions, we can this as `x.checked_mul(y).unwarp().checked_add(z).unwarp()`

```Rust
use checked_math::checked_math as cm;

cm!((x * y) + z).unwarp()
```

It is more preserve the expression mathematical notion, and only require on `.unwarp()`. Because the macro converts normal math expressions into an expression that return `None` if any checked steps return `None`. `Some(_)` is returned, if successful, which is why we unwarp the expression at the end.

## Casting
Casting between integer types using the `as` keyword without proper checks can introduce an integer overflow or underflow vulnerability. This is because casting can either truncate or extended values in unintended ways. When casting from a large integer type to a smaller one (e.g, u64 to u32),Rust will truncate the higher bits of the original value that do not fit into the target type.

### Recommendation Mitigation

```Rust 
pub fn convert_token_amount(amount: u64) -> Result<u32,ProgramError> {
    u32::try_from(amount).map_err(|_| ProgramError::InvalidArgument)
}
```

if `amount` exceed the maximum value a `u32` can hold (i.e., 4 294 967 295), the conversion fails, and the program returns an error. This prevent a potential overflow/underflow from occuring