use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

// Ensure all modules are correctly defined and imported
use crate::instructions::*;


declare_id!("J1xbFoYd8JHvUuSzU7vovZDxq4ZumzzPcMwDjT2bAqr5");

#[program]
pub mod account_data {
    use super::*;

    pub fn create_address_info(ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        crate::instructions::create_address_info(ctx, name, house_number, street, city)
    }
}
