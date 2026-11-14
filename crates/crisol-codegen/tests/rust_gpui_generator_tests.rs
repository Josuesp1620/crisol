mod support;

use crisol_codegen::rust_gpui::RustGpuiGenerator;
use crisol_codegen::{CodeGenerator, CodegenError, CodegenSettings};
use crisol_model::properties::PropertyValue;
use support::CustomerForm;

fn generate(sample: &CustomerForm) -> String {
    let generator = RustGpuiGenerator::new(&sample.catalog, CodegenSettings::default());
    generator.generate(&sample.document).unwrap().content
}

#[test]
fn the_file_is_named_after_the_form() {
    let sample = CustomerForm::build();
    let generator = RustGpuiGenerator::new(&sample.catalog, CodegenSettings::default());
    assert_eq!(
        generator.generate(&sample.document).unwrap().file_name,
        "customer_form.generated.rs"
    );
}

#[test]
fn the_events_trait_has_one_method_per_handler() {
    let content = generate(&CustomerForm::build());
    assert!(content.contains("pub trait CustomerFormEvents {"));
    assert!(content.contains("/// `btn_save`: Click."));
    assert!(content.contains("fn save(&mut self, window: &mut Window, cx: &mut Context<Self>);"));
    assert!(
        content
            .contains("fn tax_id_changed(&mut self, window: &mut Window, cx: &mut Context<Self>);")
    );
}

#[test]
fn stateful_controls_become_entity_fields() {
    let content = generate(&CustomerForm::build());
    assert!(content.contains("pub txt_tax_id: Entity<TextBox>,"));
    assert!(content.contains("pub cmb_kind: Entity<ComboBox>,"));
    assert!(content.contains(
        "let txt_tax_id = cx.new(|cx| TextBox::new(cx).placeholder(\"20123456789\").max_length(11));"
    ));
    assert!(
        content.contains(
            "let cmb_kind = cx.new(|cx| ComboBox::new(cx, vec![\"Company\", \"Person\"]));"
        )
    );
}

#[test]
fn entity_events_are_subscribed_with_a_wildcard_for_unbound_ones() {
    let content = generate(&CustomerForm::build());
    assert!(content.contains(
        "cx.subscribe_in(&txt_tax_id, window, |this, _, event: &TextBoxEvent, window, cx| {"
    ));
    assert!(content.contains("TextBoxEvent::Changed => {"));
    assert!(content.contains("this.tax_id_changed(window, cx);"));
    assert!(content.contains("_ => {}"));
}

#[test]
fn buttons_call_their_handler_and_escape_their_text() {
    let content = generate(&CustomerForm::build());
    assert!(content.contains("Button::new(\"btn_save\", \"Save \\\"now\\\"\")"));
    assert!(content.contains(".variant(ButtonVariant::Primary)"));
    assert!(
        content.contains(".on_click(cx.listener(|this, _, window, cx| this.save(window, cx)))")
    );
}

#[test]
fn containers_write_their_layout() {
    let content = generate(&CustomerForm::build());
    assert!(content.contains("Stack::vertical()"));
    assert!(content.contains(".fill()"));
    assert!(content.contains("Grid::new(2)"));
    assert!(content.contains(".first_column_width(104.0)"));
    assert!(content.contains(".cell(0, 1, 1, Length::Fill,"));
    assert!(content.contains("Stack::horizontal()"));
    assert!(content.contains(".align(StackAlignment::End)"));
}

#[test]
fn hidden_controls_are_left_out() {
    let mut sample = CustomerForm::build();
    let check = sample.document.find_by_name("chk_active").unwrap();
    sample.set(check, "visible", PropertyValue::Boolean(false));
    let content = generate(&sample);
    assert!(!content.contains("self.chk_active.clone()"));
}

#[test]
fn invalid_documents_are_refused() {
    let mut sample = CustomerForm::build();
    let save = sample.document.find_by_name("btn_save").unwrap();
    sample.set(save, "variant", PropertyValue::Text("huge".to_owned()));
    let generator = RustGpuiGenerator::new(&sample.catalog, CodegenSettings::default());
    assert!(matches!(
        generator.generate(&sample.document),
        Err(CodegenError::InvalidDocument(1))
    ));
}
