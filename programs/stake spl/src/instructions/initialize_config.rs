use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::{ANCHOR_DISCRIMINATOR, SEED_CONFIG, SEED_REWARD_VAULT, SEED_VAULT},
    states::Config,
};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Config::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [SEED_CONFIG],
        bump
    )]
    pub config: Account<'info, Config>,

    pub bolt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_VAULT],
        bump,
        token::mint = bolt_mint,
        token::authority = authority,
        token::token_program = token_program
    )]
    pub bolt_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_REWARD_VAULT],
        bump,
        token::mint = bolt_mint,
        token::authority = authority,
        token::token_program = token_program
    )]
    pub bolt_reward_vault: InterfaceAccount<'info, TokenAccount>,

    // pub associated_tok
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    ctx.accounts.config.set_inner(Config {
        authority: ctx.accounts.authority.key(),
        bolt_mint: ctx.accounts.bolt_mint.key(),
        bolt_vault: ctx.accounts.bolt_vault.key(),
        reward_vault: ctx.accounts.bolt_reward_vault.key(),
        reward_rate: 0,
        last_update_time: 0,
        total_staked: 0,
        reward_duration: 0,
        lock_period: 0,
        total_rewards_distributed: 0,
        max_stake_per_user: 0,
        paused: false,
        bolt_vault_bump: ctx.bumps.bolt_vault,
        bump: ctx.bumps.config,
    });

    Ok(())
}
