use anchor_lang::prelude::*;

pub use crate::state::*;

pub fn transfer_rewarding(ctx: Context<TransferRewarding>) {}

#[derive(Accounts)]
pub struct TransferRewarding<'info>{
    #[account(mut)]
    pub quest_accont: Account<'info, Quest> 
}
