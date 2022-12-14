pub use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub quest_account: String,
}