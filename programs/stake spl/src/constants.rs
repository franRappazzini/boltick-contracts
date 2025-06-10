use anchor_lang::constant;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
pub const SEED_CONFIG: &[u8] = b"config";

#[constant]
pub const SEED_VAULT: &[u8] = b"vault";

#[constant]
pub const SEED_REWARD_VAULT: &[u8] = b"reward_vault";
