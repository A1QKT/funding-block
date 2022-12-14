use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("Gfaf2UDB7aYzbBcZGHtAzyR3BtMubkm5EBhxGdzgwcs");

#[program]
pub mod funding_block {

    use super::*;

    pub fn create_quest(
        ctx: Context<CreateQuest>, 
        name: String, 
        fund_amount: u64,
        time_end: u64,
        quest_id: u8
    ) -> Result<()> {
        instructions::create_quest(ctx, name,fund_amount, time_end, quest_id)
    }

    pub fn join_quest_investor(ctx: Context<JoinQuest>, fund: u64) -> Result<()> {
        instructions::join_quest_investor(ctx, fund)
    }

    pub fn join_quest_solver(ctx: Context<JoinQuest>, quest_id: String) -> Result<()> {
        instructions::join_quest_solver(ctx)
    }

    pub fn trigger_voting(ctx: Context<TriggerVoting>, time_voting_end: u64) -> Result<()> {
        instructions::trigger_voting(ctx, time_voting_end)
    }

    pub fn vote(ctx: Context<Vote>, vote_up: bool) -> Result<()> {
        instructions::vote(ctx, vote_up)
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        instructions::donate(ctx, amount)
    }

    pub fn execute_rewarding(ctx: Context<Reward>) -> Result<()> {
        instructions::execute_rewarding(ctx)
    }

    pub fn clean_pda(ctx: Context<CleanPDA>) -> Result<()> {
        instructions::clean_pda(ctx)
    }
}