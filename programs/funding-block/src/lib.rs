use anchor_lang::prelude::*;
use context::*;
use account::*;

mod account;
mod context;

declare_id!("Gfaf2UDB7aYzbBcZGHtAzyR3BtMubkm5EBhxGdzgwcs");

#[program]
pub mod funding_block {
    use super::*;

    pub fn initialize_quest(ctx: Context<InitializeQuest>) -> Result<()> {
        
        Ok(())
    }

    pub fn fund_quest(ctx: Context<CreateQuest>) -> Result<()> {
        Ok(())
    }

    pub fn create_room(ctx: Context<CreateRoom>) -> Result<()> {
        Ok(())
    } 

    pub fn voting(ctx: Context<Voting>) -> Result<()> {
        Ok(())
    }
}