use anchor_lang::prelude::*;

#[account]
pub struct Balance {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Credit<'info> {
    #[account(mut)]
    pub balance: Account<'info, Balance>,
}

pub fn credit(ctx: Context<Credit>, amount: u64) -> Result<()> {
    // FIX: checked arithmetic with explicit error.
    ctx.accounts.balance.amount = ctx
        .accounts
        .balance
        .amount
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow")]
    Overflow,
}
