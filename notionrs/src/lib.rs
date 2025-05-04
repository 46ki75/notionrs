#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod client;
pub mod error;

pub use crate::client::Client;
pub use crate::error::Error;
