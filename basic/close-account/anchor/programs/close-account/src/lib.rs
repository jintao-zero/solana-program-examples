use anchor_lang::prelude::*;

declare_id!("3a7KH9Dp5DvjwJ42hyHvnzQzcneZp34JwdEAHeKSbchW");

mod instructions;
mod state;
use instructions::*;

#[program]
pub mod close_account {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        create_user::create_user(ctx, name)
    }
    pub fn close_user(ctx: Context<CloseUser>) -> Result<()> {
        close_user::close_user(ctx)
    }
}
