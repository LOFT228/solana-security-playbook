use anchor_lang::prelude::*;

#[program]
pub mod unsafe_arithmetic {
    use super::*;

    /// Vulnerable: uses unchecked addition which can overflow and reset counters.
    pub fn increment_vulnerable(ctx: Context<Increment>, amount: u64) -> Result<()> {
        // If total is near u64::MAX, this wraps to 0.
        ctx.accounts.counter.total += amount;
        Ok(())
    }

    /// Secure: use checked math and return an error on overflow.
    pub fn increment_secure(ctx: Context<Increment>, amount: u64) -> Result<()> {
        ctx.accounts.counter.total = ctx
            .accounts
            .counter
            .total
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub total: u64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter overflowed")]
    Overflow,
}
