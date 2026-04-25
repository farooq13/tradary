/// Seed prefixes for PDA derivation
pub const SEED_USER: &[u8] = b"user";
pub const SEED_TRADE: &[u8] = b"trade";
pub const SEED_TAG: &[u8] = b"tag";

/// Account size constraints
pub const MAX_USERNAME_LEN: usize = 32;
pub const MAX_BIO_LEN: usize = 128;
pub const MAX_SYMBOL_LEN: usize = 16;
pub const MAX_NOTES_LEN: usize = 512;
pub const MAX_TAG_NAME_LEN: usize = 32;
pub const MAX_TAGS_PER_TRADE: usize = 5;
pub const MAX_TAGS_PER_USER: usize = 50;

/// Dicriminator size (8 bytes, Anchor default)
pub const DISCRIMINATOR: usize = 8;

/// Price precision: stored as i64 with 6 decimal places (like USDC)
/// e.g., $1.234567 = 1_234_567
pub const PRICE_DECIMALS: u32 = 6;

/// Maximum leverage allowed (safety guard, not enforced on-chain fully)
pub const MAX_LEVERAGE: u8 = 100;

/// Version for account migration support
pub const CURRENT_VERSION: u8 = 1;