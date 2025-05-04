use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
pub const SEED_CONFIG: &[u8] = b"config";

#[constant]
pub const SEED_TREASURY: &[u8] = b"treasury";
