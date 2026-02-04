use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

#[program]
pub mod cpi_reentrancy_vulnerable {
    use super::*;

    /// Vulnerable: updates state after CPI without any re-entrancy guard.
    pub fn withdraw(ctx: Context<WithdrawVulnerable>, data: Vec<u8>) -> Result<()> {
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: vec![],
            data,
        };
        invoke_signed(&ix, &[], &[])?;
        ctx.accounts.vault.withdrawals += 1;
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub withdrawals: u64,
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Untrusted CPI program.
    pub cpi_program: AccountInfo<'info>,
}
