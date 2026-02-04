use anchor_lang::prelude::*;

#[program]
pub mod insecure_close_vulnerable {
    use super::*;

    /// Vulnerable: anyone can close the vault and take lamports.
    pub fn close_vault(ctx: Context<CloseVaultVulnerable>) -> Result<()> {
        // `close` sends lamports to `receiver` with no authority check.
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CloseVaultVulnerable<'info> {
    #[account(mut, close = receiver)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Anyone can receive lamports.
    pub receiver: AccountInfo<'info>,
}
