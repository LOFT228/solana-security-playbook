use anchor_lang::prelude::*;

#[program]
pub mod fake_account_vulnerable {
    use super::*;

    /// Vulnerable: accepts arbitrary account data without owner checks.
    pub fn read_points(ctx: Context<ReadPointsVulnerable>) -> Result<u64> {
        let data = ctx.accounts.user_state.try_borrow_data()?;
        // Attacker can provide any bytes; we treat them as a score.
        let points = u64::from_le_bytes(data[0..8].try_into().unwrap());
        Ok(points)
    }
}

#[derive(Accounts)]
pub struct ReadPointsVulnerable<'info> {
    /// CHECK: No owner check; any account can be passed.
    pub user_state: AccountInfo<'info>,
}
