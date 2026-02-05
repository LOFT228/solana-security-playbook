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
    // BUG: unchecked add can overflow and wrap.
    ctx.accounts.balance.amount += amount;
    Ok(())
}
