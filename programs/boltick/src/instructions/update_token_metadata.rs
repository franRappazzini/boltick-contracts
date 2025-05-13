use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{self, mpl_token_metadata::types::DataV2, Metadata, MetadataAccount},
    token_interface::Mint,
};

use crate::{
    Config, DappError, Event, SEED_COLLECTION_MINT, SEED_CONFIG, SEED_EVENT, SEED_TOKEN_MINT,
};

#[derive(Accounts)]
#[instruction(event_id: u64, token_id: u64, /* digital_access_id: u8 */)]
pub struct UpdateTokenMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ DappError::InvalidAuthority,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [SEED_EVENT, event_id.to_le_bytes().as_ref()],
        bump = event.bump,
        constraint = event.collection_mint_account == collection_mint.key()
    )]
    pub event: Account<'info, Event>,

    // // TODO (fran): maybe to update uri
    // #[account(
    //     seeds = [
    //         SEED_DIGITAL_ACCESS,
    //         event.key().as_ref(),
    //         digital_access_id.to_le_bytes().as_ref()
    //     ],
    //     bump = digital_access.bump,
    // )]
    // pub digital_access: Account<'info, DigitalAccess>,
    #[account(
        mut,
        seeds = [SEED_TOKEN_MINT, collection_mint.key().as_ref(), token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_COLLECTION_MINT, event_id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            token_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn process_update_token_metadata(
    ctx: Context<UpdateTokenMetadata>,
    event_id: u64,
    _token_id: u64,
    uri: String,
) -> Result<()> {
    let acc = &ctx.accounts;
    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_COLLECTION_MINT,
        &event_id.to_le_bytes(),
        &[ctx.bumps.collection_mint],
    ]];

    metadata::update_metadata_accounts_v2(
        CpiContext::new_with_signer(
            acc.token_metadata_program.to_account_info(),
            metadata::UpdateMetadataAccountsV2 {
                metadata: acc.metadata_account.to_account_info(),
                update_authority: acc.collection_mint.to_account_info(),
            },
            signer_seeds,
        ),
        None,
        Some(DataV2 {
            name: acc.metadata_account.name.clone(),
            symbol: acc.metadata_account.symbol.clone(),
            uri,
            seller_fee_basis_points: acc.metadata_account.seller_fee_basis_points.clone(),
            creators: acc.metadata_account.creators.clone(),
            collection: acc.metadata_account.collection.clone(),
            uses: acc.metadata_account.uses.clone(),
        }),
        None,
        Some(true),
    )
}
