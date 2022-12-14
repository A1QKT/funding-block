use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("Gfaf2UDB7aYzbBcZGHtAzyR3BtMubkm5EBhxGdzgwcs");

#[program]
pub mod funding_block {

    use anchor_lang::system_program::Transfer;

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

    pub fn create_solution(
        ctx: Context<CreateSolution>, 
        quest_address: Pubkey, 
        user_address: Pubkey, 
        content: String, 
        image_link: String
    ) -> Result<()>{
        instructions::create_solution(
            ctx, 
            quest_address, 
            user_address, 
            content, 
            image_link
        )
    }

    pub fn fund_quest(ctx: Context<FundQuest>, fund: u64) -> Result<()> {
        instructions::fund_quest(ctx, fund)
    }

    pub fn vote(ctx: Context<Vote>, solution_address: Pubkey) -> Result<()> {
        instructions::vote(ctx, solution_address)
    }

    pub fn transfer_rewarding(ctx: Context<TransferRewarding>) -> Result<()> {
        instructions::transfer_rewarding(ctx)
    }
}