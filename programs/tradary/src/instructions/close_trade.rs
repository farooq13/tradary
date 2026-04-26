use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::TradaryError;
use crate::state::{
    UserAccount, TradeAccount, TradeStatus,
    EmotionalState, TradeDirection,
};



// Input params
#[derive(AnchorSerialize, AnchorDeserialze, Clone, Debug)]
pub struct CloseTradeParams {
    /// Exit price in micro-units. Must be > 0;
    pub exit_price: i64,
    /// Fees paid on exit in micro-units (entry fees alreay captured or 0)
    pub fees_paid: u64,
    /// Emotional state at exit.
    pub emotion_exit: EmotionalState,
    /// Timestamp of exit
    pub exit_timestamp: i64,
}


// Context
#[derive(Accounts)]
#[instruction(params: CloseTradeParams, trade_index: u32)]
pub struct CloseTrade<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds: [SEED_USER, owner.key().as_ref()],
        bump = user_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [
            SEED_TRADE,
            owner.key().as_ref(),
            &trade_index.to_le_bytes(),
        ],
        bump = trade_account.bump,
        has_one = owner @ TradaryError::Unauthorized,
    )]
    pub trade_account: Account<'info, TradeAccount>,
}


// Handler
pub fn handler(ctx: Context<CloseTrade>, params: CloseTradeParams, _trade_index: u32 // used only in seed derivation, surfaced for clarity
    ) -> Result<()> {
        let trade = &ctx.accounts.trade_account;

        // Guard: must be open
        require!(trade.status == TradeStatus::Open, TradaryError::TradeAlreadyClosed);

        // Validate inputs
        require!(params.exit_price > 0, TradaryError::InvalidExitPrice);
        require!(
            params.exit_timestamp >= trade.entry_timestamp,
            TradaryError::InvalidTimestampOrder
        );

        let clock = Clock::get()?;

        // --- On-chain PnL calculation ---
        // PnL = (exit - entry) * size / PRICE_SCALE * direction_sign
        // All values in micro-units (6 decimals), so scale = 1_000_000
        let price_delta: i64 = params.exit_price
            .checked_sub(trade.entry_price)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        // Size is u64 base-asset micro-units. Cast carefully.
        let size_i64 = trade.size as i64;

        // raw_pnl = price_delta * size (still needs dividing by 1e12 for micro-units^2,
          // but since both are 6-decimal we only need 1e6 to normalize to USDC micro-units)
        let raw_pnl: i64 = price_delta
            .checked_mul(size_i64)
            .ok_or(TradaryError::ArithmaticOverflow)?
            .checked_div(1_000_000)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        // Apply direction (Short inverts PnL)
        let directional_pnl: i64 = match trade.direction {
            TradeDirection::Long => raw_pnl,
            TradeDirection::Short => raw_pnl.checked_neg().ok_or(TradaryError::ArithmaticOverflow)?,
        };

        // Apply leverage
        let leveraged_pnl: i64 = directional_pnl
            .checked_mul(trade.leverage as i64)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        // Subtract fees
        let net_pnl: i64 = leveraged_pnl
            .checked_sub(params.fees_paid as i64)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        // Write trade exit data
        {
            let trade = &mut ctx.accounts.trade_account;
            trade.exit_price = params.exit_price;
            trade.exit_timestamp = params.exit_timestamp;
            trade.emotion_exit = params.emotion_exit;
            trade.fees_paid = params.fees_paid;
            trade.pnl_realized = net_pnl;
            trade.status = TradeStatus::Closed;
            trade.updated_at = clock.unix_timestamp
        }

        // Update aggregate stats
        let user = &mut ctx.accounts.user_account;
        let stats = &mut user.stats;
        
        stats.total_trades = stats.total_trades.checked_add(1)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        stats.total_pnl_realized = stats.total_pnl_realized
            .checked_add(net_pnl)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        stats.total_fees_paid = stats.total_fees_paid
            .checked_add(params.fees_paid)
            .ok_or(TradaryError::ArithmaticOverflow)?;

        // Best/Worst trade tracking
        if net_pnl > stats.best_trade_pnl {
            stats.best_trade_pnl = net_pnl;
        }
        if net_pnl < stats.worst_trade_pnl {
            stats.worst_trade_pnl = net_pnl;
        }


        // Win/loss streak tracking
        if net_pnl > 0 {
            stats.winnning_trades = stats.winnning_trades.checked_add(1)
                .ok_or(TradaryError::ArithmaticOverflow)?;
            
            stats.current_win_streak = stats.current_win_streak.checked_add(1)
                .ok_or(TradaryError::ArithmaticOverflow)?;
            stats.current_lose_streak = 0;

            if stats.current_win_streak > stats.longest_win_streak {
                stats.longest_win_streak = stats.current_win_streak;
            }
        } else {
            stats.current_lose_streak = stats.current_lose_streak.checked_add(1)
                .ok_or(TradaryError::ArithmaticOverflow)?;
            stats.current_win_streak = 0;

            if stats.current_lose_streak > stats.longest_lose_streak {
                stats.longest_lose_streak = stats.current_lose_streak;
            }
        }

        let trade = &ctx.accounts.trade_account;
        emit!(TradeClosed {
            owner: ctx.accounts.owner.key(),
            trade_index: trade.trade_index,
            pnl_realized: net_pnl,
            exit_price: params.exit_price,
        });

        Ok(())
    }

    // Events
    #[event]
    pub struct TradeClosed {
        pub owner: Pubkey,
        pub trade_index: u32,
        pub pnl_realized: i64,
        pub exit_price: i64,
    }