use crate::account::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateQuest<'info> {
    #[account(init, payer = user, space = 8 )]
    pub create_quest: Account<'info, QuestState>,

    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateRoom {

}

#[derive(Accounts)]
pub struct FundQuest{

}

#[derive(Accounts)]
pub struct JoinQuest {

}

#[derive(Accounts)]
pub struct Voting {

}