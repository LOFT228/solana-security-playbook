use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // FIX: enforce PDA derivation.
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    ctx.accounts.vault.balance = ctx.accounts.vault.balance.saturating_add(amount);
    Ok(())
}
