use anchor_lang::prelude::*;

declare_id!("EHKb6YaYPQFbmbopavw5KsjwFERR6YPQi9JQcBsUaJ1E");

use cpi_target::cpi::accounts::Ping;

#[program]
pub mod simple_anchor_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, cpi_target_program_id: Pubkey) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("CPI target: {:?}", cpi_target_program_id);

        require_keys_eq!(ctx.accounts.cpi_target_program.key(), cpi_target_program_id);

        let cpi_ctx = CpiContext::new(
            *ctx.accounts.cpi_target_program.key,
            Ping {
                payer: ctx.accounts.payer.to_account_info(),
            },
        );
        cpi_target::cpi::ping(cpi_ctx)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: program id is validated against the instruction arg
    pub cpi_target_program: UncheckedAccount<'info>,
}
