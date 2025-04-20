#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8KPrPekaN1ypQhTXfXLD9pk5jp4UoAnWtpP9jegMSNiR");

#[program]
pub mod staking {
    use super::*;

    pub fn initializeconfig(ctx: Context<InitializeConfig>) -> Result<()> {
        Ok(())
    }

    pub fn initializeuser(ctx: Context<InitializUser>) -> Result<()> {
         Ok(())
    }
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        Ok(())
    }
}
