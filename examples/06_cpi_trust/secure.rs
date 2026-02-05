use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke;

pub const REWARDS_PROGRAM_ID: Pubkey = pubkey!("Reward111111111111111111111111111111111111");

#[account]
pub struct UserState {
    pub points: u64,
}

#[derive(Accounts)]
pub struct SyncPoints<'info> {
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    /// CHECK: program is validated explicitly.
    pub rewards_program: AccountInfo<'info>,
}

pub fn sync_points(ctx: Context<SyncPoints>) -> Result<()> {
    // FIX: ensure CPI goes only to the intended program.
    require_keys_eq!(ctx.accounts.rewards_program.key(), REWARDS_PROGRAM_ID, ErrorCode::InvalidProgram);

    let ix = Instruction {
        program_id: REWARDS_PROGRAM_ID,
        accounts: vec![],
        data: vec![],
    };
    invoke(&ix, &[])?;

    // FIX: validate post-conditions instead of blindly trusting CPI side effects.
    require!(ctx.accounts.user_state.points <= 1_000_000, ErrorCode::InvalidPoints);
    ctx.accounts.user_state.points = ctx.accounts.user_state.points.saturating_add(10);
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid rewards program")]
    InvalidProgram,
    #[msg("Invalid points state")]
    InvalidPoints,
}
