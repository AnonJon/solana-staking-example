use crate::instructions::*;
use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer}; not needed yet

pub mod errors;
pub mod instructions;
pub mod state;

// this is the id of the program that we will deploy to the blockchain.
// doesnt matter what this is when you build and then when you deploy
// this will be updated with the new id of the program.
declare_id!("FDXoehYLxVddHpw89CAdRQQwKvhHeymsa2Wyq7oXcTsm");

// the program is the 'smart contract' that will be deployed to the blockchain.
// think of this as the 'backend' of the application.
// the public functions are whats called 'instructions' and are the only way to interact with the program.
// You can think of them as handlers for the different types of transactions that can be sent to the program.
#[program]
pub mod staking_pool_manager {
    // since modules dont have access to the rest of the scope we import it all here.
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>) -> Result<()> {
        handle_initialize_state(ctx)
    }

    // unlike ethereum, when I want to create a new pool I am not creating a new program but just a new account that holds data.
    pub fn create_pool(ctx: Context<CreatePool>, token: Pubkey) -> Result<()> {
        handle_create_pool(ctx, token)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        handle_deposit(ctx, amount)
    }
}
