//! crate with hydrus client and traits necessary for fully using hydrus API

/// async traits, types and client implementation
#[cfg(feature = "async")]
pub mod async_lib;
/// sync traits, types and client implementation
#[cfg(feature = "sync")]
pub mod sync_lib;
