use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, Mint, TokenAccount, transfer, Transfer}
};

use crate::state::{Quest, FunderState};
use crate::errors::FundingBlockError;

pub fn create_quest (
        ctx: Context<CreateQuest>,
        title: String,
        fund_amount: u64,
        time_end: u64,
    ) -> Result<()> {
    msg!("con cac");
    if title.as_bytes().len() > 200 || fund_amount < Quest::MIN_FUND {
        return err!(FundingBlockError::InvalidLength);
    }
    let quest_account = &mut ctx.accounts.quest_account;
    let funder_state = &mut ctx.accounts.funder_state;
    
    let user = &mut ctx.accounts.user;
    let user_token = &mut ctx.accounts.user_token;
    let program_wallet = &mut ctx.accounts.program_wallet;
    let token_program = &ctx.accounts.token_program;
    
    let _transfer = match transfer(CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: user_token.to_account_info(),
                to: program_wallet.to_account_info(),
                authority: user.to_account_info(),
            },
        ), fund_amount) {
        Ok(()) => Ok(()),
        Err(_e) => err!(FundingBlockError::TransferFail),
    };

    quest_account.title = title;
    quest_account.time_end = time_end;
    quest_account.num_funder = 1;
    quest_account.num_solver = 0;
    quest_account.fund = fund_amount;
    quest_account.closed = false;

    funder_state.quest_address = quest_account.key();
    funder_state.fund = fund_amount;
    funder_state.vote = false;

    msg!("CREATE_QUEST.SUCCESS");

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
            &quest_account.key().to_bytes(),
        ],
        bump,
        space = 32 + 8 + 8
    )]
    pub funder_state: Account<'info, FunderState>,

    #[account(mut)]
    program_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    pub mint_account: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
