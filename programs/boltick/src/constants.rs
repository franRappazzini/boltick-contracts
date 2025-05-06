use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
pub const SEED_CONFIG: &[u8] = b"config";

#[constant]
pub const SEED_TREASURY: &[u8] = b"treasury";

#[constant]
pub const SEED_EVENT: &[u8] = b"event";

#[constant]
pub const SEED_COLLECTION_MINT: &[u8] = b"collection_mint";

#[constant]
pub const SEED_COLLECTION_TOKEN_ACCOUNT: &[u8] = b"collection_token_account";
