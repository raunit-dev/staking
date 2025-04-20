use anchor_lang::prelude::*;


use crate::UserAccount;

#[derive(Accounts)]
pub struct InitializeUser <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space  = UserAccount::INIT_SPACE,
    seeds = [&b"user"[..]],
    bump
    )]
    pub user_account: Account<'info,UserAccount>,
    pub system_program: Program<'info,System>
}


    impl <'info> InitializUser<'info> {
        pub fn initialize_user_account(&mut self, bumps: &InitializeUserBumps)->Result<()>{
            self.user_account.set_inner(UserAccount { 
                points: 0, 
                amount_staked: 0, 
                bump: bumps.user_account,
            });
    
            Ok(())
        }
}
