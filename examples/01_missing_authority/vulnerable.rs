use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee_bps: u16,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    // BUG: no has_one constraint and no signer requirement.
    pub config: Account<'info, Config>,
    /// CHECK: authority passed but never validated.
    pub authority: UncheckedAccount<'info>,
}

pub fn update_config(ctx: Context<UpdateConfig>, new_fee_bps: u16) -> Result<()> {
    // BUG: Anyone can update the fee, regardless of who the authority is.
    ctx.accounts.config.fee_bps = new_fee_bps;
    Ok(())
}
