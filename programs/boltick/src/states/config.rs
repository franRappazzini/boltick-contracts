use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub event_count: u64,
    pub treasury_bump: u8,
    pub bump: u8,
}
