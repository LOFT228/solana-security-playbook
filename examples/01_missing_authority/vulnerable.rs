use anchor_lang::prelude::*;

#[program]
pub mod missing_authority_vulnerable {
    use super::*;

    /// Vulnerable: anyone can change config because authority is never checked.
    pub fn set_admin(ctx: Context<SetAdminVulnerable>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.config.admin = new_admin;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct SetAdminVulnerable<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: Not required to match config.admin.
    pub caller: AccountInfo<'info>,
}
