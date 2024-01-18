use anchor_lang::prelude::*;

// Events are not built into solana yet but Anchor has a nice way of creating them.
#[event]
pub struct PoolCreated {
    pub id: u64,
    pub creator: Pubkey,
    pub asset: Pubkey,
}
#[event]
pub struct PoolDeposit {
    pub amount: u64,
    pub depositor: Pubkey,
    pub pool_id: u64,
}
