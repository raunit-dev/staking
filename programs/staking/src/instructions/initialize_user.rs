use anchor_lang::prelude::*;


use crate::UserAccount;

#[derive(Accounts)]
pub struct InitializUser <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space  = InitSpace,
    seeds = [b"user"],
    bump
    )]
    pub user_account: Account<'info,UserAccount>,
    pub system_program: Program<'info,System>
}

impl<'info> InitializUser<'info> {
    pub fn initialize_user(
        &mut self,
        mint: Pubkey,
        staked_at: i64,
        bump: u8,
    ) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            owner: self.user.key(),
            mint,
            staked_at,
            bump,
        });
        Ok(())
    }
}
