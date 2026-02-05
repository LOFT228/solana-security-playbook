use anchor_lang::prelude::*;

#[account]
pub struct Note {
    pub authority: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
pub struct CloseNote<'info> {
    // BUG: no authority check; anyone can close this account.
    #[account(mut, close = receiver)]
    pub note: Account<'info, Note>,
    /// CHECK: lamports recipient is arbitrary.
    pub receiver: AccountInfo<'info>,
}

pub fn close_note(_ctx: Context<CloseNote>) -> Result<()> {
    Ok(())
}
