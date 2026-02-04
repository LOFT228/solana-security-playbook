use anchor_lang::prelude::*;

#[program]
pub mod pda_misuse_vulnerable {
    use super::*;

    /// Vulnerable: accepts any account; does not verify PDA seeds.
    pub fn write_config(ctx: Context<WriteConfigVulnerable>, value: u64) -> Result<()> {
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
    /// CHECK: User-supplied bump is ignored.
    pub bump: AccountInfo<'info>,
}
