use anchor_lang::prelude::*;

#[account]
pub struct Quest{
    pub time_start: u64,
    pub time_end: u64,
    pub num_investor: u64,
    pub num_solver: u64,
    pub is_voting: bool,
    pub fund: u64,
    pub title: String,
}

impl Quest {
    pub const MIN_FUND: u64 = 1;
    pub const MAX_SIZE: usize = 16 + 16 + 8 + 8 + 1 + 8 + 20;
}

#[account]
pub struct MemberState {
    pub quest_address: Pubkey,
    pub fund: u64,
    pub trigger_voting: bool,
    pub is_investor: bool,
    pub bump: u8,
}