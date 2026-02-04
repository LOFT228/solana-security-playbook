use anchor_lang::prelude::*;

#[program]
pub mod overflow_vulnerable {
    use super::*;

    /// Vulnerable: unchecked subtraction can underflow.
    pub fn spend(ctx: Context<SpendVulnerable>, amount: u64) -> Result<()> {
        ctx.accounts.wallet.balance -= amount;
        Ok(())
    }
}

#[account]
pub struct Wallet {
    pub balance: u64,
}

#[derive(Accounts)]
pub struct SpendVulnerable<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
}
