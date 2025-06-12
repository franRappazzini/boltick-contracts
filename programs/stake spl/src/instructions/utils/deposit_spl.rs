use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

pub fn deposit_spl<'info>(
    authority: &Signer<'info>,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let cpi_accounts = token_interface::TransferChecked {
        authority: authority.to_account_info(),
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

    token_interface::transfer_checked(cpi_ctx, amount, mint.decimals)
}
