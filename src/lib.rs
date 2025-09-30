//! crate with hydrus client and traits necessary for accesing hydrus API

/// this crate's hydrus client async implementation
#[cfg(feature = "async")]
pub mod client_async;
/// traits for accessing hydrus API
#[cfg(feature = "async")]
pub mod traits_async;
/// various objects for de/serializing requests
pub mod types;

#[cfg(test)]
mod tests;
