mod autogenerated;

pub use crate::autogenerated::*;

pub use protobuf::Message;

#[cfg(feature = "with-serde")]
pub mod serde;