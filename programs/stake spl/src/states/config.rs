use anchor_lang::prelude::*;

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
