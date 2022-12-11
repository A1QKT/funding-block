use anchor_lang::error_code;

#[error_code]
pub enum FundingBlockError {
    #[msg("QUEST.PROCESSING_VOTING")]
    ProcessingVoting,
    #[msg("QUEST.INVALID_LENGTH")]
    InvalidLength,
}