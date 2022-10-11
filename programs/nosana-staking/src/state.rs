use anchor_lang::prelude::*;
use nosana_common::constants;

/***
Constants
*/

pub const STAKE_MINIMUM: u64 = constants::NOS_DECIMALS;
pub const SECONDS_PER_DAY: u128 = 24 * 60 * 60;
pub const DURATION_MIN: u128 = 14 * SECONDS_PER_DAY; // 2 weeks
pub const DURATION_MAX: u128 = 365 * SECONDS_PER_DAY; // 1 year
pub const XNOS_PRECISION: u128 = u128::pow(10, 15); // 1e15
pub const XNOS_DIV: u128 = 4 * DURATION_MAX / 12; // 0.25 growth per month

/***
 Accounts and Types
*/

/// The `SettingsAccount` struct holds the information about the
/// slashing authority and token account.
#[account]
pub struct SettingsAccount {
    pub authority: Pubkey,
    pub token_account: Pubkey,
}

impl SettingsAccount {
    pub const SIZE: usize = 8 + std::mem::size_of::<SettingsAccount>();

    pub fn set(&mut self, authority: Pubkey, token_account: Pubkey) {
        self.authority = authority;
        self.token_account = token_account;
    }
}

/// The `StakeAccount` struct holds all the information for any given stake.
#[account]
pub struct StakeAccount {
    pub amount: u64,
    pub authority: Pubkey,
    pub duration: u64,
    pub time_unstake: i64,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub xnos: u128,
}

impl StakeAccount {
    pub const SIZE: usize = 8 + std::mem::size_of::<StakeAccount>();

    pub fn init(
        &mut self,
        amount: u64,
        authority: Pubkey,
        duration: u64,
        vault: Pubkey,
        vault_bump: u8,
    ) {
        self.amount = amount;
        self.authority = authority;
        self.duration = duration;
        self.time_unstake = 0;
        self.vault = vault;
        self.vault_bump = vault_bump;
        self.update_xnos();
    }

    pub fn unstake(&mut self, time: i64) {
        self.time_unstake = time;
        self.update_xnos();
    }

    pub fn topup(&mut self, amount: u64) {
        self.amount += amount;
        self.update_xnos();
    }

    pub fn slash(&mut self, amount: u64) {
        self.amount -= amount;
        self.update_xnos();
    }

    pub fn extend(&mut self, duration: u64) {
        self.duration += duration;
        self.update_xnos();
    }

    fn update_xnos(&mut self) {
        self.xnos = if self.time_unstake != 0 {
            0
        } else {
            (u128::from(self.duration) * XNOS_PRECISION / XNOS_DIV + XNOS_PRECISION)
                * u128::from(self.amount)
                / XNOS_PRECISION
        }
    }
}
