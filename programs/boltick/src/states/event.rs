use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub creator: Pubkey,
    pub collection_mint_account: Pubkey,
    pub current_nft_count: u64,
    pub current_digital_access_count: u8,
    pub date: i64,
    #[max_len(24)]
    pub name: String,
    #[max_len(80)]
    pub description: String,
}
