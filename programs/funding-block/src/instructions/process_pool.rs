use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount, transfer, Transfer}
};

pub use crate::state::*;
pub use crate::errors::FundingBlockError;

pub fn transfer_rewarding(
    ctx: Context<TransferBack>, 
    fund_amount: u64, 
    close_status: String
) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    
    let wallet = &mut ctx.accounts.wallet;
    let wallet_token = &mut ctx.accounts.wallet_token;
    let user_wallet = &mut ctx.accounts.user_wallet;

    let token_program = &ctx.accounts.token_program;
 
    let _transfer = match transfer(CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: wallet_token.to_account_info(),
                to: user_wallet.to_account_info(),
                authority: wallet.to_account_info(),
            },
        ), fund_amount) {
        Ok(()) => Ok(()),
        Err(_e) => err!(FundingBlockError::TransferFail),
    };

    quest_account.fund = quest_account.fund - fund_amount;
    quest_account.closed = close_status;

    msg!("CREATE_QUEST.SUCCESS");
    
    Ok(())
}

pub fn close_quest(ctx: Context<CloseQuest>) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;
    quest_account.closed = String::from("TRUE");

    Ok(())
}

pub fn send_fund_back(ctx: Context<TransferBack>, fund_amount: u64) -> Result<()> {
    let quest_account = &mut ctx.accounts.quest_account;

    if quest_account.closed != String::from("TRUE") {
        return err!(FundingBlockError::InvalidTransferBack)
    }
    
    let wallet = &mut ctx.accounts.wallet;
    let wallet_token = &mut ctx.accounts.wallet_token;
    let user_wallet = &mut ctx.accounts.user_wallet;

    let token_program = &ctx.accounts.token_program;
 
    let _transfer = match transfer(CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: wallet_token.to_account_info(),
                to: user_wallet.to_account_info(),
                authority: wallet.to_account_info(),
            },
        ), fund_amount) {
        Ok(()) => Ok(()),
        Err(_e) => err!(FundingBlockError::TransferFail),
    };

    quest_account.fund = quest_account.fund - fund_amount;

    msg!("CREATE_QUEST.SUCCESS");

    Ok(())
}

#[derive(Accounts)]
pub struct TransferBack<'info>{
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,

    #[account(mut)]
    user_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(mut)]
    pub wallet_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CloseQuest<'info>{
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
}
