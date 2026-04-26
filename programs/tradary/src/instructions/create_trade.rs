use anchor_lang::prelude::*:
use crate::constants::*:
use crate::errors::TradaryError;
use crate::state::{
    UserAccount, TradeAccount, TradeStatus,
    EmotionalState, TradeDirection, AssetClass,
};



// Input params (separate struct for clarity)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateTradeParams {
    pub symbol: String,
    pub direction: TradeDirection,
    pub asset_class: AssetClass,
    /// Entry price in micro-units (6 decimals). Must be > 0.
    pub entry_price: i64,
    /// Position size in base-asset micro-units. Must be > 0.
    pub size: u64,
    /// Leverage multiplier [1, 100].
    pub leverage: u8,
    pub emotion_entry: EmotionalState,
    /// Initial notes (can be appended to later while Open).
    pub notes: String,
    /// Tag indices (references TagAccount.tag_index values for this user).
    pub tag_indices: Vec<u8>,
    /// Entry timestamp (Unix). Passed by client so it can use trade execution
    pub entry_timestamp: i64,
}



// Context
#[derive(Accounts)]
#[instruction(params: CreateTradeParams)]
pub struct CreateTrade<'info> {
    /// UserAccount must already be initialized and owned by signer.
    #[account(
        mut, 
        seeds = [USER_SEED, signer.key().as_ref()],
        bump = user_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,

    
    /// TradeAccount PDA — uniquely derived from (owner, trade_count).
    /// trade_count is incremented AFTER this account is created so the
    /// index in the seed matches the account's trade_index field.

    #[account(
        init,
        payer = owner,
        space = TradeAccount::LEN,
        seeds = [
            SEED_TRADE,
            owner.key().as_ref(),
            &user_account.trade_count.to_le_bytes(),
        ],
        bump,
    )]
    pub trade_account: Account<'info, TradeAccount>,

    pub system_program: Program<'info, System>,
}

/// Handle
pub fn handler(ctx: Context<CreateTrade>, params: CreateTradeParams
) -> Result<()> {
    // input validation
    require!(params.symbol.len() <= MAX_SYMBOLE_LEN, TradaryError::SymbolTooLong);
    require!(params.notes.len() <= MAX_NOTES_LEN, TradaryError::NotesTooLong);
    require!(params.entry_price > 0, TradaryError::InvalidEntryPrice);
    require!(params.size > 0, TradaryError::InvalidSize);
    require!(params.leverage >= 1 && params.leverage <= MAX_LEVERAGE, TradaryError::InvalidLeverage);
    require!(params.tag_indices.len() <= MAX_TAGS_PER_TRADE, TradaryError::TooManyTagsOnTrade);

    let clock = Clock::get()?;
    let trade_index = ctx.accounts.user_account.trade_count;

    let trade = &mut ctx.accounts.trade_account;

    trade.version = CURRENT_VERSION;
    trade.owner = ctx.accounts.owner.key();
    trade.bump = ctx.bumps.trade_account;
    trade.symbol = params.symbol.clone();
    trade.direction = params.direction;
    trade.asset_class = params.asset_class;
    trade.entry_price = params.entry_price;
    trade.exit_price = 0;
    trade.size = params.size;
    trade.leverage = params.leverage;
    trade.pnl_realized = 0;
    trade.fees_paid = 0;
    trade.entry_timestamp = params.entry_timestamp;
    trade.exit_timestamp = 0;
    trade.emotion_entry = params.emotion_entry;
    trade.emotion_exit = EmotionalState::Neutral;
    trade.notes = params.notes;
    trade.tag_indices = params.tag_indices;
    trade.status = TradeStatus::Open;
    trade.update_at = clock.unix_timestamp;
    trade._reserved = [0u8; 16];

    // Increment trade_count to ensure next PDA uses a different seed
    let user = &mut ctx.accounts.user_account;
    user.trade_count = user.trade_count.checked_add(1)
        .ok_or(TradaryError::ArithmeticOverflow)?;

    emit!(TradeOpened {
        owner: ctx.accounts.owner.key(),
        trade_index,
        symbol: params.symbol,
        entry_price: params.entry_price,
        entry_timestamp: params.entry_timestamp,
    });

    Ok(())
}



// Events
#[event]
pub struct TradeOpened {
    pub owner: Pubkey,
    pub trade_index: u32,
    pub symbol: String,
    pub entry_price: i64,
    pub entry_timestamp: i64,
}