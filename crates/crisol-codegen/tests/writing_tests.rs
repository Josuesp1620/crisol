use crisol_codegen::writing::{NameConverter, RustLiteral};

#[test]
fn names_convert_between_styles() {
    assert_eq!(NameConverter::snake_case("CustomerForm"), "customer_form");
    assert_eq!(
        NameConverter::pascal_case("selection_changed"),
        "SelectionChanged"
    );
}

#[test]
fn string_literals_escape_quotes_and_line_breaks() {
    assert_eq!(RustLiteral::string("a \"b\"\n"), "\"a \\\"b\\\"\\n\"");
    assert_eq!(RustLiteral::string_list(&["x".to_owned()]), "vec![\"x\"]");
}
