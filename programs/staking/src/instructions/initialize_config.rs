use anchor_lang::prelude::*;
use anchor_spl::token::{Mint,Token};


use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        mut
    )]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE,
        seeds = [&b"config"[..]],
        bump
    )]
    pub config: Account<'info,StakeConfig>,
    #[account(
        mint::token_program = token_program,
        seeds = [b"reward",config.key().as_ref()],
        bump
    )]
    pub reward_mint: Account<'info,Mint>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self,points_per_stake: u64,max_stake: u64,freeze_period: i64,bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(StakeConfig {
            rewards_bump: bumps.config,
            bump: bumps.reward_mint,
            points_per_stake,
            freeze_period,
            max_stake
        });

        Ok(())
    }
}

