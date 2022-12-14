use anchor_lang::{
    prelude::*, 
    solana_program::{
        program::invoke,
        clock,
        system_instruction
    }
};

use crate::state::quest::*;
use crate::errors::FundingBlockError;

pub fn create_quest (
        ctx: Context<CreateQuest>,
        name: String,
        fund_amount: u64,
        time_end: u64,
        quest_id: u8,
    ) -> Result<()> {
    if name.as_bytes().len() > 200 || fund_amount < Quest::MIN_FUND {
        return err!(FundingBlockError::InvalidLength);
    }
    
    let quest_account = &mut ctx.accounts.quest_account;
    let time_start: i64 = clock::Clock::get()?.unix_timestamp.try_into().unwrap();
    quest_account.quest_id = quest_id;
    quest_account.time_start = time_start as u64;
    quest_account.title = name;
    quest_account.num_investor = 1;
    quest_account.num_solver = 0;
    quest_account.is_voting = false;
    quest_account.fund = fund_amount;
    quest_account.time_end = time_end;


    let member_state = &mut ctx.accounts.member_state;
    member_state.quest_address = quest_account.key();
    member_state.fund = fund_amount;
    member_state.trigger_voting = false;
    member_state.is_investor = true;
    member_state.bump = *ctx.bumps.get("member_state").unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    name: String,
    fund_amount: u64,
    time_end: u64,
    quest_id: u8,)]
pub struct CreateQuest<'info> {
    #[account(
        init, 
        payer = user, 
        seeds = [
            b"account_quest",
            user.key().as_ref(),
            &[quest_id]
        ],
        bump,
        space = Quest::MAX_SIZE + 8)]
    pub quest_account: Account<'info, Quest>,
    #[account(
        init,
        payer = user,
        seeds = [
            b"member_quest",
            user.key().as_ref(), 
            quest_account.key().as_ref()
        ],
        bump,
        space = 32 + 8 + 1 + 8)]
    pub member_state: Account<'info, MemberState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}
