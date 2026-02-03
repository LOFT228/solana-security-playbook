use anchor_lang::prelude::*;

#[program]
pub mod pda_seed_validation {
    use super::*;

    /// Vulnerable: trusts user-provided bump and account without verifying PDA seeds.
    /// An attacker can supply a fake account and bypass ownership assumptions.
    pub fn write_config_vulnerable(ctx: Context<WriteConfigVulnerable>, value: u64) -> Result<()> {
        ctx.accounts.config.value = value;
        Ok(())
    }

    /// Secure: enforce PDA seeds and bump via Anchor constraints.
    pub fn write_config_secure(ctx: Context<WriteConfigSecure>, value: u64) -> Result<()> {
        ctx.accounts.config.value = value;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub value: u64,
}

#[derive(Accounts)]
pub struct WriteConfigVulnerable<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: Bump is passed in but never validated.
    pub bump: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WriteConfigSecure<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
}
