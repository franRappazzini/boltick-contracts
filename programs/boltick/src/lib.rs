mod constants;
mod instructions;
mod states;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use states::*;

declare_id!("B1dbydiCFRgTz9ZZtxGZ63AvBhwwguAbZe2CgmG3JJyY");

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
        ticket_price: u64,
    ) -> Result<()> {
        process_initialize_event(ctx, name, symbol, uri, event_description, ticket_price)
    }

    pub fn mint_token(
        ctx: Context<MintToken>,
        event_id: u64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        process_mint_token(ctx, event_id, name, symbol, uri)
    }

    pub fn buy_token(
        ctx: Context<BuyToken>,
        event_id: u64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        process_buy_token(ctx, event_id, name, symbol, uri)
    }
}
