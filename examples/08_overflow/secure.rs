use anchor_lang::prelude::*;

#[program]
pub mod overflow_secure {
    use super::*;

    /// Secure: checked subtraction prevents underflow.
    pub fn spend(ctx: Context<SpendSecure>, amount: u64) -> Result<()> {
        ctx.accounts.wallet.balance = ctx
            .accounts
            .wallet
            .balance
            .checked_sub(amount)
            .ok_or(ErrorCode::Underflow)?;
        Ok(())
    }
}

#[account]
pub struct Wallet {
    pub balance: u64,
}

#[derive(Accounts)]
pub struct SpendSecure<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    Underflow,
}
