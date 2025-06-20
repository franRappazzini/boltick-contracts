use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

use crate::{constants::SEED_CONFIG, states::Config};

pub fn withdraw_spl<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    authority: &Account<'info, Config>,
    amount: u64,
    mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
    bump: u8,
) -> Result<()> {
    // from vault to depositor_bolt_account

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_CONFIG, &[bump]]];

    let accounts = token_interface::TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
        mint: mint.to_account_info(),
    };

    let cpi_ctx =
        CpiContext::new_with_signer(token_program.to_account_info(), accounts, signer_seeds);

    token_interface::transfer_checked(cpi_ctx, amount, mint.decimals)
}
