//!
//! The zkEVM ABI memory constants.
//!

/// The header offset (cells).
pub const OFFSET_HEADER: usize = 0;

/// The call and return data offset (cells).
pub const OFFSET_DATA: usize = 1;

/// The long-return flag offset (cells).
pub const OFFSET_LONG_RETURN: usize = 0xffff;
