use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::user_account::{EmotionalState, TradeDirection, AssetClass};



// Lifecycle of a trade
#[derive(AnchorSerialise, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TradeStatus {
    Open  = 0,
    Closed = 1,
}

impl Default for TradeStatus {
    fn default() -> Self {
        TradeStatus::Open
    }
}


#[account]
#[derive(Debug)]
pub struct TradeAccount {
    /// Schema version for forward-compatibility
    pub version: u8,
    /// The wallet that owns this trade
    pub owner: Pubkey,
    pub bump: u8,
    /// Sequential index within this user's trade history (0-based)
    pub trade_index: u32,
    /// Trade symbol/ticker (e.g., "BTC/USDC", "SOL/PERP")
    pub symbol: String, /// 4 + MAX_SYMBOL_LEN
    /// Long or Short
    pub direction: TradeDirection,
    /// Spot, Perp, Options, Futures
    pub asset_class: AssetClass,
    /// Entry price in micro-units
    pub entry_price: i64,
    /// Exit price in micro-uints (0 if still open)
    pub exit_price: iu64,
    /// Position size (quantity of base asset, 6 decimals)
    pub size: u64,
    /// Leverage used (1 = no leverage, max 100)
    pub leverage: u8,
    /// Realized PnL in USDC micro-units (negative = loss)
    pub pnl_realized: i64,
    /// Total fees paid (entry + exit) in USDC micro-units
    pub fees_paid: u64,
    /// Unix timestamp of trade entry
    pub entry_timestamp: i64,
    /// Unix timestamp of trade exit (0 if still open)
    pub exit_timestamp: i64,
    /// Emotional state at trade entry
    pub emotion_entry: EmotionalState,
    /// Emotional state at trade exit (default Neutral untill closed)
    pub emotion_exit: EmotionalState,
    // Optional free-form notes (encrypted client-side if privacy_enabled)
    pub notes: String,   // 4 + MAX_NOTES_LEN
    /// Tag indices referencing TagAccount PDAs for this user
    /// Stored as u8 indices (max 5 tags, max 50 tags per user)
    pub tag_indices: Vec<u8>, // 4 + MAX_TAGS_PER_TRADE
    /// Current lifecycle status
    pub status: TradeStatus,
    /// Unix timestamp of last on-chain mutation (creation or close)
    pub updated_at: i64,
    /// Reserved for future use
    pub _reserved: [u8; 16],
}

impl TradeAccount {
    pub const LEN: usize =
        DISCRIMINATOR
        + 1                         // version
        + 32                        // owner
        + 1                         // bump
        + 4                         // trade_index
        + 4 + MAX_SYMBOL_LEN        // symbol
        + 1                         // direction
        + 1                         // asset_class
        + 8                         // entry_price
        + 8                         // exit_price
        + 8                         // size
        + 1                         // leverage
        + 8                         // pnl_realized
        + 8                         // fees_paid
        + 8                         // entry_timestamp
        + 8                         // exit_timestamp
        + 1                         // emotion_entry
        + 1                         // emotion_exit
        + 4 + MAX_NOTES_LEN         // notes
        + 4 + MAX_TAGS_PER_TRADE    // tag_indices
        + 1                         // status
        + 8                         // updated_at
        + 16;                       // _reserved
}