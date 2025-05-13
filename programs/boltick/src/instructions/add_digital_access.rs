use anchor_lang::prelude::*;

use crate::{
    DappError, DigitalAccess, Event, ANCHOR_DISCRIMINATOR, SEED_DIGITAL_ACCESS, SEED_EVENT,
};

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct AddDigitalAccess<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_EVENT, event_id.to_le_bytes().as_ref()],
        bump = event.bump,
        has_one = creator @ DappError::InvalidCreator,
    )]
    pub event: Account<'info, Event>,

    #[account(
        init,
        payer = creator,
        space = DigitalAccess::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [
            SEED_DIGITAL_ACCESS,
            event.key().as_ref(),
            event.current_digital_access_count.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub digital_access: Account<'info, DigitalAccess>,

    pub system_program: Program<'info, System>,
}

pub fn process_add_digital_access(
    ctx: Context<AddDigitalAccess>,
    _event_id: u64,
    price: u64,
    max_supply: u64,
    name: String,
    symbol: String,
    description: String,
    uri: String,
) -> Result<()> {
    ctx.accounts.digital_access.set_inner(DigitalAccess {
        event: ctx.accounts.event.key(),
        id: ctx.accounts.event.current_digital_access_count,
        price,
        max_supply,
        current_minted: 0,
        name,
        symbol,
        description,
        uri,
        bump: ctx.bumps.digital_access,
    });

    ctx.accounts.event.current_digital_access_count += 1;

    Ok(())
}
