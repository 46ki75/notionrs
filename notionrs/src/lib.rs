#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod client;
pub mod error;
pub mod filter;
pub mod list_response;
pub mod r#macro;
pub mod object;
pub mod others;
pub mod search;
pub mod user;

pub use crate::client::Client;
pub use crate::others::color::Color;
pub use crate::others::emoji::Emoji;
pub use crate::others::file::{ExternalFile, File, UploadedFile};
pub use crate::others::icon::Icon;
pub use crate::others::language::Language;
pub use crate::others::rich_text::RichText;
pub use crate::others::select::{Select, SelectColor, SelectGroup};
pub use crate::user::{User, bot::*, person::*};
