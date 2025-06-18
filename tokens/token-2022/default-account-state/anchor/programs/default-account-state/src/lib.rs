use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{extension::ExtensionType, pod::PodMint, state::AccountState},
        InitializeMint2,
    },
    token_interface::{
        default_account_state_initialize, default_account_state_update,
        DefaultAccountStateInitialize, DefaultAccountStateUpdate, Mint, Token2022,
    },
};
declare_id!("31mUc2bocoT88YsQEtRkvAoXYkC9yofdddXh5PS73kqz");

#[program]
pub mod default_account_state {
    use anchor_spl::mint;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mint_size = ExtensionType::try_calculate_account_len::<PodMint>(&[
            ExtensionType::DefaultAccountState,
        ])?;
        let lamports = (Rent::get()?).minimum_balance(mint_size);
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.mint_account.to_account_info(),
            },
        );
        create_account(
            cpi_ctx,
            lamports,
            mint_size as u64,
            &ctx.accounts.token_program.key(),
        )?;

        // Initialize the NonTransferable extension
        // This instruction must come before the instruction to initialize the mint data
        default_account_state_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                DefaultAccountStateInitialize {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                },
            ),
            &AccountState::Frozen,
        )?;

        // Initialize the standard mint account data
        let mint_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint2 {
                mint: ctx.accounts.mint_account.to_account_info(),
            },
        );
        initialize_mint2(
            mint_cpi_ctx,
            2,
            ctx.accounts.payer.key,
            Some(ctx.accounts.payer.key),
        )?;

        pub fn update_default_state(
            ctx: Context<UpdateDefaultState>,
            account_state: AnchorAccountState,
        ) -> Result<()> {
            // Convert AnchorAccountState to spl_token_2022::state::AccountState
            let account_state = account_state.to_spl_account_state();

            default_account_state_update(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    DefaultAccountStateUpdate {
                        token_program_id: ctx.accounts.token_program.to_account_info(),
                        mint: ctx.accounts.mint_account.to_account_info(),
                        freeze_authority: ctx.accounts.freeze_authority.to_account_info(),
                    },
                ),
                &account_state,
            )?;
            Ok(())
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDefaultState<'info> {
    #[account(mut)]
    pub freeze_authority: Signer<'info>,
    #[account(
        mut,
        mint::freeze_authority = freeze_authority,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// Custom enum to implement AnchorSerialize and AnchorDeserialize
// This is required to pass the enum as an argument to the instruction
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AnchorAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

// Implement conversion from AnchorAccountState to spl_token_2022::state::AccountState
impl AnchorAccountState {
    pub fn to_spl_account_state(&self) -> AccountState {
        match self {
            AnchorAccountState::Uninitialized => AccountState::Uninitialized,
            AnchorAccountState::Initialized => AccountState::Initialized,
            AnchorAccountState::Frozen => AccountState::Frozen,
        }
    }
}
