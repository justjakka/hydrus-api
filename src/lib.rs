//! crate with hydrus client and traits necessary for fully using hydrus API

/// async traits and client implementation
#[cfg(feature = "async")]
pub mod async_lib;
/// sync traits and client implementation
#[cfg(feature = "sync")]
pub mod sync_lib;
/// types for hydrus API
pub mod types;
