use std::fs::Metadata;

use anchor_lang::{prelude::*, solana_program::stake::state::Meta};

use crate::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut
    )]
    pub stake_account: Account<'info,StakeAccount>,
    #[account(
        mut
    )]
    pub user_account: Account<'info,StakeConfig>,
    pub token_program: Account<'info,UserAccount>,
    pub metadata_program: Account<'info,Metadata>,
    pub system_program: Account<'info,System>
}