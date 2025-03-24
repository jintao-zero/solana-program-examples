use crate::constants::*;
use crate::state::AddressInfo;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + AddressInfo::INIT_SPACE,
    )]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    ctx.accounts.address_info.set_inner(AddressInfo {
        name,
        house_number,
        street,
        city,
    });
    Ok(())
}
