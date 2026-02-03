use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[program]
pub mod missing_account_validation {
    use super::*;

    /// Vulnerable: accepts any `vault` account without checking its owner or mint.
    /// An attacker can pass their own token account as `vault` and drain funds
    /// that were meant to be protected.
    pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>, amount: u64) -> Result<()> {
        // No constraints on `vault`, so this CPI can move tokens from an
        // attacker-controlled account instead of the intended vault.
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    /// Secure: enforce owner/mint constraints so only the expected vault can be used.
    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    /// CHECK: No owner/mint constraints; this is the vulnerability.
    pub vault: AccountInfo<'info>,
    /// CHECK: No validation on the recipient either.
    pub recipient: AccountInfo<'info>,
    /// CHECK: Authority is not verified to match the vault.
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(
        // Require the vault to be a real token account owned by the program's PDA.
        constraint = vault.owner == vault_authority.key(),
        // Require a specific mint for the vault (passed in from config/state).
        constraint = vault.mint == vault_mint.key(),
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, constraint = recipient.mint == vault_mint.key())]
    pub recipient: Account<'info, TokenAccount>,
    /// CHECK: PDA authority derived and verified in the handler.
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: Mint account used only for comparison.
    pub vault_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
