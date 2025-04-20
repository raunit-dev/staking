use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked, TransferChecked},
};
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
    pub collection_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub token_program: Account<'info,UserAccount>,
    pub metadata_program: Account<'info,Metadata>,
    pub system_program: Program<'info,System>
}

impl <'info> Stake <'info> {
    
}