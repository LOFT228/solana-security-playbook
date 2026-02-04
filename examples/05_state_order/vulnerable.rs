use anchor_lang::prelude::*;

#[program]
pub mod state_order_vulnerable {
    use super::*;

    /// Vulnerable: increments total before validating the limit.
    pub fn add_points(ctx: Context<AddPointsVulnerable>, amount: u64) -> Result<()> {
        ctx.accounts.counter.total = ctx.accounts.counter.total.saturating_add(amount);
        // Validation happens after mutation; if it fails, state is already modified.
        require!(ctx.accounts.counter.total <= ctx.accounts.counter.limit, ErrorCode::OverLimit);
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub total: u64,
    pub limit: u64,
}

#[derive(Accounts)]
pub struct AddPointsVulnerable<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter exceeded limit")]
    OverLimit,
}
