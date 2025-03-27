# FrontRunning

### The Vulnerability

The front Running is the common attack in web3 security in EVM chain also . With the rising the popularity of the transaction bundlers is a concern that should be taken seriously by protocol with solana. FrontRunning as a malicious actor manipulate the expected versus actual values through carefully constructed Transaction. By gas fee manipulation in the EVm common and take advantage as insider / miner or MEV attack or common and sandwich attack also

### Example Scenario

Imagine a protocol that handles the purchasing and selling of a product, storing the sellers pricing information in an account named SellInfo:

```Rust
#[derive(Accounts)]
pub struct SellProduct<'info> {
    product_listing: Account<'info,ProductListing>,
    sale_token_mint: Account<'info,Mint>,
    sale_token_destination: Account<'info,TokenAccount>,
    product_owner: Signer<'info>,
    purchaser_token_source: Account<'info, TokenAccount>,
    product: Account<'info,Product>,
}

#[derive(Accounts)]
pub struct PurchaseProduct<'info> {
    product_listing: Account<'info,ProductListing>,
    token_destination: Account<'info,TokenAccount>,
    token_source: Account<'info,TokenAccount>,
    buyer: Signer<'info>,
    product_account: Account<'info,Product>,
    mint_token_sale: Account<'info,Mint>,
}

#[account]
pub struct ProductListing<'info> {
    sale_price: u64,
    token_mint: Pubkey,
    destination_token_account: Pubkey,
    product_owner: Pubkey,
    product: Pubkey,
}


```

To purchase a Product listed, a buyer must pass in the `ProductListing` account related to the product they want. But what is the seller can change the sale_price of their listing?

```Rust
pub fn change_sale_price(ctx: Context<ChangeSalePrice>, new_price: u64) -> Result<()> {

}
```

This would introduce the frontrunning opportunity for the seller , especially when buyer tx has nit been finialized , donot can not contain expected_price check to ensure the buyer donot pay more then expected for teh product they want.
Product is would be possible for the seller call for `change_sell_price` and using Jito ensure this tx is included the purchaser tx. A malicious seller could change the price in the `ProductListing` account to be exorbitant amount, unbekNownst to the purchaser, forcing then to pay much more then expected for the `Product!`

### Recommendation Mitigate
A simple solution would be including `expected_price` checks on the purchasing side of the deal, preventing the buyer from paying more than expected for the `Product` they want to buy:

```Rust
pub fn purchase_products(ctx: Context<PurchaseProduct> , expected_price: u64) -> Result<()> {
    assert!(ctx.accounts.product_listing.sale_price <= expected_price);
    //
    ...
}
```


# Insecure Initialization

Unlike contract deployed to EVM, Solana program not deployed with a constructor to set set state variable. Instead, they are initialized manually(normally by a function called `initialize` or something similar) 


## Insecure Example and How to Mitigate

```Rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.central_state.authority = authority.key();
    ...
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    authority: Signer<'info>,
    #[account(
        mut,
        payer = authority,
        space = CentralSpace::SIZE,
        seeds = [b"central_state"],
        bump
    )]
    central_state: Account<'info,CentralAccount>,
}


#[account]
pub struct CentralAccount{
    pub authority: Pubkey,
}
```

The example above is strippped Down initialize function that set the authroity of teh CentralState account for the instruction caller. However, this could this could any account can call the the Initialize. As prevriously , We mentioned, a common way to secure an initlization function is to use teh program `upgrade_authority` known as deployment.

We have to constraint to ensure only the program authority can call initialize.

```Rust

use anchor_lang::prelude::*;
use crate::program::MyProgram;

declare_id!("...")

#[program]
pub mod my_program {
    use super::*

    pub fn set_initial_admin(
        ctx: Context<SetInitialAdmin>,
        admin_key : Pubkey
    ) -> Result<()> {
        ctx.accounts.admin_settings.admin_key = admin_key;
        Ok(())
    }

    pub fn set_admin(){}

    pub fn set_setting(){}
    
}

#[account]
#[derive(Default, Debug)]
pub struct AdminSettings {
    admin_key: Pubkey,
}

#[derive(Accounts)]
pub struct SetInitialAdmin<'info> {
    #[account(mut, payer = authority, seeds = [b"admin"], bump)]
    pub admin_setting: Account<'info,AdminSettings>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Account<'info, MyProgram>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info,System>,
}


```

