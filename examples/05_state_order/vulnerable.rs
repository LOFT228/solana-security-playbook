use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Spend<'info> {
    #[account(mut, has_one = authority)]
    pub treasury: Account<'info, Treasury>,
    pub authority: Signer<'info>,
}

pub fn spend(ctx: Context<Spend>, amount: u64) -> Result<()> {
    // BUG: state is mutated before validation.
    ctx.accounts.treasury.balance = ctx.accounts.treasury.balance.saturating_sub(amount);

    // Later validation fails, but the mutation already happened.
    require!(amount > 0, ErrorCode::InvalidAmount);

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
}
