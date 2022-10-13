use crate::*;

#[derive(Accounts)]
pub struct Extend<'info> {
    #[account(
        mut,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.time_unstake == 0 @ NosanaError::StakeAlreadyUnstaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    pub authority: Signer<'info>,
}

impl<'info> Extend<'info> {
    pub fn handler(&mut self, duration: u64) -> Result<()> {
        // test duration
        require!(duration > 0, NosanaError::StakeDurationTooShort);

        // test new duration
        require!(
            self.stake.duration + duration <= u64::try_from(DURATION_MAX).unwrap(),
            NosanaError::StakeDurationTooLong
        );

        // extend stake
        self.stake.extend(duration)
    }
}
