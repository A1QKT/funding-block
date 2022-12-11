use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("Gfaf2UDB7aYzbBcZGHtAzyR3BtMubkm5EBhxGdzgwcs");

#[program]
pub mod funding_block {

    use super::*;

    pub fn create_quest(ctx: Context<CreateQuest>, name: String, fund_amount: u64,
        time_end: u64 ) -> Result<()> {
        instructions::create_quest(ctx, name,fund_amount, time_end)
    }

    pub fn join_quest_investor(ctx: Context<JoinQuest>, fund: u64) -> Result<()> {
        instructions::join_quest_investor(ctx, fund)
    }

    pub fn join_quest_solver(ctx: Context<JoinQuest>, quest_id: String) -> Result<()> {
        instructions::join_quest_solver(ctx)
    }

    // pub fn trigger_voting(ctx: Context<TriggerVoting>) -> Result<()> {

    // }

    // pub fn vote(ctx: Context<Vote>, solver: Pubkey, vote_up: bool) -> Result<()> {

    // }

    // pub fn execute_rewarding(ctx: Context<Rewarding>) -> Resulte<()> {

    // }
}