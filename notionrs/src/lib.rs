#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub mod client;
pub mod error;
pub mod r#macro;
pub mod object;

pub use crate::client::Client;
pub use crate::object::color::Color;
pub use crate::object::emoji::Emoji;
pub use crate::object::file::{ExternalFile, File, UploadedFile};
pub use crate::object::icon::Icon;
pub use crate::object::language::Language;
pub use crate::object::rich_text::RichText;
pub use crate::object::select::{Select, SelectColor, SelectGroup};
