mod constants;
mod instructions;
mod states;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use states::*;

declare_id!("7GreiHo9FwZBLucB2r8Mj3czeRGYWUJqbHqRetYNNsFm");

#[program]
pub mod boltick {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
}
