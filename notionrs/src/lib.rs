#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod client;
pub mod error;
pub mod r#macro;
pub mod object;
pub mod prelude;
pub(crate) mod serde;

pub use crate::client::Client;
pub use crate::error::Error;
