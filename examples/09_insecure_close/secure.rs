use anchor_lang::prelude::*;

#[program]
pub mod insecure_close_secure {
    use super::*;

    /// Secure: only the authority can close and receive lamports.
    pub fn close_vault(ctx: Context<CloseVaultSecure>) -> Result<()> {
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.vault.authority);
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CloseVaultSecure<'info> {
    #[account(mut, close = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}
