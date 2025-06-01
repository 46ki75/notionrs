#[derive(Default, notionrs_macro::Setter)]
pub struct TestDeriveSetter {
    pub string_field: String,

    pub string_option_field: Option<String>,

    pub i32_field: i32,

    pub i32_option_field: Option<i32>,

    #[skip]
    pub skip: String,
}

#[test]
pub fn test_derive_setter() {
    let seed = TestDeriveSetter::default();

    seed.string_field("str")
        .string_field(String::from("String"));

    let seed = TestDeriveSetter::default();

    seed.string_option_field("str")
        .string_option_field(String::from("String"));

    let seed = TestDeriveSetter::default();

    seed.i32_field(32);

    let seed = TestDeriveSetter::default();

    seed.i32_option_field(32);
}
