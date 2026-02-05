use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke;

#[account]
pub struct UserState {
    pub points: u64,
}

#[derive(Accounts)]
pub struct SyncPoints<'info> {
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    /// CHECK: any program can be passed here.
    pub rewards_program: AccountInfo<'info>,
}

pub fn sync_points(ctx: Context<SyncPoints>) -> Result<()> {
    // BUG: CPI target is not validated and could be malicious.
    let ix = Instruction {
        program_id: *ctx.accounts.rewards_program.key,
        accounts: vec![],
        data: vec![],
    };
    invoke(&ix, &[])?;

    // BUG: blindly trust that CPI set points correctly.
    ctx.accounts.user_state.points = ctx.accounts.user_state.points.saturating_add(10);
    Ok(())
}
