use anchor_lang::prelude::*;
use crate::errors::FundingBlockError;

use crate::state::quest::*;

pub fn join_quest_solver(ctx: Context<JoinQuest>) -> Result<()>{
    let quest_account = &mut ctx.accounts.quest_account;
    if quest_account.is_voting {
        return err!(FundingBlockError::ProcessingVoting);
    }
    quest_account.num_solver = quest_account.num_solver + 1;

    let member_state = &mut ctx.accounts.member_state;
    member_state.quest_address = quest_account.key();
    member_state.fund = 0;
    member_state.trigger_voting = false;
    member_state.is_investor = false;
    member_state.bump = *ctx.bumps.get("member_state").unwrap();

    Ok(())
}

pub fn join_quest_investor(ctx: Context<JoinQuest>, fund: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let member_state = &mut ctx.accounts.member_state;

    if quest_account.is_voting {
        return err!(FundingBlockError::ProcessingVoting);
    }
    
    quest_account.num_investor = quest_account.num_investor + 1;
    quest_account.fund = quest_account.fund + fund;

    member_state.quest_address = quest_account.key();
    member_state.fund = fund;
    member_state.trigger_voting = false;
    member_state.is_investor = true;
    member_state.bump = *ctx.bumps.get("member_state").unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(quest_id: String)]
pub struct JoinQuest<'info> {
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(
        init,
        payer = user,
        seeds = [
            b"investor_quest",
            user.key().as_ref(), 
            quest_id.as_ref()
        ],
        bump,
        space = 32 + 8 + 1 + 8
    )]
    pub member_state: Account<'info, MemberState>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}