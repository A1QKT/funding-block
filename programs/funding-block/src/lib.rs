use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("9RgWo49pJ9pD24QkBMFTJ1J6RQdHbLoTa4J65V3n8eKm");

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

    pub fn update_solution (
        ctx: Context<UpdateSolution>,
        content: String,
        image_link: String
    ) -> Result<()> {
        instructions::update_solution(ctx, content, image_link)
    }

    pub fn fund_quest(ctx: Context<FundQuest>, fund_amount: u64) -> Result<()> {
        instructions::fund_quest(ctx, fund_amount)
    }

    pub fn join_fund(ctx: Context<JoinFund>, fund_amount: u64) -> Result<()> {
        instructions::join_fund(ctx, fund_amount)
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        instructions::vote(ctx)
    }

    // pub fn un_vote(ctx: Context<Vote>) -> Result<()> {
    //     instructions::un_vote(ctx)
    // }

    pub fn transfer_rewarding(
        ctx: Context<TransferBack>,
        fund_amount: u64,
        close_status: String
    ) -> Result<()> {
        instructions::transfer_rewarding(ctx, fund_amount, close_status)
    }

    pub fn close_quest(ctx: Context<CloseQuest>) -> Result<()> {
        instructions::close_quest(ctx)
    }

    pub fn send_fund_back(ctx: Context<TransferBack>, fund_amount: u64) -> Result<()> {
        instructions::send_fund_back(ctx, fund_amount)
    }
}