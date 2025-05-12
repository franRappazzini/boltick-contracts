use anchor_lang::error_code;

#[error_code]
pub enum DappError {
    #[msg("The signer authority is invalid")]
    InvalidAuthority,
    #[msg("The signer creator is invalid")]
    InvalidCreator,
    #[msg("The digital access has reached its max supply")]
    MaxSupplyReached,
    // #[msg("The event is not active")]
    // EventNotActive,
}
