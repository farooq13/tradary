use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::TradaryError;
use crate::state::{
    UserAccount, TradeAccount, TradeStatus
};

// Append-only notes update for an open trade.
// Appends new content to existing notes — never overwrites.
// Only permitted while trade status is Open.


// Context
#[derive(Accounts)]
#[instruction(trade_index: u32, additional_notes: String)]
pub struct AddTradeNotes<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [SEED_USER, owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [SEED_TRADE, owner.key().as_ref(), &trade_index.to_le_bytes()],
        bump = trade_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub trade_account: Account<'info, TradeAccount>,
}

// Handler
pub fn handler(
    ctx: Context<AddTradeNotes>, _trade_index: u32, additional_notes: String
) -> Result<()> {
    let trade = &ctx.accounts.trade_account;

    // Guard: immutable once closed
    require!(trade.status == TradeStatus::Open, TradaryError::TradeAlreadyClosed);

    // Build appended notes - separator prevents ambiguity
    let appended = if trade.notes.is_empty() {
        additional_notes.clone()
    } else {
        format!("{}\n---\n{}", trade.notes, additional_notes)
    };

    require!(appended.len() <= MAX_NOTES_LEN, TradaryError::NotesTooLong);

    let clock = Clock::get()?;
    let trade = &mut ctx.accounts.trade_account;
    trade.notes = appended;
    trade.updated_at = clock.unix_timestamp;

    Ok(())
}