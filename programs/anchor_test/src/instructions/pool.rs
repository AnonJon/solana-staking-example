use crate::errors::Error;
use crate::state::events::{PoolCreated, PoolDeposit};
use crate::state::{pool::Pool, state::State};
use anchor_lang::{prelude::*, solana_program::program_pack::Pack};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use spl_token::state::Account as TokenAccountData;

pub fn handle_create_pool(ctx: Context<CreatePool>, token: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let pool = &mut ctx.accounts.pool;
    pool.id = state.next_pool_id;
    pool.creator = *ctx.accounts.authority.key;
    pool.asset = token;
    pool.is_frozen = false;
    pool.is_closed = false;
    state.next_pool_id += 1;
    emit!(PoolCreated {
        id: pool.id,
        creator: pool.creator,
        asset: pool.asset,
    });

    Ok(())
}

pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Validate deposit requirements
    ctx.accounts.validate_deposit()?;

    // 1. Deposit token from user ATA to the vault ATA
    ctx.accounts.deposit_to_pool(amount)?;

    // 2. Mint tokens to the user's pool ATA
    ctx.accounts
        .mint_lp_tokens_to_user(ctx.program_id, amount)?;

    emit!(PoolDeposit {
        amount: amount,
        depositor: ctx.accounts.signer.key(),
        pool_id: ctx.accounts.pool.id,
    });
    Ok(())
}

/**
* Program Derived Accounts (PDAs) are derived from the program id and some seed.
  They dont have private keys and are used to store data.
  We make a struct that represents the data we want to store in the account
  and this is the context passed to the instruction.
*/
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"pool", state.next_pool_id.to_le_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + Pool::MAX_SIZE,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    // This is the pool's ATA for the token they are depositing
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    // This is the user's ATA for the token they are depositing
    #[account(mut, constraint = user_token_account.owner == signer.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub user_pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK: The `mint_authority` is a PDA derived with known seeds and is used
    /// as the mint authority for the token. We ensure it matches the derived address
    /// and is the correct authority for minting tokens.
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: AccountInfo<'info>,

    // this is needed to create the associated token account if not already created
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl Deposit<'_> {
    fn validate_deposit(&self) -> Result<()> {
        require!(
            self.user_token_account.owner == self.signer.key(),
            Error::InvalidTokenAccountOwnerError
        );
        require!(!self.pool.is_frozen, Error::PoolFrozenError);
        require!(!self.pool.is_closed, Error::PoolClosedError);

        if self.user_pool_token_account.to_account_info().data_len() > 0 {
            let ata_data = TokenAccountData::unpack(
                &self.user_pool_token_account.to_account_info().data.borrow(),
            )?;
            // Ensure the ATA is for the correct mint
            require!(
                ata_data.mint == self.mint.key(),
                Error::InvalidAssociatedTokenAccount
            );
        }

        Ok(())
    }
    fn deposit_to_pool(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.pool_token_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
    fn mint_lp_tokens_to_user(&self, program_id: &Pubkey, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_pool_token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        let (_, bump) = Pubkey::find_program_address(&[b"mint_authority"], program_id);
        let seeds: &[&[u8]; 2] = &[b"mint_authority", &[bump][..]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
