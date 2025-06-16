use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

declare_id!("AR9DEXSUmkTJxXivyc8YwcCmcSBZqWVM2rLLvC1rjN7W");

#[program]
pub mod basics {

    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, _token_name: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!(
            "Creating token account for: {:?}",
            ctx.accounts.signer.key()
        );
        Ok(())
    }
    pub fn create_associated_token_account(
        ctx: Context<CreateAssociatedTokenAccount>,
    ) -> Result<()> {
        msg!(
            "Creating associated token account for: {:?}",
            ctx.accounts.signer.key()
        );
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;
        // Return Ok to indicate success
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        msg!(
            "Minting {} tokens to: {:?}",
            amount,
            ctx.accounts.receiver.key()
        );
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        mint_to(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = signer,
        seeds = [b"token-2022-token", signer.key().as_ref(),token_name.as_bytes()],
        bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = signer,
        seeds = [b"token-2022-account", signer.key().as_ref()],
        bump,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, anchor_spl::token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateAssociatedTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    pub to: SystemAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = to,
    )]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub receiver: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
