use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake: u64,     
    pub max_stake: u64,            
    pub freeze_period: i64,        
    pub rewards_bump: u8,
    pub bump: u8,
}