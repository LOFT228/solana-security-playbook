use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

#[program]
pub mod cpi_reentrancy_secure {
    use super::*;

    /// Secure: lock state before CPI and unlock after.
    pub fn withdraw(ctx: Context<WithdrawSecure>, data: Vec<u8>) -> Result<()> {
        require!(!ctx.accounts.vault.locked, ErrorCode::Reentrancy);
        ctx.accounts.vault.locked = true;

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: vec![],
            data,
        };
        invoke_signed(&ix, &[], &[])?;

        ctx.accounts.vault.withdrawals = ctx
            .accounts
            .vault
            .withdrawals
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        ctx.accounts.vault.locked = false;
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub withdrawals: u64,
    pub locked: bool,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: CPI target should be allowlisted externally.
    pub cpi_program: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Re-entrancy detected")]
    Reentrancy,
    #[msg("Counter overflowed")]
    Overflow,
}
