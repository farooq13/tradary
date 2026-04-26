use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::TradaryError;
use crate::state::UserAccount;



#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdateUserProfileParams {
    pub username: Option<String>,
    pub bio: Option<String>,
    pub privacy_enabled: Option<bool>,
}


// Context
#[derive(Accounts)]
pub struct UpdateUserProfile<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_USER, owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner @ TradaryError::Unauthorized
    )]
    pub user_account: Account<'info, UserAccount>,

}


// Handler
pub fn handler(ctx: Context<UpdateUserProfile>, params: UpdateUserProfileParams) -> Result<()> {
    let user = &mut ctx.accounts.user_account;

    if let Some(username) = params.username {
        require!(username.len() <= MAX_USERNAME_LEN, TradaryError::UsernameTooLong);
        user.username = username;
    }

    if let Some(bio) = params.bio {
        require!(bio.len() <= MAX_BIO_LEN, TradaryError::BioTooLong);
        user.bio = bio;
    }

    if let Some(privacy_enabled) = params.privacy_enabled {
        user.privacy_enabled = privacy_enabled;
    }

    Ok(())
}
