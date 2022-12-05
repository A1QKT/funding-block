use anchor_lang::prelude::*;

#[account]
pub struct QuestState {
    title: String,
    body: String,
    amount_donated: i64,
    admin: Pubkey
}

#[account]
pub struct QuestRoom {
    name: String, 
    admin: Pubkey
}