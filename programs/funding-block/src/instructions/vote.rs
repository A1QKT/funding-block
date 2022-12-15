use anchor_lang::prelude::*;

use crate::state::{Quest, FunderState ,Solution};
use crate::errors::FundingBlockError;


pub fn vote(ctx: Context<Vote>) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let fund_state = &mut ctx.accounts.funder_state;
    let solution_account = &mut ctx.accounts.solution_account;

    if quest_account.closed != String::from("FALSE") {
        return err!(FundingBlockError::InvalidTimeStamp)
    }
 
    if fund_state.vote {
        return err!(FundingBlockError::FunderVoted)
    }

    solution_account.num_vote = solution_account.num_vote + 1;

    fund_state.vote = true;
    Ok(())
}

pub fn un_vote(ctx: Context<Vote>) -> Result<()> {
    let fund_state = &mut ctx.accounts.funder_state;
    let solution_account = &mut ctx.accounts.solution_account;

    if !fund_state.vote {
        return err!(FundingBlockError::FunderNotVoted)
    }

    if solution_account.num_vote < 1 {
        return err!(FundingBlockError::InvalidActionVote);
    }

    solution_account.num_vote = solution_account.num_vote - 1;

    fund_state.vote = true;

    Ok(())
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    quest_account: Account<'info, Quest>,
  
    #[account(
        mut, 
        seeds = [
            b"funder_state",
            user.key().as_ref(),
            &quest_account.key().to_bytes(),
        ],
        bump
    )]
    funder_state: Account<'info, FunderState>,

    #[account(
        mut,
        seeds = [
            b"solution_account",
            user.key().as_ref(),
            &quest_account.key().to_bytes(),
        ],
        bump
    )]
    solution_account: Account<'info, Solution>,

    user: Signer<'info>
}

