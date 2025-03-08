use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token::{Token,TokenAccount,Mint};

declare_id!("EoRy5bfFPTUWFcvHQsvj3JU6yPENPN7HdErnQdEkoGw9");

#[program]
pub mod associated_token_creator {
    use super::*;
    // This instruction create an associated token account (ATA)
    // Since the ATA account is marked with the 'init' attribute , it will be created automatically
    pub fn create_ata(ctx : Context<CreateAta>) -> Result<()> {
        msg!("Associated Token Account Created Successfully");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAta<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // The ATA (associated token account) is automatically initialized using:
    //    - associated_token::mint = mint : This specifies the token mint
    //    - associated_token::authority = payer: this makes the payer the owner of the ATA
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub ata: Account<'info,TokenAccount>,
    // The mint account for the token whose ATA ae are creating
    pub mint: Account<'info,Mint>,
    // Standard programs required for account creation
    pub system_program: Program<'info,System>,
    pub token_system: Program<'info,Token>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub rent: Sysvar<'info,Rent>,
}
