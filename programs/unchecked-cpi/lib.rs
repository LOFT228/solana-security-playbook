use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

#[program]
pub mod unchecked_cpi {
    use super::*;

    /// Vulnerable: accepts any CPI program and does not guard against re-entrancy.
    /// A malicious program can re-enter and mutate state multiple times.
    pub fn dispatch_vulnerable(ctx: Context<DispatchVulnerable>, data: Vec<u8>) -> Result<()> {
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: vec![],
            data,
        };
        // No program ID validation and no re-entrancy lock.
        invoke_signed(&ix, &[], &[])?;
        ctx.accounts.state.calls += 1;
        Ok(())
    }

    /// Secure: validate the CPI target and use a re-entrancy guard.
    pub fn dispatch_secure(ctx: Context<DispatchSecure>, data: Vec<u8>) -> Result<()> {
        require!(!ctx.accounts.state.locked, ErrorCode::Reentrancy);
        require_keys_eq!(ctx.accounts.cpi_program.key(), ctx.accounts.allowed_program);

        ctx.accounts.state.locked = true;
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: vec![],
            data,
        };
        invoke_signed(&ix, &[], &[])?;
        ctx.accounts.state.calls = ctx.accounts.state.calls.checked_add(1).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.state.locked = false;
        Ok(())
    }
}

#[account]
pub struct State {
    pub calls: u64,
    pub locked: bool,
}

#[derive(Accounts)]
pub struct DispatchVulnerable<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    /// CHECK: Any program can be passed here.
    pub cpi_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DispatchSecure<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    /// CHECK: Verified in handler against `allowed_program`.
    pub cpi_program: AccountInfo<'info>,
    /// CHECK: Stored in config/state; only used for comparison.
    pub allowed_program: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Re-entrancy detected")]
    Reentrancy,
    #[msg("Counter overflowed")]
    Overflow,
}
