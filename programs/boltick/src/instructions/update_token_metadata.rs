use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        self,
        mpl_token_metadata::types::{Collection, Creator, DataV2},
        Metadata,
    },
    token_interface::Mint,
};

use crate::{Config, Event, SEED_COLLECTION_MINT, SEED_CONFIG, SEED_EVENT, SEED_TOKEN_MINT};

#[derive(Accounts)]
#[instruction(event_id: u64, token_id: u64)]
pub struct UpdateTokenMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [SEED_EVENT, event_id.to_le_bytes().as_ref()],
        bump,
        constraint = event.collection_mint_account == collection_mint.key()
    )]
    pub event: Account<'info, Event>,

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

    /// CHECK: Validate address by deriving pda
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
    pub metadata_account: UncheckedAccount<'info>,

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
    token_id: u64,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let acc = &ctx.accounts;
    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_COLLECTION_MINT,
        &event_id.to_le_bytes(),
        &[ctx.bumps.collection_mint],
    ]];

    // TODO (fran): check how to get metadata onchain
    // let data = ctx.accounts.metadata_account.data.try_borrow().unwrap();
    // let metadata_acc = MPLMetadata::safe_deserialize(data.as_ref())?;

    metadata::update_metadata_accounts_v2(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            metadata::UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.collection_mint.to_account_info(),
            },
            signer_seeds,
        ),
        None,
        Some(DataV2 {
            name: format!("{} #{}", name, token_id),
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: acc.collection_mint.key(),
                verified: true,
                share: 100,
            }]),
            collection: Some(Collection {
                key: acc.collection_mint.key(),
                verified: true,
            }),
            uses: None,
        }),
        None,
        Some(true),
    )
}
