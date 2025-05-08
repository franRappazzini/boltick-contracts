use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{mpl_token_metadata::types::Collection, Metadata},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Config, Event, SEED_COLLECTION_MINT, SEED_CONFIG, SEED_EVENT, SEED_TOKEN_MINT};

use super::{create_master_edition, create_metadata_accounts, mint_to, set_and_verify_sized_collection_item};

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_EVENT, event_id.to_le_bytes().as_ref()],
        bump,
        constraint = event.collection_mint_account == collection_mint.key(),
    )]
    pub event: Account<'info, Event>,

    #[account(mut, address = event.creator)]
    pub event_creator: SystemAccount<'info>,

    // #[account(
    //     mut,
    //     seeds = [SEED_TREASURY],
    //     bump = config.treasury_bump
    // )]
    // pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = buyer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        mint::token_program = token_program,
        seeds = [
            SEED_TOKEN_MINT,
            collection_mint.key().as_ref(),
            event.current_nft_count.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer, 
        associated_token::token_program = token_program,
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

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

pub fn process_buy_token(
    ctx: Context<BuyToken>,
    event_id: u64,
    name: String,
    symbol: String,
    uri: String
) -> Result<()> {
    let acc = &ctx.accounts;

    // transfer SOL to event creator
    system_program::transfer(
        CpiContext::new(
            acc.system_program.to_account_info(),
            system_program::Transfer {
                from: acc.buyer.to_account_info(),
                // to: acc.treasury.to_account_info()
                to: acc.event_creator.to_account_info()
            }
        ),
        acc.event.ticket_price
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_COLLECTION_MINT,
        &event_id.to_le_bytes(),
        &[ctx.bumps.collection_mint]
    ]];

    mint_to(
        &acc.token_program,
        &acc.token_mint,
        &acc.buyer_token_account,
        &acc.collection_mint,
        signer_seeds
    )?;

    create_metadata_accounts(
        &acc.token_metadata_program,
        &acc.metadata_account,
        &acc.token_mint,
        &acc.collection_mint,
        &acc.buyer,
        &acc.system_program,
        &acc.rent,
        signer_seeds,
        name,
        symbol,
        uri,
        0,
        Some(Collection{ key: acc.collection_mint.key(), verified: false }),
        true,
        None
    )?;

    create_master_edition(
        &acc.token_metadata_program,
        &acc.edition_account,
        &acc.token_mint,
        &acc.collection_mint,
        &acc.buyer,
        &acc.metadata_account,
        &acc.token_program,
        &acc.system_program,
        &acc.rent,
        signer_seeds
    )?;

    set_and_verify_sized_collection_item(
        &acc.token_metadata_program,
        &acc.metadata_account,
        &acc.buyer,
        &acc.collection_mint,
        &acc.collection_metadata_account,
        &acc.collection_master_edition,
        signer_seeds
    )?;

    ctx.accounts.event.current_nft_count += 1;

    Ok(())
}
