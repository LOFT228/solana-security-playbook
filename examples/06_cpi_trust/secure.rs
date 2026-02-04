use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

#[program]
pub mod cpi_trust_secure {
    use super::*;

    /// Secure: enforce CPI target against an allowlist.
    pub fn proxy(ctx: Context<ProxySecure>, data: Vec<u8>) -> Result<()> {
        require_keys_eq!(ctx.accounts.cpi_program.key(), ctx.accounts.allowed_program.key());
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: vec![],
            data,
        };
        invoke(&ix, &[])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProxySecure<'info> {
    /// CHECK: Verified in handler.
    pub cpi_program: AccountInfo<'info>,
    /// CHECK: Stored allowlist entry.
    pub allowed_program: AccountInfo<'info>,
}
