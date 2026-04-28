use anchor_lang::prelude::*;

declare_id!("7wMho9kEXyLj43sPnC9iM3PmafS4wpuADJhc3FJwqi7C");

#[program]
pub mod cpi_target {
    use super::*;

    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        msg!("Ping received by cpi_target!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}
