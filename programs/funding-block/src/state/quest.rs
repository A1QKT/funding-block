use anchor_lang::prelude::*;

#[account]
pub struct Quest {
    pub title: String,
    pub time_start: u64,
    pub time_end: u64,
    pub num_funder: u64,
    pub num_solver: u64,
    pub fund: u64,
    pub closed: bool
}

impl Quest {
    pub const MIN_FUND: u64 = 1;
    pub const MAX_SIZE: usize = 200 + 8 + 8 + 8 + 8 + 8 + 1 + 8;
}

#[account]
pub struct FunderState {
    pub quest_address: Pubkey, // 32
    pub fund: u64, // 8
    pub vote: bool, // 1
    pub bump: u8, // 1
}