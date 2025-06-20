use anchor_lang::prelude::*;

use crate::{constants::REWARD_PRECISION, errors::DappError};

#[account]
#[derive(InitSpace)]
/// Configuration state for the staking program.
///
/// # Fields
/// - `authority`: The public key of the account with authority over the staking program.
/// - `bolt_mint`: The public key of the BOLT token mint.
/// - `bolt_staking_vault`: The public key of the vault holding staked BOLT tokens.
/// - `reward_vault`: The public key of the vault holding reward tokens.
/// - `reward_rate`: The rate at which rewards are distributed (tokens per second).
/// - `reward_per_token`: The amount of rewards per token staked, adjusted for the last update time.
/// - `last_update_time`: The last timestamp when rewards or staking state was updated.
/// - `total_staked`: The total amount of BOLT tokens currently staked.
/// - `reward_duration`: The duration (in seconds) over which rewards are distributed.
/// - `lock_period`: The minimum lock period (in seconds) for staked tokens.
/// - `total_rewards_distributed`: The total amount of rewards distributed so far.
/// - `max_stake_per_user`: The maximum amount of BOLT tokens a single user can stake.
/// - `paused`: A boolean indicating whether the staking program is paused.
/// - `bolt_staking_vault_bump`: The bump seed for the BOLT staking vault PDA.
/// - `bump`: The bump seed for the config PDA.
pub struct Config {
    pub authority: Pubkey,
    pub bolt_mint: Pubkey,
    pub bolt_staking_vault: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_rate: u64,
    pub reward_per_token: u128,
    pub last_update_time: u64,
    pub total_staked: u64,
    pub reward_duration: u64,
    pub lock_period: u64,
    pub total_rewards_distributed: u64,
    pub max_stake_per_user: u64,
    pub paused: bool,
    pub bolt_staking_vault_bump: u8,
    pub bump: u8,
}

impl Config {
    /// Updates the `reward_per_token` value based on the elapsed time since the last update.
    ///
    /// This function calculates the amount of reward accrued since the last update, up to the end of the current reward duration.
    /// If there are staked tokens (`total_staked > 0`), it computes the increment in reward per token and updates the `reward_per_token` field.
    /// The function also updates `last_update_time` to the current timestamp.
    ///
    /// # Errors
    ///
    /// Returns a `DappError::ArithmeticOverflow` if any arithmetic operation overflows.
    /// Returns an error if the current clock time cannot be retrieved.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update is successful.
    pub fn update_reward_per_token(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;

        if self.total_staked > 0 {
            let end_time = self.last_update_time + self.reward_duration;
            let effective_time = std::cmp::min(now, end_time);
            let time_elapsed = effective_time.saturating_sub(self.last_update_time);

            let reward_accrued = (self.reward_rate as u128) * (time_elapsed as u128);

            let reward_per_token_increment = reward_accrued
                .checked_mul(REWARD_PRECISION)
                .ok_or(DappError::ArithmeticOverflow)?
                .checked_div(self.total_staked as u128)
                .unwrap();

            self.reward_per_token = self
                .reward_per_token
                .checked_add(reward_per_token_increment)
                .ok_or(DappError::ArithmeticOverflow)?;
        }

        self.last_update_time = now;

        Ok(())
    }
}
