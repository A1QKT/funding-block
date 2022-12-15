use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::FundingBlockError;

pub fn create_solution(
    ctx: Context<CreateSolution>,
    content: String, 
    image_link: String) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let solution_account = &mut ctx.accounts.solution_account;
    let user = &mut ctx.accounts.user;

    //Check contraint
     if content.as_bytes().len() > 400 || image_link.as_bytes().len() > 200 {
        return err!(FundingBlockError::InvalidLength);
    }

    quest_account.num_solver = quest_account.num_solver + 1;

    solution_account.quest_address = quest_account.key();
    solution_account.user_address = user.key();
    solution_account.num_vote = 0;
    solution_account.image_link = image_link;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateSolution<'info> {
    #[account(mut)]
    quest_account: Account<'info, Quest>,

    #[account(
        init,
        payer = user,
        seeds = [
            b"solution_account",
            user.key().as_ref(),
            quest_account.key().as_ref()
        ],
        bump,
        space = 32 + 32 + 400 + 8 + 200 + 8
    )]
    solution_account: Account<'info, Solution>,

    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}