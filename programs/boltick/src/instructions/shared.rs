use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        self,
        mpl_token_metadata::types::{Collection, CollectionDetails, Creator, DataV2},
        Metadata,
    },
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

pub fn mint_to<'info>(
    token_program: &Interface<'info, TokenInterface>,
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    authority: &InterfaceAccount<'info, Mint>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    token_interface::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token_interface::MintTo {
                mint: mint.to_account_info(),
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )
}

pub fn create_metadata_accounts<'info>(
    metadata_program: &Program<'info, Metadata>,
    metadata_account: &UncheckedAccount<'info>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &InterfaceAccount<'info, Mint>,
    payer: &Signer<'info>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    signer_seeds: &[&[&[u8]]],
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    collection: Option<Collection>,
    is_mutable: bool,
    collection_details: Option<CollectionDetails>,
) -> Result<()> {
    metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            metadata_program.to_account_info(),
            metadata::CreateMetadataAccountsV3 {
                metadata: metadata_account.to_account_info(),
                mint: mint.to_account_info(),
                mint_authority: authority.to_account_info(),
                payer: payer.to_account_info(),
                update_authority: authority.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points, // TODO (fran): CHECK
            creators: Some(vec![
                // TODO (fran): CHECK
                Creator {
                    address: authority.key(),
                    verified: true,
                    share: 100,
                },
            ]),
            collection,
            uses: None,
        },
        is_mutable,
        false,
        collection_details,
    )
}

pub fn create_master_edition<'info>(
    metadata_program: &Program<'info, Metadata>,
    edition_account: &UncheckedAccount<'info>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &InterfaceAccount<'info, Mint>,
    payer: &Signer<'info>,
    metadata_account: &UncheckedAccount<'info>,
    token_program: &Interface<'info, TokenInterface>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    metadata::create_master_edition_v3(
        CpiContext::new_with_signer(
            metadata_program.to_account_info(),
            metadata::CreateMasterEditionV3 {
                edition: edition_account.to_account_info(),
                mint: mint.to_account_info(),
                update_authority: authority.to_account_info(),
                mint_authority: authority.to_account_info(),
                payer: payer.to_account_info(),
                metadata: metadata_account.to_account_info(),
                token_program: token_program.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
            signer_seeds,
        ),
        Some(0),
    )
}

pub fn set_and_verify_sized_collection_item<'info>(
    metadata_program: &Program<'info, Metadata>,
    metadata_account: &UncheckedAccount<'info>,
    payer: &Signer<'info>,
    collection_mint: &InterfaceAccount<'info, Mint>,
    collection_metadata: &UncheckedAccount<'info>,
    collection_master_edition: &UncheckedAccount<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    metadata::set_and_verify_sized_collection_item(
        CpiContext::new_with_signer(
            metadata_program.to_account_info(),
            metadata::SetAndVerifySizedCollectionItem {
                metadata: metadata_account.to_account_info(),
                collection_authority: collection_mint.to_account_info(),
                payer: payer.to_account_info(),
                update_authority: collection_mint.to_account_info(),
                collection_mint: collection_mint.to_account_info(),
                collection_metadata: collection_metadata.to_account_info(),
                collection_master_edition: collection_master_edition.to_account_info(),
            },
            signer_seeds,
        ),
        None,
    )
}
