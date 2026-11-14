use crisol_model::identity::{Identifier, TypeName};

#[test]
fn snake_case_names_are_valid_identifiers() {
    assert!(Identifier::parse("btn_save").is_ok());
    assert!(Identifier::parse("txt_tax_id2").is_ok());
}

#[test]
fn identifiers_reject_capitals_spaces_and_leading_digits() {
    assert!(Identifier::parse("BtnGuardar").is_err());
    assert!(Identifier::parse("btn save").is_err());
    assert!(Identifier::parse("2btn").is_err());
    assert!(Identifier::parse("").is_err());
}

#[test]
fn identifiers_reject_rust_keywords() {
    assert!(Identifier::parse("type").is_err());
    assert!(Identifier::parse("match").is_err());
}

#[test]
fn type_names_are_pascal_case() {
    assert!(TypeName::parse("CustomerForm").is_ok());
    assert!(TypeName::parse("customerForm").is_err());
    assert!(TypeName::parse("Customer_Form").is_err());
}
