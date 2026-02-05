use anchor_lang::prelude::*;

#[account]
pub struct RewardState {
    pub admin: Pubkey,
    pub reward_rate: u64,
}

#[derive(Accounts)]
pub struct SetReward<'info> {
    // FIX: Anchor validates the account owner automatically.
    #[account(mut)]
    pub state: Account<'info, RewardState>,
    pub admin: Signer<'info>,
}

pub fn set_reward(ctx: Context<SetReward>, new_rate: u64) -> Result<()> {
    require_keys_eq!(ctx.accounts.state.admin, ctx.accounts.admin.key(), ErrorCode::Unauthorized);
    ctx.accounts.state.reward_rate = new_rate;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
