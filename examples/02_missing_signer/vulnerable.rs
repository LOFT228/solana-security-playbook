use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    /// CHECK: authority is not required to sign.
    pub authority: UncheckedAccount<'info>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // BUG: attacker can pass the real authority pubkey without signature.
    ctx.accounts.vault.balance = ctx.accounts.vault.balance.saturating_sub(amount);
    Ok(())
}
