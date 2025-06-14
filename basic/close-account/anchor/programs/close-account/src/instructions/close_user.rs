use crate::state::UserState;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CloseUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"USER",
            user.key().as_ref(),
        ],
        bump = user_account.bump,
        close = user, // close account and return lamports to user
    )]
    pub user_account: Account<'info, UserState>,
}

pub fn close_user(_ctx: Context<CloseUser>) -> Result<()> {
    Ok(())
}
