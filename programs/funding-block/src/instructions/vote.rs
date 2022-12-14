use anchor_lang::prelude::*;

use crate::state::{Quest, FunderState ,Solution};
use crate::errors::FundingBlockError;

pub fn vote(ctx: Context<Vote>) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    quest_account: Account<'info, Quest>,
  
    #[account(
        mut, 
        seeds = [],
        bump
    )]
    funder_state: Account<'info, FunderState>,

    #[account(
        mut,
        seeds = [],
        bump
    )]
    solution_account: Account<'info, Solution>,

    user: Signer<'info>
}

