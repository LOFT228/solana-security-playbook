use anchor_lang::prelude::*;

#[account]
pub struct RewardState {
    pub admin: Pubkey,
    pub reward_rate: u64,
}

#[derive(Accounts)]
pub struct SetReward<'info> {
    /// CHECK: treated as RewardState but owner is never verified.
    pub state: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

pub fn set_reward(ctx: Context<SetReward>, new_rate: u64) -> Result<()> {
    // BUG: attacker can pass a fake account they own with crafted data.
    let mut data: RewardState = RewardState::try_from_slice(&ctx.accounts.state.data.borrow())?;
    require_keys_eq!(data.admin, ctx.accounts.admin.key(), ErrorCode::Unauthorized);

    data.reward_rate = new_rate;
    data.serialize(&mut &mut ctx.accounts.state.data.borrow_mut()[..])?;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
