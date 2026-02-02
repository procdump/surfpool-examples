use anchor_lang::prelude::*;

declare_id!("BMTJhCbJoav38RGnm6nBZKKba9ScFTwpP4fJ9Regfgp3");

#[program]
pub mod simple_anchor_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
