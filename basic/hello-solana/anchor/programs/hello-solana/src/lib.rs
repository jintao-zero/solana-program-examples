use anchor_lang::prelude::*;

declare_id!("59DwW24aTvuww1LNQNeepnn39zFaE2Xno5ZCy82Ljpd8");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
