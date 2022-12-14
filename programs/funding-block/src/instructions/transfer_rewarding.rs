use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub use crate::state::*;

pub fn transfer_rewarding(ctx: Context<TransferRewarding>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct TransferRewarding<'info>{
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,

    #[account(mut)]
    pub program_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
}
