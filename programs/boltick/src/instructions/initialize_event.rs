use anchor_lang::prelude::*;
use anchor_spl::{metadata::{self, mpl_token_metadata::types::CollectionDetails, Metadata}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{Config, Event, ANCHOR_DISCRIMINATOR, SEED_COLLECTION_MINT, SEED_COLLECTION_TOKEN_ACCOUNT, SEED_CONFIG, SEED_EVENT};

use super::{create_master_edition, create_metadata_accounts, mint_to};

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Event::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [SEED_EVENT, config.event_count.to_le_bytes().as_ref()],
        bump

    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = creator,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        seeds = [SEED_COLLECTION_MINT, config.event_count.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = creator, 
        token::mint = collection_mint,
        token::authority = collection_mint,
        token::token_program = token_program,
        seeds = [SEED_COLLECTION_TOKEN_ACCOUNT, config.event_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_initialize_event(
    ctx: Context<InitializeEvent>,
    name: String,
    symbol: String,
    uri: String,
    event_description: String
) -> Result<()> {
    let acc = &ctx.accounts;
    let event_count: [u8; 8] = acc.config.event_count.to_le_bytes();

    let signer_seeds : &[&[&[u8]]] = &[&[
        SEED_COLLECTION_MINT,
        &event_count,
        &[ctx.bumps.collection_mint]
    ]];

    // mint collection
    mint_to(
        &acc.token_program,
        &acc.collection_mint,
        &acc.collection_token_account,
        &acc.collection_mint,
        signer_seeds
    )?;
    
    // create metadata
    create_metadata_accounts(
        &acc.token_metadata_program,
        &acc.metadata_account,
        &acc.collection_mint,
        &acc.collection_mint,
        &acc.creator,
        &acc.system_program,
        &acc.rent,
        signer_seeds,
        name.clone(),
        symbol,
        uri,
        0,
        None,
        false,
        Some(CollectionDetails::V1 { size: 0 })
    )?;
    
    // create master edition
    create_master_edition(
        &acc.token_metadata_program,
        &acc.edition_account,
        &acc.collection_mint,
        &acc.collection_mint,
        &acc.creator,
        &acc.metadata_account,
        &acc.token_program,
        &acc.system_program,
        &acc.rent,
        signer_seeds
    )?;

    // verify collection metadata 
    metadata::sign_metadata(CpiContext::new_with_signer(
        acc.token_metadata_program.to_account_info(),
        metadata::SignMetadata {
            creator: acc.collection_mint.to_account_info(),
            metadata: acc.metadata_account.to_account_info()
        },
        signer_seeds
    ))?;


    ctx.accounts.event.set_inner(Event {
        creator: acc.creator.key(),
        collection_mint_account: acc.collection_mint.key(),
        current_nft_count: 0,
        current_digital_access_count: 0,
        date: Clock::get()?.unix_timestamp,
        name,
        description: event_description,
        bump: ctx.bumps.event,
    });
    
    ctx.accounts.config.event_count += 1;

    Ok(())
}
