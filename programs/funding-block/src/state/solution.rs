use anchor_lang::prelude::*;

#[account]
pub struct Solution {
    pub quest_address: Pubkey, // 32
    pub user_address: Pubkey, // 32
    pub content: String, // 400
    pub num_vote: u64, // 8
    pub image_link: String, // 200
}