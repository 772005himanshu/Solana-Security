# Closing Accounts

### The Vulnerability

Improper Closing accounts in a program can lead to several vulnerabilities , including teh potential for "closed" accounts to be reinitialized or misused. The issue arises from a failure to properly mark an account as closed or failing to prevent its reuse in subsequent transaction. This oversight can allow malicious actors to exploits a given account, leading to unauthorized action or access with in program

### Example Scenario

Consider a program that allows users to create and close data storage accounts. The Program closes an account by transferring out its lamports.

```Rust 
pub fn close_account(ctx: Context<CloseAccount>) -> ProgramResult {
    let account = ctx.accounts.data_account.to_account_info();

    let destination = ctx.accounts.destination.to_account_info();

    **destination.lamports.borrow_mut() = destination.lamports().checked_add(account.lamports()).unwarp();

    **account.lamports.borrow_mut() = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info,Data>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}

#[account]
pub struct Data {
    data: u64,
}
```

This is problematic as the program fails to zero out the account's data o rmark it as closed . Merely transferering ot its remaining lamports doesnot means account is close .


## Recommendation Mitigation:
This is not only should the program transfer out all lamports, it should also zero the account's data and mark it with a discriminator (i.e "CLOSED_PROGRAM_DESCRIMINATOR"). The program should also implement checks to prevent closed accounts from being reused in future transaction.

In solidity we deal with these issue with the help of mapping.

```Rust 
use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use std::io::Cursor;
use std::ops::DerefMut;

// Other code 

pub fn close_amount(ctx: Context<CloseAccount>) -> ProgramResult {
    let account = ctx.accounts.data_account.to_account_info();

    let destination = ctx.accounts.destination.to_account_info();

    **destination.lamports.borrow_mut() = destination.lamports().checked_add(account.lamports()).unwarp();
    **account.lamports.borrow_mut() = 0;

    // Zero out the account data
    let mut data = account.try_borrow_mut_data()?;

    for byte in data.deref_mut().iter_mut() {
        *byte = 0;
    }

    // Mark the account as closed
    let dst: &mut [u8] = &mut data;
    let mut cursor = Cursor::new(dst);
    cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwarp();

    Ok(())

}

pub fn force_defund(ctx: Context<ForceDefund>) -> ProgramResult {
    let account = &ctx.accounts.account;
    let data = account.try_borrow_data()?;

    if data.len() < 8 || data[0..8] != CLOSED_ACCOUNT_DISCRIMINATOR {
        return Err(ProgramError::InvalidAccountData);
    }

    let destination = ctx.accounts.destination.to_account_info();

    **destination.lamports.borrow_mut() = destination.lamports().checked_add(account.lamports()).unwarp();
    **account.lamports.borrow_mut() = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct ForceDefund<'info> {
    #[accounts(mut)]
    pub data_account: AccountInfo<'info>,
    #[accounts(mut)]
    pub destination: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info,Data>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}

#[account]
pub struct Data {
    pub data: u64,
}
```

Zeroing out the data and adding the closed discriminator is not enough. A user can keep an account from being garbage collected by refunding the account's lamports before the end of the instruction. This will put the account in a weired state where it cannot be weired be used or garbage collected. Thus, we added a `force_defund` function to address this edge case; now anyone can defund closed accounts.

Anchor simplifies this process with the `#[account(close = destination)]` constraint, automating the secure closure of accounts by  transferring lamports, zeroing data, and setting the closed account discriminator, all in one operation.

