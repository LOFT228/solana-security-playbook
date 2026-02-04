use anchor_lang::prelude::*;

#[program]
pub mod missing_signer_secure {
    use super::*;

    /// Secure: authority must sign to pause the program.
    pub fn pause(ctx: Context<PauseSecure>) -> Result<()> {
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.config.authority);
        ctx.accounts.config.paused = true;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub paused: bool,
}

#[derive(Accounts)]
pub struct PauseSecure<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}
