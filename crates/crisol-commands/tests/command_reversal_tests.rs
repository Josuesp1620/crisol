mod support;

use crisol_commands::Command;
use crisol_commands::commands::{
    BindEventCommand, InsertNodeCommand, MoveNodeCommand, RemoveNodeCommand, RenameNodeCommand,
    SetPlacementCommand, SetPropertyCommand,
};
use crisol_model::geometry::Rect;
use crisol_model::identity::Identifier;
use crisol_model::layout::{AnchorSet, Placement};
use crisol_model::properties::PropertyValue;
use support::SampleForm;

fn assert_reverses(sample: &mut SampleForm, command: &dyn Command) {
    let before = sample.document.clone();
    let reverse = command.apply(&mut sample.document).unwrap();
    assert_ne!(sample.document, before, "{} changed nothing", command.id());
    reverse.apply(&mut sample.document).unwrap();
    assert_eq!(sample.document, before, "{} was not reverted", command.id());
}

#[test]
fn inserting_is_reversible() {
    let mut sample = SampleForm::build();
    let command = InsertNodeCommand {
        parent: sample.stack,
        index: 1,
        node: SampleForm::node("Label", "lbl_title"),
    };
    assert_reverses(&mut sample, &command);
}

#[test]
fn removing_is_reversible() {
    let mut sample = SampleForm::build();
    let command = RemoveNodeCommand { node: sample.stack };
    assert_reverses(&mut sample, &command);
}

#[test]
fn moving_is_reversible() {
    let mut sample = SampleForm::build();
    let command = MoveNodeCommand {
        node: sample.button,
        new_parent: sample.document.root_id(),
        index: 0,
    };
    assert_reverses(&mut sample, &command);
}

#[test]
fn setting_and_resetting_properties_is_reversible() {
    let mut sample = SampleForm::build();
    let command = SetPropertyCommand {
        node: sample.button,
        property: "text".to_owned(),
        value: Some(PropertyValue::Text("Save".to_owned())),
    };
    assert_reverses(&mut sample, &command);
}

#[test]
fn changing_placement_is_reversible() {
    let mut sample = SampleForm::build();
    let command = SetPlacementCommand {
        node: sample.button,
        placement: Placement::Absolute {
            rect: Rect::new(10, 10, 100, 32),
            anchor: AnchorSet::default(),
        },
    };
    assert_reverses(&mut sample, &command);
}

#[test]
fn binding_events_is_reversible() {
    let mut sample = SampleForm::build();
    let command = BindEventCommand {
        node: sample.button,
        event: "click".to_owned(),
        handler: Some(Identifier::parse("save").unwrap()),
    };
    assert_reverses(&mut sample, &command);
}

#[test]
fn renaming_is_reversible_and_keeps_names_unique() {
    let mut sample = SampleForm::build();
    let command = RenameNodeCommand {
        node: sample.button,
        name: Identifier::parse("btn_store").unwrap(),
    };
    assert_reverses(&mut sample, &command);
    let clash = RenameNodeCommand {
        node: sample.button,
        name: Identifier::parse("stk_fields").unwrap(),
    };
    assert!(clash.apply(&mut sample.document).is_err());
}
