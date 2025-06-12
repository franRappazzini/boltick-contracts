mod constants;
mod errors;
mod instructions;
mod states;

use anchor_lang::prelude::*;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use states::*;

declare_id!("1aUtb8YSYuGUXkouRHy5bpxuuiyViqpgAUDb6rK7a8E");

#[program]
pub mod boltick {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        name: String,
        symbol: String,
        uri: String,
        event_description: String,
    ) -> Result<()> {
        process_initialize_event(ctx, name, symbol, uri, event_description)
    }

    pub fn add_digital_access(
        ctx: Context<AddDigitalAccess>,
        event_id: u64,
        price: u64,
        max_supply: u64,
        name: String,
        symbol: String,
        description: String,
        uri: String,
    ) -> Result<()> {
        process_add_digital_access(
            ctx,
            event_id,
            price,
            max_supply,
            name,
            symbol,
            description,
            uri,
        )
    }

    pub fn mint_token(ctx: Context<MintToken>, event_id: u64, digital_access_id: u8) -> Result<()> {
        process_mint_token(ctx, event_id, digital_access_id)
    }

    pub fn buy_token(ctx: Context<BuyToken>, event_id: u64, digital_access_id: u8) -> Result<()> {
        process_buy_token(ctx, event_id, digital_access_id)
    }

    pub fn update_token_metadata(
        ctx: Context<UpdateTokenMetadata>,
        event_id: u64,
        token_id: u64,
        uri: String,
    ) -> Result<()> {
        process_update_token_metadata(ctx, event_id, token_id, uri)
    }
}
