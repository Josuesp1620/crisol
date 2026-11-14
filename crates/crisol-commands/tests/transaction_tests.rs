mod support;

use crisol_commands::Command;
use crisol_commands::commands::{InsertNodeCommand, SetPropertyCommand, Transaction};
use crisol_model::properties::PropertyValue;
use support::SampleForm;

#[test]
fn a_transaction_applies_and_reverts_as_one_step() {
    let mut sample = SampleForm::build();
    let before = sample.document.clone();
    let transaction = Transaction::new(
        "Add title",
        vec![
            Box::new(InsertNodeCommand {
                parent: sample.stack,
                index: 0,
                node: SampleForm::node("Label", "lbl_title"),
            }),
            Box::new(SetPropertyCommand {
                node: sample.button,
                property: "text".to_owned(),
                value: Some(PropertyValue::Text("Save".to_owned())),
            }),
        ],
    );
    let reverse = transaction.apply(&mut sample.document).unwrap();
    assert_eq!(sample.document.children_of(sample.stack).len(), 2);
    reverse.apply(&mut sample.document).unwrap();
    assert_eq!(sample.document, before);
}

#[test]
fn a_failing_transaction_leaves_the_document_untouched() {
    let mut sample = SampleForm::build();
    let before = sample.document.clone();
    let transaction = Transaction::new(
        "Broken",
        vec![
            Box::new(InsertNodeCommand {
                parent: sample.stack,
                index: 0,
                node: SampleForm::node("Label", "lbl_title"),
            }),
            Box::new(InsertNodeCommand {
                parent: sample.stack,
                index: 0,
                node: SampleForm::node("Label", "btn_save"),
            }),
        ],
    );
    assert!(transaction.apply(&mut sample.document).is_err());
    assert_eq!(sample.document, before);
}
