use anchor_lang::prelude::*;

use crate::state::quest::*;
use crate::errors::FundingBlockError;

pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let member_state = &mut ctx.accounts.member_state;

    

    Ok(())
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    quest_account: Account<'info, Quest>,
    #[account(
        seeds = [],
        bump
    )]
    member_state: Account<'info, MemberState>,
    user: Signer<'info>,
    system_program: Program<'info, System>
}
