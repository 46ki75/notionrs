#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod block;
pub mod client;
pub mod database;
pub mod error;
pub mod filter;
pub mod list_response;
pub mod others;
pub mod page;
pub mod prelude;
pub mod to_json;
pub mod user;

pub use crate::client::Client;
pub use crate::others::color::Color;
pub use crate::others::emoji::Emoji;
pub use crate::others::file::{ExternalFile, File, UploadedFile};
pub use crate::others::icon::Icon;
pub use crate::others::language::Language;
pub use crate::others::rich_text::RichText;
pub use crate::others::select::{Select, SelectColor, SelectGroup};
pub use crate::user::{bot::*, person::*, User};

pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}
