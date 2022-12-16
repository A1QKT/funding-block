use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount, transfer, Transfer}
};

use crate::errors::FundingBlockError;

use crate::state::quest::*;

pub fn fund_quest(ctx: Context<FundQuest>, fund_amount: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let funder_state = &mut ctx.accounts.funder_state;
    
    let user = &mut ctx.accounts.user;
    let user_token = &mut ctx.accounts.user_token;
    let program_wallet = &mut ctx.accounts.program_wallet;

    let token_program = &ctx.accounts.token_program;

    if quest_account.closed == String::from("TRUE") || quest_account.closed == String::from("PARTIAL") {
        return err!(FundingBlockError::InvalidTimeStamp)
    }

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

    quest_account.fund = quest_account.fund + fund_amount;
    funder_state.fund = funder_state.fund + fund_amount;
    
    Ok(())
}

pub fn join_fund(ctx: Context<JoinFund>, fund_amount: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    let funder_state = &mut ctx.accounts.funder_state;
    
    let user = &mut ctx.accounts.user;
    let user_token = &mut ctx.accounts.user_token;
    let program_wallet = &mut ctx.accounts.program_wallet;

    let token_program = &ctx.accounts.token_program;

    if quest_account.closed == String::from("TRUE") || quest_account.closed == String::from("PARTIAL") {
        return err!(FundingBlockError::InvalidTimeStamp)
    }
 
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

    quest_account.fund = quest_account.fund + fund_amount;
    quest_account.num_funder = quest_account.num_funder + 1;

    funder_state.quest_address = quest_account.key();
    funder_state.fund = fund_amount;
    funder_state.vote = false;

    msg!("CREATE_QUEST.SUCCESS");

    Ok(())
}

#[derive(Accounts)]
pub struct FundQuest<'info> {
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,

    #[account(
        mut,
        seeds = [
            b"funder_state",
            user.key().as_ref(),
            &quest_account.key().to_bytes(),
        ],
        bump,
    )]
    pub funder_state: Account<'info, FunderState>,

    #[account(mut)]
    program_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct JoinFund<'info> {
    #[account(mut)]
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
        space = 32 + 32 + 8 + 1 + 8
    )]
    pub funder_state: Account<'info, FunderState>,

    #[account(mut)]
    program_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

