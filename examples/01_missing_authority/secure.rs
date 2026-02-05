use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee_bps: u16,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    // FIX: require the authority stored in config to match the signer.
    #[account(has_one = authority)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}

pub fn update_config(ctx: Context<UpdateConfig>, new_fee_bps: u16) -> Result<()> {
    ctx.accounts.config.fee_bps = new_fee_bps;
    Ok(())
}
