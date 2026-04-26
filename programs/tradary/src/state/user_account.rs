use anchor_lang::prelude::*;
use crate::constants::*;



/// Emotional state at time of trade entry.
/// Stored as a u8 on-chain (1 byte) for minimal footprint.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum EmotionalState {
    Neutral    = 0,
    Confident  = 1,
    Fearful    = 2,
    Greedy     = 3,
    Anxious    = 4,
    Calm       = 5,
    Frustrated = 6,
    Euphoric   = 7,
    Revenge    = 8, // Revenge trading flag — a critical behavioral pattern
}

impl Default for EmotionalState {
    fn default() -> Self {
        EmotionalState::Neutral
    }
}

/// Direction of the trade.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TradeDirection {
    Long = 0,
    Short = 1,
}

impl Default for TradeDirection {
    fn default() -> Self {
        TradeDirection::Long
    }
}


/// Asset class for categorization & filtering
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AssetClass {
    Spot      = 0,
    Perpetual = 1,
    Opitions  = 2,
    Futures   = 3,
    Other     = 4
}

impl Default for AssetClass {
    fn default() -> Self {
        AssetClass::Spot
    }
}

/// Aggregate statistics stored inline to avoid expensive iteration.
/// Updated atomically on each trade close instruction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct TradingStats {
    /// Total njmber of closed trades
    pub total_trades: u32,
    /// Number of winning trades (pnl_realized > 0)
    pub winning_trades: u32,
    /// Cummulative realized PnL in USDC micro-units (i64 supports negative)
    pub total_pnl_realized: i64,
    /// Best single trade PnL
    pub best_trade_pnl: i64,
    /// Worst single trade PnL
    pub worst_trade_pnl: i64,
    /// Total fees paid (helps calculate net vs gross PnL)
    pub total_fees_paid: u64,
    /// Consecutive winning streak (current)
    pub current_win_streak: u16,
    /// Longest winning streak ever recorded
    pub longest_win_streak: u16,
    /// Current consecutive losing streak
    pub current_lose_streak: u16,
    /// Longest losing streak ever recorded
    pub longest_lose_streak: u16,
    /// Reserved bytes for future stat fields without account migration
    pub _reserved: [u8; 16],
}

impl TradingStats {
       pub const LEN: usize =
        4   // total_trades
        + 4   // winning_trades
        + 8   // total_pnl_realized
        + 8   // best_trade_pnl
        + 8   // worst_trade_pnl
        + 8   // total_fees_paid
        + 2   // current_win_streak
        + 2   // longest_win_streak
        + 2   // current_lose_streak
        + 2   // longest_lose_streak
        + 16; // _reserved
}

/// Main user profile account
#[account]
#[derive(Debug)]
pub struct UserAccount {
    /// Schema version for forward-compatibility
    pub version: u8,
    /// The wallet that owns this account
    pub owner: Pubkey,
    /// PDA bump for self-referencing
    pub bump: u8,
    /// Human-readable display name (optional)
    pub username: String, // 4 + MAX_USERNAME_LEN
    /// Short bio / trader description
    pub bio: String, // 4 + MAX_BIO_LEN
    /// Unix timestamp of account creation
    pub created_at: i64,
    /// Sequential counter - used as seed for TradeAccount PDAs
    pub trade_count: u32,
    /// Tag count for this user
    pub tag_count: u8,
    /// Aggregate trading statistics (updated on trade close)
    pub stats: TradingStats,
    /// Privacy flag: if true, notes field on trades is client-side encrypted
    pub privacy_enabled: bool,
    /// Reserved for future extension (e.g., referal, tier)
    pub _reserved: [u8; 32],
}

impl UserAccount {
   pub const LEN: usize =
        DISCRIMINATOR
        + 1                         // version
        + 32                        // owner
        + 1                         // bump
        + 4 + MAX_USERNAME_LEN      // username
        + 4 + MAX_BIO_LEN          // bio
        + 8                         // created_at
        + 4                         // trade_count
        + 1                         // tag_count
        + TradingStats::LEN         // stats
        + 1                         // privacy_enabled
        + 32;                       // _reserved
}