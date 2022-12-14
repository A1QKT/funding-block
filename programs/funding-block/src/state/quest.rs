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
pub struct MemberState {
    pub quest_address: Pubkey,
    pub fund: u64,
    pub trigger_voting: bool,
    pub is_investor: bool,
    pub bump: u8,
}