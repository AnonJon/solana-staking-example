use anchor_lang::prelude::*;

/**
* Errors
* https://book.anchor-lang.com/anchor_in_depth/errors.html
*/
#[error_code]
pub enum Error {
    #[msg("Signer is not the owner of the token account")]
    InvalidTokenAccountOwnerError,
    #[msg("Invalid associated token account")]
    InvalidAssociatedTokenAccount,
    #[msg("State already initialized")]
    StateAlreadyInitialized,
    #[msg("Pool is closed")]
    PoolClosedError,
    #[msg("Pool is frozen")]
    PoolFrozenError,
}
