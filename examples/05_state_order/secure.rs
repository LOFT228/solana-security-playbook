use anchor_lang::prelude::*;

#[program]
pub mod state_order_secure {
    use super::*;

    /// Secure: validate first, then mutate.
    pub fn add_points(ctx: Context<AddPointsSecure>, amount: u64) -> Result<()> {
        let new_total = ctx
            .accounts
            .counter
            .total
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        require!(new_total <= ctx.accounts.counter.limit, ErrorCode::OverLimit);
        ctx.accounts.counter.total = new_total;
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub total: u64,
    pub limit: u64,
}

#[derive(Accounts)]
pub struct AddPointsSecure<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter exceeded limit")]
    OverLimit,
    #[msg("Counter overflowed")]
    Overflow,
}
