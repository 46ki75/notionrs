/// This macro is used to generate a setter method for a struct field.
///
/// ```ignore
/// crate::impl_from_as_ref!(BookmarkBlock, url);
/// ```
/// ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓
/// ```ignore
/// impl<T> From<T> for BookmarkBlock
/// where
///     T: AsRef<str>,
/// {
///     fn from(url: T) -> Self {
///         Self::default().url(url)
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_from_as_ref {
    ($struct_name:ident, $field:ident) => {
        impl<T> From<T> for $struct_name
        where
            T: AsRef<str>,
        {
            fn from(value: T) -> Self {
                let mut instance = Self::default();
                instance.$field = value.as_ref().to_string();
                instance
            }
        }
    };
}
