use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer}; not needed yet

// this is the id of the program that we will deploy to the blockchain.
// doesnt matter what this is when you build and then when you deploy
// this will be updated with the new id of the program.
declare_id!("69Qd1B33Uo7PR2JzfC7finFDaccts85pdpoCSMYbNf8K");

// the program is the 'smart contract' that will be deployed to the blockchain.
// think of this as the 'backend' of the application.
// the public functions are whats called 'instructions' and are the only way to interact with the program.
// You can think of them as handlers for the different types of transactions that can be sent to the program.
#[program]
pub mod staking_pool_manager {
    // since modules dont have access to the rest of the scope we import it all here.
    use super::*;

    // unlike ethereum, when I want to create a new pool I am not creating a new program but just a new account that holds data.
    pub fn create_pool(ctx: Context<CreatePool>, id: u64, token: Pubkey) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.id = id;
        pool.creator = *ctx.accounts.authority.key;
        pool.asset = token;
        emit!(PoolCreated {
            id: pool.id,
            creator: pool.creator,
            asset: pool.asset,
        });

        Ok(())
    }
}

/**
* Program Derived Accounts (PDAs) are derived from the program id and some seed.
  They dont have private keys and are used to store data.
  We make a struct that represents the data we want to store in the account
  and this is the context passed to the instruction.
*/
#[derive(Accounts)]
#[instruction(id : u64)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"pool", id.to_le_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + Pool::MAX_SIZE,
    )]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>,
}

/**
* Here are the accounts that will be created.
*/
#[account]
#[derive(Default)]
pub struct Pool {
    pub id: u64,
    pub creator: Pubkey,
    pub asset: Pubkey,
}

// impl are just methods that are attached to a struct.
impl Pool {
    // we need to size the account so that we can store the data.
    // 8 bytes for the id, 32 bytes for the creator, 32 bytes for the asset.
    pub const MAX_SIZE: usize = 8 + 32 + 32;
}

// Events are not built into solana yet but Anchor has a nice way of creating them.
#[event]
pub struct PoolCreated {
    pub id: u64,
    pub creator: Pubkey,
    pub asset: Pubkey,
}
