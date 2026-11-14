mod support;

use crisol_model::ModelError;
use crisol_model::file::DocumentSerializer;
use support::SampleForm;

#[test]
fn reading_what_was_written_gives_the_same_document() {
    let sample = SampleForm::build();
    let serializer = DocumentSerializer::default();
    let text = serializer.write(&sample.document).unwrap();
    let read_back = serializer.read(&text).unwrap();
    assert_eq!(read_back, sample.document);
}

#[test]
fn writing_twice_gives_identical_bytes() {
    let sample = SampleForm::build();
    let serializer = DocumentSerializer::default();
    let first = serializer.write(&sample.document).unwrap();
    let second = serializer.write(&serializer.read(&first).unwrap()).unwrap();
    assert_eq!(first, second);
}

#[test]
fn the_file_nests_children_and_omits_empty_parts() {
    let sample = SampleForm::build();
    let text = DocumentSerializer::default()
        .write(&sample.document)
        .unwrap();
    assert!(text.starts_with("{\n  \"format\": \"crisol/1\""));
    assert!(text.contains("\"children\": ["));
    assert!(text.contains("\"click\": \"save\""));
    assert!(!text.contains("\"note\""));
    assert!(text.ends_with("}\n"));
}

#[test]
fn other_format_versions_are_rejected() {
    let sample = SampleForm::build();
    let serializer = DocumentSerializer::default();
    let text = serializer
        .write(&sample.document)
        .unwrap()
        .replace("crisol/1", "crisol/9");
    assert!(matches!(
        serializer.read(&text),
        Err(ModelError::UnsupportedFormat { .. })
    ));
}
