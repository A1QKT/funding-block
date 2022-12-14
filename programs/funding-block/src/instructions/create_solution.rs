use anchor_lang::prelude::*;

use crate::state::*;

pub fn create_solution(
    ctx: Context<CreateSolution>,
    content: String, 
    image_link: String) -> Result<()>{
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
            b"solution",
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