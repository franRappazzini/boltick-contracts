use anchor_lang::error_code;

#[error_code]
pub enum DappError {
    #[msg("The provided mint does not match the expected mint")]
    BoltMintMismatch,
    #[msg("The staking program is currently paused")]
    StakingPaused,
    #[msg("The provided amount is zero")]
    ZeroAmount,
    #[msg("The provided amount exceeds the maximum allowed limit")]
    AmountExceedsLimit,
    #[msg("Arithmetic overflow occurred during the operation")]
    ArithmeticOverflow,
    #[msg("Arithmetic underflow occurred during the operation")]
    ArithmeticUnderflow,
}
