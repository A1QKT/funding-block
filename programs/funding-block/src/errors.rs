use anchor_lang::error_code;

#[error_code]
pub enum FundingBlockError {
    #[msg("QUEST.ENDED_QUEST")]
    EndedQuest,
    #[msg("QUEST.INVALID_LENGTH")]
    InvalidLength,
    #[msg("QUEST.INVALID_TIME_STAMP")]
    InvalidTimeStamp,
    #[msg("TRANSFER.TRANSFER_FAIL")]
    TransferFail,
    #[msg("VOTE.FUNDER_HAS_VOTED")]
    FunderVoted,
    #[msg("VOTE.FUNDER_HAS_NOT_VOTED")]
    FunderNotVoted,
    #[msg("VOTE.INVALID_ACTION")]
    InvalidActionVote,
    #[msg("TRANSFER_BACK.INVALID")]
    InvalidTransferBack
}