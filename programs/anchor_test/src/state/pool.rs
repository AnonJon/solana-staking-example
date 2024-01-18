use anchor_lang::prelude::*;

/**
* Here are the accounts that will be created.
*/
#[account]
#[derive(Default)]
pub struct Pool {
    pub id: u64,
    pub creator: Pubkey,
    pub asset: Pubkey,
    pub is_closed: bool,
    pub is_frozen: bool,
}

// impl are just methods that are attached to a struct.
impl Pool {
    // we need to size the account so that we can store the data.
    // 8 bytes for the id, 32 bytes for the creator, 32 bytes for the asset.
    pub const MAX_SIZE: usize = 8 + 32 + 32;
}
