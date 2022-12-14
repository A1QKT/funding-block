use anchor_lang::prelude::*;

pub use crate::state::*;

pub fn transfer_rewarding(ctx: Context<TransferRewarding>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct TransferRewarding<'info>{
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,

    #[account(
        mut,
        seeds = [],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub user: AccountInfo<'info>,
}
