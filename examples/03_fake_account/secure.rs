use anchor_lang::prelude::*;

#[program]
pub mod fake_account_secure {
    use super::*;

    /// Secure: Anchor enforces account ownership and layout via Account<T>.
    pub fn read_points(ctx: Context<ReadPointsSecure>) -> Result<u64> {
        Ok(ctx.accounts.user_state.points)
    }
}

#[account]
pub struct UserState {
    pub points: u64,
}

#[derive(Accounts)]
pub struct ReadPointsSecure<'info> {
    pub user_state: Account<'info, UserState>,
}
