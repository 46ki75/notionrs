#[derive(Default, notionrs_macro::Setter)]
pub struct TestDeriveSetter {
    pub string_field: String,

    pub string_option_field: Option<String>,

    pub i32_field: i32,

    pub i32_option_field: Option<i32>,

    #[setter(skip)]
    pub skip: String,
}

#[test]
pub fn test_derive_setter() {
    let seed = TestDeriveSetter::default()
        .string_field("str")
        .string_field(String::from("String"));
    assert_eq!(seed.string_field, "String");

    let seed = TestDeriveSetter::default()
        .string_option_field("str")
        .string_option_field(String::from("String"));
    assert_eq!(seed.string_option_field.as_deref(), Some("String"));

    let seed = TestDeriveSetter::default().i32_field(32);
    assert_eq!(seed.i32_field, 32);

    let seed = TestDeriveSetter::default().i32_option_field(32);
    assert_eq!(seed.i32_option_field, Some(32));
}

#[derive(Default, notionrs_macro::Setter)]
pub struct GenericStruct<T>
where
    T: Default,
{
    pub payload: T,
    pub name: String,
}

#[test]
fn test_generic_struct() {
    let s: GenericStruct<i64> = GenericStruct::default().payload(42).name("hello");
    assert_eq!(s.payload, 42);
    assert_eq!(s.name, "hello");
}

/// Documented for rustdoc coverage.
#[derive(Default, notionrs_macro::Setter)]
pub struct DocumentedStruct {
    /// The user's display name.
    /// Spans multiple lines.
    pub name: String,
}

#[test]
fn test_documented_struct_compiles() {
    let s = DocumentedStruct::default().name("x");
    assert_eq!(s.name, "x");
}

// Exercises the non-`Type::Path` fall-through in `is_option_type` /
// `is_string_type` (a tuple type is `Type::Tuple`).
#[derive(Default, notionrs_macro::Setter)]
pub struct NonPathFieldType {
    pub pair: (i32, i32),
}

#[test]
fn test_non_path_field_type() {
    let s = NonPathFieldType::default().pair((1, 2));
    assert_eq!(s.pair, (1, 2));
}

// `#[doc(hidden)]` parses as `Meta::List`, not `Meta::NameValue`, so the
// inner string-literal extraction in `generate_comment` returns `None`.
#[derive(Default, notionrs_macro::Setter)]
pub struct HiddenDocField {
    #[doc(hidden)]
    pub field: String,
}

#[test]
fn test_hidden_doc_field() {
    let s = HiddenDocField::default().field("v");
    assert_eq!(s.field, "v");
}

