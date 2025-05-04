use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    Ok(())
}
