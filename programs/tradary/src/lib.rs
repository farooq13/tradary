use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("6Waiw2whwzAKVmqmsdSmJdBXtqyveYEMvPitRK3MoHRL");

#[program]
pub mod tradary {
    use super::*;

    // -- User Profile --

    /// Initialize a new trading journal profile.
    /// Must be called once per wallet before any trades can be logged.
    pub fn initialize_user(ctx: Context<InitializeUser>, username: String, bio: String, privacy_enabled: bool) -> Result<()> {
        initialize_user::handler(ctx, username, bio, privacy_enabled)
    }

    // update mutable profile fields (username, bio, privacy settings)
    pub fn update_user_profile(ctx: Context<UpdateUserProfile>, params: UpdateUserProfileParams) -> Result<()> {
        update_user_profile::handler(ctx, params)
    }


    // -- Trades --

    /// Open a new trade entry in the journal.
    /// Creates an immutable TradeAccount PDA seeded by the trade index.
    pub fn create_trade(ctx: Context<CreateTrade>, params: CreateTradeParams) -> Result<()> {
        create_trade::handler(ctx, params)
    }

    /// Close an open trade, recording exit data and updating stats.
    /// After this call the trade is permanently immutable.
    pub fn close_trade(ctx: Context<CloseTrade>, params: CloseTradeParams, trade_index: u32) -> Result<()> {
        close_trade::handler(ctx, params, trade_index)
    }

    /// Append notes to an open trade (append-only; not allowed after closure).
    pub fn add_trade_notes(
        ctx: Context<AddTradeNotes>,
        trade_index: u32,
        additional_notes: String,
    ) -> Result<()> {
        add_trade_notes::handler(ctx, trade_index, additional_notes)
    }
 
     // Tags
     /// Create a strategy tag for categorizing trades.
    pub fn create_tag(
        ctx: Context<CreateTag>,
        name: String,
        color: u32,
    ) -> Result<()> {
        create_tag::handler(ctx, name, color)
    }
}


