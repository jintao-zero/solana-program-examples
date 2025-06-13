#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

declare_id!("4rbTTAfnNLefm3eZXiU9yv69XD7CsvWe1jGb9ERTmppK");

#[program]
pub mod checking_accounts {
    use super::*;

    pub fn check_accounts(ctx: Context<CheckAccounts>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckAccounts<'info> {
    payer: Signer<'info>,

    /// CHECK: No checks performed, example of an unchecked account
    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,
    /// CHECK: Perform owner check using constraint
    #[account(
        mut,
        owner = id()
    )]
    account_to_change: UncheckedAccount<'info>,
    system_program: Program<'info, System>, // checks account is executable, and is the system program
}
