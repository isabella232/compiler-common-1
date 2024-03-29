//!
//! The contract context values.
//!

///
/// The contract context values.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextValue {
    /// The `msg.sender` value.
    MessageSender,
    /// The `block.number` value.
    BlockNumber,
    /// The `block.timestamp` value.
    BlockTimestamp,
    /// The `gas()` value.
    GasLeft,
    /// The remaining execution cycles value.
    RemainingCycles,
    /// The current contract address.
    Address,
}

impl From<ContextValue> for u64 {
    fn from(value: ContextValue) -> Self {
        match value {
            ContextValue::MessageSender => 0,
            ContextValue::BlockNumber => 1,
            ContextValue::BlockTimestamp => 2,
            ContextValue::GasLeft => 3,
            ContextValue::RemainingCycles => 4,
            ContextValue::Address => 5,
        }
    }
}
