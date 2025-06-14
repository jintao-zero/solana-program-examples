use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 8, // Adjust space as needed
        seeds = [b"USER", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.name = name;
    user_account.bump = ctx.bumps.user_account;
    user_account.user = ctx.accounts.user.key();
    msg!("User account created with name: {}", user_account.name);
    Ok(())
}
