//!
//! The constant identifiers.
//!

/// The circuit entry function name.
pub static FUNCTION_MAIN: &str = crate::file_name::APPLICATION_ENTRY;

/// The contract entry selector function name.
pub static FUNCTION_SELECTOR: &str = "__selector";

/// The intrinsic `msg` variable name.
pub static VARIABLE_MSG: &str = "msg";

/// The intrinsic `msg.sender` field name.
pub static FIELD_MSG_SENDER: &str = "sender";

/// The intrinsic `block` variable name.
pub static VARIABLE_BLOCK: &str = "block";

/// The intrinsic `block.number` field name.
pub static FIELD_BLOCK_NUMBER: &str = "number";

/// The intrinsic `block.timestamp` field name.
pub static FIELD_BLOCK_TIMESTAMP: &str = "timestamp";
