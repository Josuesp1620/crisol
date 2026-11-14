mod support;

use crisol_codegen::CodegenSettings;
use crisol_codegen::stubs::HandlerStubWriter;
use support::CustomerForm;

#[test]
fn a_new_handlers_file_implements_every_handler() {
    let sample = CustomerForm::build();
    let writer = HandlerStubWriter::new(&sample.document, CodegenSettings::default());
    let text = writer.create(&["save".to_owned()]);
    assert_eq!(writer.file_name(), "customer_form_events.rs");
    assert!(text.contains("use super::customer_form::{CustomerForm, CustomerFormEvents};"));
    assert!(text.contains("impl CustomerFormEvents for CustomerForm {\n    fn save(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}\n}\n"));
}

#[test]
fn extending_only_adds_missing_methods_and_keeps_user_code() {
    let sample = CustomerForm::build();
    let writer = HandlerStubWriter::new(&sample.document, CodegenSettings::default());
    let existing = "impl CustomerFormEvents for CustomerForm {\n    fn save(&mut self, _window: &mut Window, cx: &mut Context<Self>) {\n        store(cx);\n    }\n}\n";
    let extended = writer
        .extend(existing, &["save".to_owned(), "cancel".to_owned()])
        .unwrap();
    assert!(extended.contains("        store(cx);\n    }\n"));
    assert!(extended.contains(
        "\n    fn cancel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}\n}\n"
    ));
    assert_eq!(extended.matches("fn save(").count(), 1);
}

#[test]
fn a_file_without_the_impl_block_is_reported() {
    let sample = CustomerForm::build();
    let writer = HandlerStubWriter::new(&sample.document, CodegenSettings::default());
    assert!(
        writer
            .extend("fn main() {}\n", &["save".to_owned()])
            .is_err()
    );
}
