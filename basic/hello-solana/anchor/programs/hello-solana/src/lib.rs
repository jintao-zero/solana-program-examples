use anchor_lang::prelude::*;

declare_id!("3LZsFm26YnY7WRS7gvCSSc2ZVbsuHwzakH5BdHrbVrHm");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, Solana!");
        msg!("Our program's Program ID is: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
