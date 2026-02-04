use anchor_lang::prelude::*;

#[program]
pub mod missing_signer_vulnerable {
    use super::*;

    /// Vulnerable: authority is compared, but not required to sign.
    pub fn pause(ctx: Context<PauseVulnerable>) -> Result<()> {
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
pub struct PauseVulnerable<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: Not a signer.
    pub authority: AccountInfo<'info>,
}
