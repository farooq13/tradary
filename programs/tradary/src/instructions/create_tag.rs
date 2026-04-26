use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::TradaryError;
use crate::state::{ UserAccount, TagAccount };


// Context
#[derive(Accounts)]
#[instruction(name: String, color: u32)]
pub struct CreateTag<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_USER, owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,

    /// TagAccount PDA - seeds include tag_index for deterministic lookup
    #[account(
        init,
        payer = owner,
        space = TagAccount::LEN,
        seeds = [
            SEED_TAG,
            owner.key().as_ref(),
            &[user_account.tag_count],
        ],
        bump,
    )]
    pub tag_account: Account<'info, TagAccount>,

    pub system_program: Program<'info, System>,

}

// Handler
pub fn handler(ctx: Context<TagAccount>, name: String, color: u32) -> Result<()> {
    require!(name.len() <= MAX_TAG_NAME_LEN, TradaryError::TagNameTooLong);
    require!(name.len() > 0, TradaryError::TagNameTooLong); // non-empty

    let user = &ctx.accounts.user_account;
    require!(
        (user.tag_count as usize) < MAX_TAGS_PER_USER,
        TradaryError::TooManyUserTags
    );

    let clock = Clock::get()?;
    let tag_index = ctx.accounts.user_account.tag_count;

    let tag = &mut ctx.accounts.tag_account;
    tag.owner = ctx.accounts.owner.key();
    tag.bump = ctx.bumps.tag_account;
    tag.tag_index = tag_index;
    tag.name = name.clone();
    tag.color = color;
    tag.usage_count = 0;
    tag.created_at = clock.unix_timestamp;

    // Increment counter
    let user = &mut ctx.accounts.user_account;
    user.tag_count = user.tag_count.checked_add(1)
        .ok_or(TradaryError::ArithmeticError)?;
    
    emit!(TagCreated {
        owner: ctx.accounts.owner.key(),
        tag_index,
        name,
    });

    Ok(())
}

// Events
#[event]
pub struct TagCreated {
    pub owner: Pubkey,
    pub tag_index: u8,
    pub name: String,
}