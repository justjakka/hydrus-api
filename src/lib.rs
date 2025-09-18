//! crate with hydrus client and traits necessary for accesing hydrus API

/// this crates hydrus client implementation
pub mod client;
/// traits for accessing hydrus API
pub mod traits;
/// various objects for de/serializing requests
pub mod types;

#[cfg(test)]
mod tests;
