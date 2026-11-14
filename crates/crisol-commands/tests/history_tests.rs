mod support;

use crisol_commands::commands::SetPropertyCommand;
use crisol_commands::history::CommandHistory;
use crisol_model::properties::PropertyValue;
use support::SampleForm;

fn rename_text(sample: &SampleForm, text: &str) -> SetPropertyCommand {
    SetPropertyCommand {
        node: sample.button,
        property: "text".to_owned(),
        value: Some(PropertyValue::Text(text.to_owned())),
    }
}

fn text_of(sample: &SampleForm) -> Option<String> {
    let node = sample.document.node(sample.button).unwrap();
    node.properties
        .get("text")
        .and_then(|value| value.as_text().map(str::to_owned))
}

#[test]
fn undo_and_redo_walk_the_history() {
    let mut sample = SampleForm::build();
    let mut history = CommandHistory::new();
    history
        .execute(&rename_text(&sample, "Save"), &mut sample.document)
        .unwrap();
    history
        .execute(&rename_text(&sample, "Store"), &mut sample.document)
        .unwrap();
    history.undo(&mut sample.document).unwrap();
    assert_eq!(text_of(&sample).as_deref(), Some("Save"));
    history.undo(&mut sample.document).unwrap();
    assert_eq!(text_of(&sample), None);
    history.redo(&mut sample.document).unwrap();
    assert_eq!(text_of(&sample).as_deref(), Some("Save"));
}

#[test]
fn a_new_command_clears_the_redo_stack() {
    let mut sample = SampleForm::build();
    let mut history = CommandHistory::new();
    history
        .execute(&rename_text(&sample, "Save"), &mut sample.document)
        .unwrap();
    history.undo(&mut sample.document).unwrap();
    history
        .execute(&rename_text(&sample, "Keep"), &mut sample.document)
        .unwrap();
    assert!(!history.can_redo());
}

#[test]
fn unsaved_changes_follow_undo_back_to_the_saved_point() {
    let mut sample = SampleForm::build();
    let mut history = CommandHistory::new();
    assert!(!history.has_unsaved_changes());
    history
        .execute(&rename_text(&sample, "Save"), &mut sample.document)
        .unwrap();
    assert!(history.has_unsaved_changes());
    history.undo(&mut sample.document).unwrap();
    assert!(!history.has_unsaved_changes());
    history
        .execute(&rename_text(&sample, "Keep"), &mut sample.document)
        .unwrap();
    history.mark_saved();
    history.undo(&mut sample.document).unwrap();
    history
        .execute(&rename_text(&sample, "Other"), &mut sample.document)
        .unwrap();
    assert!(history.has_unsaved_changes());
}
