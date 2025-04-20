use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct  StakeAccount {
    pub points: u32,
    pub amount_stake: u8,
    pub bump: u8
}