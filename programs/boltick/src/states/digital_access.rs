use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DigitalAccess {
    pub event: Pubkey,
    pub id: u8,
    pub price: u64,
    // pub used: bool,
    pub max_supply: u64,
    pub current_minted: u64,
    #[max_len(32)]
    pub name: String,
    #[max_len(8)]
    pub symbol: String,
    #[max_len(240)]
    pub description: String,
    #[max_len(160)]
    pub uri: String,
    // #[max_len(160)]
    // pub uri_updated: String,
}
