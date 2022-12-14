use anchor_lang::prelude::*;
use crate::errors::FundingBlockError;

use crate::state::quest::*;

pub fn fund_quest(ctx: Context<FundQuest>, fund: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let user = &mut ctx.accounts.user;

    //transfer(user, quest_aacount)

    Ok(())
}

#[derive(Accounts)]
pub struct FundQuest<'info> {
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(
        seeds = [
            b"funder_state",
            user.key().as_ref()
        ],
        bump
    )]
    pub fun_state: Account<'info, FunderState>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}