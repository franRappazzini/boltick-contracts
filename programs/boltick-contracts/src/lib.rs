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
}
