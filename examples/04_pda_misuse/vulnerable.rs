use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // BUG: vault is assumed to be the PDA but seeds/bump are not enforced.
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub depositor: Signer<'info>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    ctx.accounts.vault.balance = ctx.accounts.vault.balance.saturating_add(amount);
    Ok(())
}
