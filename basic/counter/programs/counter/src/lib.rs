use anchor_lang::prelude::*;

declare_id!("BsyLFuGFMqZzRiVar8yp9YNvFGdAgzr5nmniqWNZWpjg");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment_counter(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count += 1;
        msg!("Counter incremented to: {}", counter_account.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub counter: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + CounterAccount::INIT_SPACE)]
    pub counter: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CounterAccount {
    pub count: u64,
}
