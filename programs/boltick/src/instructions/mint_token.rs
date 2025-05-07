use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{self, mpl_token_metadata::types::Collection, Metadata},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Config, Event, SEED_COLLECTION_MINT, SEED_CONFIG, SEED_EVENT, SEED_TOKEN_MINT};

use super::{create_master_edition, create_metadata_accounts, mint_to};

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub destination: SystemAccount<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_EVENT, event_id.to_le_bytes().as_ref()],
        bump,
        constraint = event.collection_mint_account == collection_mint.key()
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [SEED_COLLECTION_MINT, event_id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        mint::token_program = token_program,
        seeds = [SEED_TOKEN_MINT, collection_mint.key().as_ref(), event.current_nft_count.to_le_bytes().as_ref()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = destination,
        associated_token::token_program = token_program
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,

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
    pub collection_metadata_account: UncheckedAccount<'info>,

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
    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_mint_token(
    ctx: Context<MintToken>,
    event_id: u64,
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

    mint_to(
        &acc.token_program,
        &acc.token_mint,
        &acc.destination_token_account,
        &acc.collection_mint,
        signer_seeds,
    )?;

    // TODO (fran): get from collection_metadata_account [?]
    let name = format!("{} #{}", name, acc.event.current_nft_count);

    create_metadata_accounts(
        &acc.token_metadata_program,
        &acc.metadata_account,
        &acc.token_mint,
        &acc.collection_mint,
        &acc.authority,
        &acc.system_program,
        &acc.rent,
        signer_seeds,
        name,
        symbol,
        uri,
        0,
        Some(Collection {
            key: acc.collection_mint.key(),
            verified: false, // will be verified then (set_and_verify_sized_collection_item)
        }),
        true,
        None,
    )?;

    create_master_edition(
        &acc.token_metadata_program,
        &acc.edition_account,
        &acc.token_mint,
        &acc.collection_mint,
        &acc.authority,
        &acc.metadata_account,
        &acc.token_program,
        &acc.system_program,
        &acc.rent,
        signer_seeds,
    )?;

    metadata::set_and_verify_sized_collection_item(
        CpiContext::new_with_signer(
            acc.token_metadata_program.to_account_info(),
            metadata::SetAndVerifySizedCollectionItem {
                metadata: acc.metadata_account.to_account_info(),
                collection_authority: acc.collection_mint.to_account_info(),
                payer: acc.authority.to_account_info(),
                update_authority: acc.collection_mint.to_account_info(),
                collection_mint: acc.collection_mint.to_account_info(),
                collection_metadata: acc.collection_metadata_account.to_account_info(),
                collection_master_edition: acc.collection_master_edition.to_account_info(),
            },
            signer_seeds,
        ),
        None,
    )?;

    ctx.accounts.event.current_nft_count += 1;

    Ok(())
}
