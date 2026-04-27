use anchor_lang::prelude::*;
use crate::constants::*;



#[account]
#[derive(Debug)]
pub struct TagAccount {
    pub owner: Pubkey,
    pub bump: u8,
    /// Position in user's tag list (0-based, max 49)
    pub tag_index: u8,
    /// Human-readable label
    pub name: String,  // 4 + MAX_TAG_NAME_LEN
    /// ARBG color stored as u32 for UI rendering
    pub color: u32,
     /// How many trades use this tag (maintained client-side, informational)
    pub usage_count: u32,
    /// Unix timestamp of creation
    pub created_at: i64,
}

impl TagAccount {
  pub const LEN: usize =
        DISCRIMINATOR
        + 32                    // owner
        + 1                     // bump
        + 1                     // tag_index
        + 4 + MAX_TAG_NAME_LEN  // name
        + 4                     // color
        + 4                     // usage_count
        + 8;                    // created_at
}