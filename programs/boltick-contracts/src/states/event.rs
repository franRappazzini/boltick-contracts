use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub creator: Pubkey,
    pub collection_mint_account: Pubkey,
    pub date: i64,
    #[max_len(24)]
    pub name: String,
    #[max_len(80)]
    pub description: String,
    // TODO (fran): check more properties
}
