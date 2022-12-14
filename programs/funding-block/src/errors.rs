use anchor_lang::error_code;

#[error_code]
pub enum FundingBlockError {
    #[msg("QUEST.PROCESSING_VOTING")]
    ProcessingVoting,
    #[msg("QUEST.ENDED_QUEST")]
    EndedQuest,
    #[msg("QUEST.INVALID_LENGTH")]
    InvalidLength,
    #[msg("VOTING.INVALID_USER_TYPE")]
    InvalidUserType,
    #[msg("VOTING.INVALIDE_TIMESTAMP")]
    InvalidTimeStamp,
    #[msg("WALLET.INSUFFICIENT_BALANCE")]
    InSufficientBalance,
    #[msg("PDA.INVALID_ACTION")]
    PDAInvalidAction
}