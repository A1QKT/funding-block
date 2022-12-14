use anchor_lang::{prelude::*, solana_program::clock};

use crate::state::quest::*;
use crate::errors::FundingBlockError;

pub fn vote(ctx: Context<Vote>, vote_up: bool) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    quest_account: Account<'info, Quest>,
    #[account(mut)]
    investor_account: Account<'info, MemberState>
}

#[event]
pub struct EventTrigger {
    trigger_address: Pubkey,
    quest_address: Pubkey   
}

