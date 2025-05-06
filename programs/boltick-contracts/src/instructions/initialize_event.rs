use anchor_lang::prelude::*;
use anchor_spl::{metadata::{self, mpl_token_metadata::types::{Creator, DataV2}, Metadata}, token_interface::{self, Mint, TokenAccount, TokenInterface}};

use crate::{Config, Event, ANCHOR_DISCRIMINATOR, SEED_COLLECTION_MINT, SEED_COLLECTION_TOKEN_ACCOUNT, SEED_CONFIG, SEED_EVENT};

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
    event_description: String,
) -> Result<()> {
    let acc = &ctx.accounts;
    let event_count = acc.config.event_count.to_le_bytes();

    let signer_seeds : &[&[&[u8]]] = &[&[
        SEED_COLLECTION_MINT,
        &event_count,
        &[ctx.bumps.collection_mint]
    ]];

    // mint collection
    token_interface::mint_to(
        CpiContext::new_with_signer(
            acc.token_program.to_account_info(),
            token_interface::MintTo {
                mint: acc.collection_mint.to_account_info(),
                to: acc.collection_token_account.to_account_info(),
                authority: acc.collection_mint.to_account_info()
            },
            signer_seeds
        ),
        1 
    )?;

    // create metadata
    metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            acc.token_metadata_program.to_account_info(),
            metadata::CreateMetadataAccountsV3 {
                metadata: acc.metadata_account.to_account_info(),
                mint: acc.collection_mint.to_account_info(),
                mint_authority: acc.collection_mint.to_account_info(),
                payer: acc.creator.to_account_info(),
                update_authority: acc.collection_mint.to_account_info(),
                system_program: acc.system_program.to_account_info(),
                rent: acc.rent.to_account_info()
            },
            signer_seeds
        ),
        DataV2 {
            name: name.clone(),
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: Some(vec![ // TODO (fran): CHECK
                Creator {
                    address: acc.collection_mint.key(),
                    verified: true,
                    share: 100,
                },
            ]),
            collection: None,
            uses: None
        },
        false,
        false,
        Some(metadata::mpl_token_metadata::types::CollectionDetails::V1 { size: 0 })
    )?;

    // create master edition
    metadata::create_master_edition_v3(
        CpiContext::new_with_signer(
            acc.token_metadata_program.to_account_info(),
            metadata::CreateMasterEditionV3 {
                edition: acc.edition_account.to_account_info(),
                mint: acc.collection_mint.to_account_info(),
                update_authority: acc.collection_mint.to_account_info(),
                mint_authority: acc.collection_mint.to_account_info(),
                payer: acc.creator.to_account_info(),
                metadata: acc.metadata_account.to_account_info(),
                token_program: acc.token_program.to_account_info(),
                system_program: acc.system_program.to_account_info(),
                rent: acc.rent.to_account_info() 
            },
            signer_seeds
        ),
        Some(0)
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
        date: Clock::get()?.unix_timestamp,
        name,
        description: event_description
    });
    
    ctx.accounts.config.event_count += 1;

    Ok(())
}
