use anchor_lang::prelude::*;

#[account]
pub struct Note {
    pub authority: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
pub struct CloseNote<'info> {
    // FIX: authority must match and sign before close.
    #[account(mut, has_one = authority, close = authority)]
    pub note: Account<'info, Note>,
    pub authority: Signer<'info>,
}

pub fn close_note(_ctx: Context<CloseNote>) -> Result<()> {
    Ok(())
}
