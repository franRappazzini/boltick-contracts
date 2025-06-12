use anchor_lang::prelude::*;

/// Represents a stake made by a depositor in the staking program.
///
/// Fields:
/// - `depositor`: The public key of the account that made the deposit.
/// - `amount`: The amount of tokens staked by the depositor.
/// - `reward_debt`: The amount of rewards already paid out or accounted for the depositor.
/// - `accumulated_reward`: The total rewards accumulated by the depositor.
/// - `initialized`: Indicates whether the stake has been initialized.
/// - `bump`: The bump seed used for PDA (Program Derived Address) generation.
#[account]
#[derive(InitSpace)]
pub struct Stake {
    pub depositor: Pubkey,
    pub amount: u64,
    pub reward_debt: u128,
    pub accumulated_reward: u64,
    pub initialized: bool,
    pub bump: u8,
}
