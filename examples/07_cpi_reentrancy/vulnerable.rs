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
    /// CHECK: external program can re-enter.
    pub payout_program: AccountInfo<'info>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // BUG: no re-entrancy guard and state updated after CPI.
    let ix = Instruction {
        program_id: *ctx.accounts.payout_program.key,
        accounts: vec![],
        data: vec![],
    };
    invoke(&ix, &[])?;

    ctx.accounts.vault.balance = ctx.accounts.vault.balance.saturating_sub(amount);
    Ok(())
}
