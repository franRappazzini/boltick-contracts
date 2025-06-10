mod constants;
mod instructions;
mod states;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("64E4SYr3hmvxegaKhRxCcD5Di6UwBP8Y7u32hzd5VgnL");

#[program]
pub mod stake_spl {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
}
