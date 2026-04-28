use anchor_lang::prelude::*;

declare_id!("FLbnMiKXum2LXwLxnMdDuoqpTmzVFdsj3EEFGSE4xYKh");

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
