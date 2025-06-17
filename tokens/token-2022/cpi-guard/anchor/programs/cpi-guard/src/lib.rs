use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, Token2022, TokenAccount},
};

declare_id!("6fZE5NvzCtkZKqwfYKSNB4DEptGHkaQHDu8Xg9uD83hV");

#[program]
pub mod cpi_guard {
    use super::*;

    pub fn cpi_transfer(ctx: Context<CpiTransfer>) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.sender_token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            1,
            ctx.accounts.mint.decimals,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds=[b"pda"],
        bump,
        token::mint = mint,
        token::authority = sender,
        token::token_program = token_program,
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
