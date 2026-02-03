use anchor_lang::prelude::*;

#[program]
pub mod authority_check {
    use super::*;

    /// Vulnerable: accepts any `authority` account without requiring a signature.
    /// An attacker can pass the real authority's public key and bypass checks.
    pub fn set_fee_vulnerable(ctx: Context<SetFeeVulnerable>, new_fee_bps: u16) -> Result<()> {
        // This equality check is meaningless if `authority` is not a signer.
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.config.authority);
        ctx.accounts.config.fee_bps = new_fee_bps;
        Ok(())
    }

    /// Secure: require the authority to be a signer and match config.
    pub fn set_fee_secure(ctx: Context<SetFeeSecure>, new_fee_bps: u16) -> Result<()> {
        // Anchor enforces `authority` is a signer.
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.config.authority);
        ctx.accounts.config.fee_bps = new_fee_bps;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee_bps: u16,
}

#[derive(Accounts)]
pub struct SetFeeVulnerable<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: Not a signer; this is the vulnerability.
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFeeSecure<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}
