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
        title: String, 
        fund_amount: u64,
        time_end: u64,
    ) -> Result<()> {
        instructions::create_quest(
            ctx, 
            title,
            fund_amount,
            time_end,
        )
    }

    pub fn create_solution(
        ctx: Context<CreateSolution>, 
        content: String, 
        image_link: String
    ) -> Result<()>{
        instructions::create_solution(
            ctx, 
            content, 
            image_link
        )
    }

    pub fn fund_quest(ctx: Context<FundQuest>, fund: u64) -> Result<()> {
        instructions::fund_quest(ctx, fund)
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        instructions::vote(ctx)
    }

    pub fn transfer_rewarding(ctx: Context<TransferRewarding>) -> Result<()> {
        instructions::transfer_rewarding(ctx)
    }
}