use anchor_lang::prelude::*;

#[error_code]
pub enum TradaryError {
    // Authorization
    #[msg("Signer is not the account owner")]
    Unauthorized,

    // String length violation
    #[msg("Username exceeds maximum length of 32 characters")]
    UsernameTooLong,

    #[msg("Bio exceeds maximum length of 128 characters")]
    BioTooLong,

    #[msg("Symbol exceeds maximum length of 16 characters")]
    SymbolTooLong,

    #[msg("Notes exceeds maximum length of 512 characters")]
    NotesTooLong,

    #[msg("Tag name exceeds maximum length of 32 characters")]
    TagNameTooLong,

    // Collection limits
    #[msg("Trade already has the maximum number of tags (5)")]
    TooManyTagsOnTrade,

    #[msg("User has reached the maximum number of tags (50)")]
    TooManyUserTags,

    #[msg("Tag already exists for this user")]
    TagAlreadyExists,

    // Trade logic
    #[msg("Entry price must be greater thatn zero")]
    InvalidEntryPrice,

    #[msg("Exit price must be greater than zero")]
    InvalidExitPrice,

    #[msg("Size/quantity must be greater than zero")]
    InvalidSize,

    #[msg("Leverage must be between 1 and 100")]
    InvalidLeverage,

    #[msg("Exit timestamp must be after entry timestamp")]
    InvalidTimestampOrder,

    #[msg("Trade is already closed: no further updates permitted")]
    TradeAlreadyClosed,

    #[msg("Trade is still open: cannot close with no exit data")]
    TradeNotCloseable,

    // Arithmatic
    #[msg("Arithmatic overflow during PnL calculation")]
    ArithmaticOverflow,

    // Account versioning
    #[msg("Account version mismatch: please migrate your account")]
    VersionMismatch,
}