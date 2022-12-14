use anchor_lang::prelude::*;

use crate::state::*;

pub fn create_solution(
    ctx: Context<CreateSolution>,
    quest_address: Pubkey, 
    user_address: Pubkey, 
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

        ],
        bump,
        space = 32 + 32 + 400 + 8 + 200 + 8
    )]
    solution_account: Account<'info, Solution>,
    
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}