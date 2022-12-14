use anchor_lang::error_code;

#[error_code]
pub enum FundingBlockError {
    #[msg("QUEST.ENDED_QUEST")]
    EndedQuest,
    #[msg("QUEST.INVALID_LENGTH")]
    InvalidLength,
    #[msg("VOTING.INVALID_TIMESTAMP")]
    InvalidTimeStamp,
    #[msg("TRANSFER.TRANSFER_FAIL")]
    TransferFail,
}