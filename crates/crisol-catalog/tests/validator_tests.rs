mod support;

use crisol_catalog::validation::{CatalogValidator, DiagnosticSeverity};
use crisol_model::properties::PropertyValue;
use support::SampleDocument;

fn topics(sample: &SampleDocument) -> Vec<(DiagnosticSeverity, &'static str)> {
    let validator = CatalogValidator::new(&sample.catalog);
    let mut found = Vec::new();
    for diagnostic in validator.validate(&sample.document) {
        found.push((diagnostic.severity, diagnostic.topic));
    }
    found
}

#[test]
fn a_well_formed_form_has_no_findings() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let button = sample.add("Button", root);
    sample.set(button, "text", PropertyValue::Text("Save".to_owned()));
    assert!(topics(&sample).is_empty());
}

#[test]
fn unknown_properties_and_wrong_values_are_errors() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let button = sample.add("Button", root);
    sample.set(button, "colour", PropertyValue::Text("red".to_owned()));
    sample.set(button, "variant", PropertyValue::Text("huge".to_owned()));
    assert_eq!(
        topics(&sample),
        vec![
            (DiagnosticSeverity::Error, "properties"),
            (DiagnosticSeverity::Error, "properties")
        ]
    );
}

#[test]
fn controls_cannot_hold_children() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let button = sample.add("Button", root);
    sample.add("Label", button);
    assert!(topics(&sample).contains(&(DiagnosticSeverity::Error, "structure")));
}

#[test]
fn absolute_positions_and_tooltips_are_not_portable() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let panel = sample.add("Panel", root);
    let button = sample.add("Button", panel);
    sample.set(
        button,
        "tooltip",
        PropertyValue::Text("Saves the customer".to_owned()),
    );
    let found = topics(&sample);
    assert_eq!(found.len(), 2);
    assert!(
        found
            .iter()
            .all(|finding| *finding == (DiagnosticSeverity::Warning, "portability"))
    );
}

#[test]
fn inputs_without_an_accessible_name_are_warned() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let text_box = sample.add("TextBox", root);
    assert_eq!(
        topics(&sample),
        vec![(DiagnosticSeverity::Warning, "accessibility")]
    );
    sample.set(
        text_box,
        "accessible_name",
        PropertyValue::Text("Tax id".to_owned()),
    );
    assert!(topics(&sample).is_empty());
}
