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

    pub fn fund_quest_(ctx: Context<FundQuest>, fund: u64) -> Result<()> {
        instructions::fund_quest(ctx, fund)
    }   

    pub fn vote(ctx: Context<Vote>, vote_up: bool) -> Result<()> {
        instructions::vote(ctx, vote_up)
    }

    pub fn transfer_rewarding(ctx: Context<Reward>) -> Result<()> {
        instructions::execute_rewarding(ctx)
    }
}