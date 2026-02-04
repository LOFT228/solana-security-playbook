use anchor_lang::prelude::*;

#[program]
pub mod pda_misuse_secure {
    use super::*;

    /// Secure: enforce seeds and bump for the PDA.
    pub fn write_config(ctx: Context<WriteConfigSecure>, value: u64) -> Result<()> {
        ctx.accounts.config.value = value;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub value: u64,
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
