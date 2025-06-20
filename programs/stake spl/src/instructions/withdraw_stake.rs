use anchor_lang::prelude::*;
use anchor_spl::{
    // associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{REWARD_PRECISION, SEED_CONFIG, SEED_STAKE, SEED_VAULT},
    errors::DappError,
    instructions::withdraw_spl,
    states::{Config, Stake},
};

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
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
        constraint = config.bolt_staking_vault == bolt_staking_vault.key()
    )]
    pub bolt_staking_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_STAKE, depositor.key().as_ref()],
        bump = depositor_stake.bump,
    )]
    // pub withdrawer_stake: Account<'info, Stake>,
    pub depositor_stake: Account<'info, Stake>,

    #[account(address = config.bolt_mint @ DappError::BoltMintMismatch)]
    pub bolt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bolt_mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub depositor_bolt_account: InterfaceAccount<'info, TokenAccount>,

    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    // pub system_program: Program<'info, System>,
}

impl<'info> WithdrawStake<'info> {
    fn validate(&self, amount: u64) -> Result<()> {
        require!(amount > 0, DappError::ZeroAmount);
        require!(
            self.depositor_stake.amount >= amount,
            DappError::InsufficientStake
        );

        Ok(())
    }

    fn update_accumulated_reward(&mut self) -> Result<()> {
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

    fn update_depositor_stake_post_withdraw(&mut self, amount: u64) -> Result<()> {
        let depositor_stake: &mut Account<'info, Stake> = &mut self.depositor_stake;

        depositor_stake.amount = depositor_stake
            .amount
            .checked_sub(amount)
            .ok_or(DappError::ArithmeticUnderflow)?;

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
            .checked_sub(amount)
            .ok_or(DappError::ArithmeticUnderflow)?;

        Ok(())
    }
}

pub fn process_withdraw_stake(ctx: Context<WithdrawStake>, amount: u64) -> Result<()> {
    ctx.accounts.validate(amount)?;
    ctx.accounts.config.update_reward_per_token()?;
    ctx.accounts.update_accumulated_reward()?;

    let acc = &ctx.accounts;
    withdraw_spl(
        &acc.bolt_staking_vault,
        &acc.depositor_bolt_account,
        &acc.config,
        amount,
        &acc.bolt_mint,
        &acc.token_program,
        acc.config.bump,
    )?;

    ctx.accounts.update_depositor_stake_post_withdraw(amount)?;
    ctx.accounts.update_total_staked(amount)?;

    Ok(())
}
