//! Tests that the `Setter` derive recognizes `String` and `Option<T>` even when
//! written with a fully-qualified path (`std::string::String`,
//! `std::option::Option<_>`).
//!
//! These currently fail because `is_string_type` / `is_option_type` /
//! the `Option` inner-type extraction in `setter.rs` inspect
//! `path.segments.first()` instead of `.last()`, so qualified paths fall
//! through to the generic `fn field(self, T) -> Self` branch and lose the
//! `AsRef<str>` / auto-`Some` ergonomics.

#[derive(Default, notionrs_macro::Setter)]
pub struct QualifiedString {
    pub field: ::std::string::String,
}

#[derive(Default, notionrs_macro::Setter)]
pub struct QualifiedOptionOfStr {
    pub field: ::std::option::Option<String>,
}

#[derive(Default, notionrs_macro::Setter)]
pub struct QualifiedOptionOfQualifiedStr {
    pub field: ::std::option::Option<::std::string::String>,
}

#[derive(Default, notionrs_macro::Setter)]
pub struct QualifiedOptionOfI32 {
    pub field: ::std::option::Option<i32>,
}

#[test]
fn qualified_string_accepts_as_ref_str() {
    // Should compile: `String`-typed setters take `impl AsRef<str>`.
    let s = QualifiedString::default().field("str");
    assert_eq!(s.field, "str");
}

#[test]
fn qualified_option_of_string_accepts_as_ref_str_and_wraps() {
    // Should compile: `Option<String>` setters take `impl AsRef<str>` and
    // wrap the value in `Some`.
    let s = QualifiedOptionOfStr::default().field("str");
    assert_eq!(s.field.as_deref(), Some("str"));
}

#[test]
fn qualified_option_of_qualified_string_accepts_as_ref_str_and_wraps() {
    let s = QualifiedOptionOfQualifiedStr::default().field("str");
    assert_eq!(s.field.as_deref(), Some("str"));
}

#[test]
fn qualified_option_of_i32_accepts_bare_value() {
    // Should compile: `Option<T>` (non-String) setters take `T` and wrap.
    let s = QualifiedOptionOfI32::default().field(42);
    assert_eq!(s.field, Some(42));
}
