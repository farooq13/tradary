use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::TradaryError;
use crate::state::{UserAccount, TradingStats};



#[derive(Accounts)]
#[instruction(username: String, bio: String)]
pub struct InitializeUser {
    /// The wallet creating the journal
    #[account(mut)]
    pub owner: Signer<'info>,

    /// UserAccount PDA - created here, one per owner
    #[account(
        init,
        payer = owner,
        space = UserAccount::LEN,
        seeds = [SEED_USER, owner.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

// ------------- Handler -----------
// initialize a new user journal profile
pub fn handler(
    ctx: Context<InitializeUser>,
    username: String,
    bio: String,
    privacy_enabled: bool,
) -> Result<()> {
    // Input validation
    require!(username.len() <= MAX_USERNAME_LEN, TradaryError::UsernameTooLong);
    require!(bio.len() <= MAX_BIO_LEN, TradaryError::BioTooLong);

    let user = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    user.version = CURRENT_VERSION;
    user.owner = ctx.accounts.owner.key();
    user.bump = ctx.bumps.user_account;
    user.username = username;
    user.bio = bio;
    user.created_at = clock.unix_timestamp;
    user.trade_account = 0;
    user.tag_count = 0;
    user.stats = TradingStats::default();
    user.privacy_enabled = privacy_enabled;
    user._reserved = [0u8; 32];
    

    emit!(UserInitialized {
        owner: ctx.accounts.owner.key(),
        created_at: clock.unix_timestamp,
    });

    Ok(())
}


// Events
#[events]
pub struct UserInitialized {
    pub owner: Pubkey,
    pub created_at: i64,
}