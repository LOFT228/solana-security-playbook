use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

#[program]
pub mod cpi_trust_vulnerable {
    use super::*;

    /// Vulnerable: calls any program passed by the user.
    pub fn proxy(ctx: Context<ProxyVulnerable>, data: Vec<u8>) -> Result<()> {
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
pub struct ProxyVulnerable<'info> {
    /// CHECK: No program ID validation.
    pub cpi_program: AccountInfo<'info>,
}
