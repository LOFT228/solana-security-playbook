use anchor_lang::prelude::*;

#[program]
pub mod missing_authority_secure {
    use super::*;

    /// Secure: require the admin to sign and match the stored authority.
    pub fn set_admin(ctx: Context<SetAdminSecure>, new_admin: Pubkey) -> Result<()> {
        require_keys_eq!(ctx.accounts.admin.key(), ctx.accounts.config.admin);
        ctx.accounts.config.admin = new_admin;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct SetAdminSecure<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub admin: Signer<'info>,
}
