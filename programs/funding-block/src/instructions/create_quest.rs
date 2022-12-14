use anchor_lang::prelude::*;

use crate::state::{Quest, FunderState, Pool};
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

    Ok(())
}

#[derive(Accounts)]
pub struct CreateQuest<'info> {
    #[account(
        init, 
        payer = user, 
        space = Quest::MAX_SIZE + 8
    )]
    pub quest_account: Account<'info, Quest>,

    #[account(
        init,
        payer = user,
        seeds = [
            b"funder_state",
            user.key().as_ref(), 
            quest_account.key().as_ref(),
        ],
        bump,
        space = 32 + 8 + 1 + 8
    )]
    pub funder_state: Account<'info, FunderState>,

    #[account(
        init,
        seeds = [],
        bump,
        payer = user,
        space = 32 + 8 + 1 + 1 + 8
    )]
    pool: Account<'info, Pool>,

    #[account(mut)]
    pub user: Signer<'info>,
    // programe chuyen tien
    // sysvar
    pub system_program: Program<'info, System>
}
