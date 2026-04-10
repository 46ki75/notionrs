#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod client;
pub mod error;
pub(crate) mod r#macro;
pub mod r#trait;
pub(crate) mod util;

pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::r#trait::PaginateExt;

pub mod types {
    pub mod prelude {
        pub use notionrs_types::prelude::*;
    }
}
