use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{ANCHOR_DISCRIMINATOR, REWARD_PRECISION, SEED_CONFIG, SEED_STAKE, SEED_VAULT},
    errors::DappError,
    instructions::deposit_spl,
    states::{Config, Stake},
};

#[derive(Accounts)]
pub struct DepositStake<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        constraint = !config.paused @ DappError::StakingPaused,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_VAULT],
        bump = config.bolt_staking_vault_bump,
        constraint = config.bolt_staking_vault == bolt_staking_vault.key(),
    )]
    pub bolt_staking_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = Stake::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [SEED_STAKE, depositor.key().as_ref()],
        bump
    )]
    pub depositor_stake: Account<'info, Stake>,

    #[account(constraint = bolt_mint.key() == config.bolt_mint @ DappError::BoltMintMismatch)]
    pub bolt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bolt_mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub depositor_bolt_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositStake<'info> {
    pub fn validate(&self, amount: u64) -> Result<()> {
        require!(amount > 0, DappError::ZeroAmount);
        require!(
            self.depositor_stake
                .amount
                .checked_add(amount)
                .ok_or(DappError::ArithmeticOverflow)?
                <= self.config.max_stake_per_user,
            DappError::AmountExceedsLimit
        );
        Ok(())
    }

    pub fn init_stake_if_needed(&mut self, bump: u8) {
        if !self.depositor_stake.initialized {
            self.depositor_stake.set_inner(Stake {
                depositor: self.depositor.key(),
                amount: 0,
                reward_debt: 0,
                accumulated_reward: 0,
                initialized: true,
                bump,
            });
        }
    }

    pub fn update_reward_per_token(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;
        let config = &mut self.config;

        if config.total_staked > 0 {
            let end_time = config.last_update_time + config.reward_duration;
            let effective_time = std::cmp::min(now, end_time);
            let time_elapsed = effective_time.saturating_sub(config.last_update_time);

            let reward_accrued = (config.reward_rate as u128) * (time_elapsed as u128);

            let reward_per_token_increment = reward_accrued
                .checked_mul(REWARD_PRECISION)
                .ok_or(DappError::ArithmeticOverflow)?
                .checked_div(config.total_staked as u128)
                .unwrap();

            config.reward_per_token = config
                .reward_per_token
                .checked_add(reward_per_token_increment)
                .ok_or(DappError::ArithmeticOverflow)?;
        }

        config.last_update_time = now;

        Ok(())
    }

    pub fn update_accumulated_reward(&mut self) -> Result<()> {
        let depositor_stake = &mut self.depositor_stake;

        // Calculate pending rewards for the user before updating their stake
        let pending_reward = (depositor_stake.amount as u128)
            .checked_mul(self.config.reward_per_token)
            .ok_or(DappError::ArithmeticOverflow)?
            .checked_div(REWARD_PRECISION)
            .unwrap()
            .checked_sub(depositor_stake.reward_debt)
            .unwrap_or(0);

        depositor_stake.accumulated_reward = (depositor_stake.accumulated_reward as u128)
            .checked_add(pending_reward)
            .ok_or(DappError::ArithmeticOverflow)?
            as u64;

        Ok(())
    }

    pub fn update_depositor_stake_post_deposit(&mut self, amount: u64) -> Result<()> {
        let depositor_stake = &mut self.depositor_stake;

        depositor_stake.amount = depositor_stake
            .amount
            .checked_add(amount)
            .ok_or(DappError::ArithmeticOverflow)?;
        depositor_stake.reward_debt = (depositor_stake.amount as u128)
            .checked_mul(self.config.reward_per_token)
            .ok_or(DappError::ArithmeticOverflow)?
            .checked_div(REWARD_PRECISION)
            .unwrap();

        Ok(())
    }

    pub fn update_total_staked(&mut self, amount: u64) -> Result<()> {
        let config = &mut self.config;
        config.total_staked = config
            .total_staked
            .checked_add(amount)
            .ok_or(DappError::ArithmeticOverflow)?;

        Ok(())
    }
}

pub fn process_deposit_stake(ctx: Context<DepositStake>, amount: u64) -> Result<()> {
    ctx.accounts.validate(amount)?;
    ctx.accounts.init_stake_if_needed(ctx.bumps.depositor_stake);
    ctx.accounts.update_reward_per_token()?;
    ctx.accounts.update_accumulated_reward()?;

    let acc = &ctx.accounts;
    deposit_spl(
        &acc.depositor,
        &acc.depositor_bolt_account,
        &acc.bolt_staking_vault,
        amount,
        &acc.bolt_mint,
        &acc.token_program,
    )?;

    ctx.accounts.update_depositor_stake_post_deposit(amount)?;
    ctx.accounts.update_total_staked(amount)?;

    Ok(())
}
