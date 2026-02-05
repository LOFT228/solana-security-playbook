use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
    pub locked: bool,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    /// CHECK: external program can be called, but we guard re-entry.
    pub payout_program: AccountInfo<'info>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // FIX: re-entrancy guard.
    require!(!ctx.accounts.vault.locked, ErrorCode::Reentrancy);
    ctx.accounts.vault.locked = true;

    // FIX: update state before CPI.
    require!(ctx.accounts.vault.balance >= amount, ErrorCode::InsufficientFunds);
    ctx.accounts.vault.balance = ctx.accounts.vault.balance.saturating_sub(amount);

    let ix = Instruction {
        program_id: *ctx.accounts.payout_program.key,
        accounts: vec![],
        data: vec![],
    };
    invoke(&ix, &[])?;

    // Unlock after CPI completes.
    ctx.accounts.vault.locked = false;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Re-entrancy detected")]
    Reentrancy,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
