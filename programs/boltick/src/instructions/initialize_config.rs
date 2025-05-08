use anchor_lang::prelude::*;

use crate::{Config, ANCHOR_DISCRIMINATOR, SEED_CONFIG, SEED_TREASURY};

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

    #[account(
        mut,
        seeds = [SEED_TREASURY],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    ctx.accounts.config.set_inner(Config {
        authority: ctx.accounts.authority.key(),
        treasury: ctx.accounts.treasury.key(),
        event_count: 0,
        treasury_bump: ctx.bumps.treasury,
        bump: ctx.bumps.config,
    });

    Ok(())
}
