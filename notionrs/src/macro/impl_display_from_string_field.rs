/// Implement `Display` trait for a struct that has a field of type `String`.
///
///
/// ```rust
/// impl std::fmt::Display for BookmarkBlock {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         write!(f, "{}", self.url)
///     }
/// }
/// ```
/// ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓
/// ```rust
/// crate::impl_display_from_string_field!(BookmarkBlock, url);
/// ```
#[macro_export]
macro_rules! impl_display_from_string_field {
    ($struct_name:ident, $field:ident) => {
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.$field)
            }
        }
    };
}
