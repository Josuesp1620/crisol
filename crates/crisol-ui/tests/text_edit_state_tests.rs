use crisol_ui::components::text_box::{InputKind, TextEditState};

fn state_with(text: &str, kind: InputKind, max_length: usize) -> TextEditState {
    let mut state = TextEditState::new(kind, max_length);
    state.set_content(text);
    state
}

#[test]
fn typing_replaces_the_selection_and_moves_the_caret() {
    let mut state = state_with("hello", InputKind::Text, 0);
    state.set_selection(1..4);
    assert!(state.replace(None, "ipp"));
    assert_eq!(state.content(), "hippo");
    assert_eq!(state.selection(), 4..4);
}

#[test]
fn backspacing_removes_whole_graphemes() {
    let mut state = state_with("año👍🏽", InputKind::Text, 0);
    let previous = state.previous_boundary(state.cursor());
    state.select_to(previous);
    state.replace(None, "");
    assert_eq!(state.content(), "año");
}

#[test]
fn the_maximum_length_counts_characters() {
    let mut state = state_with("2012345678", InputKind::Text, 11);
    state.replace(None, "999");
    assert_eq!(state.content(), "20123456789");
}

#[test]
fn number_fields_drop_letters() {
    let mut state = state_with("", InputKind::Number, 0);
    state.replace(None, "12a3.5x");
    assert_eq!(state.content(), "123.5");
}

#[test]
fn single_line_fields_drop_line_breaks() {
    let mut state = state_with("", InputKind::Email, 0);
    state.replace(None, "a@b.c\nd");
    assert_eq!(state.content(), "a@b.cd");
}

#[test]
fn selecting_backwards_keeps_the_anchor() {
    let mut state = state_with("abcdef", InputKind::Text, 0);
    state.move_to(4);
    state.select_to(1);
    assert_eq!(state.selection(), 1..4);
    assert_eq!(state.cursor(), 1);
    assert_eq!(state.selected_text(), "bcd");
}

#[test]
fn utf16_offsets_round_trip() {
    let state = state_with("a😀b", InputKind::Text, 0);
    assert_eq!(state.offset_to_utf16(5), 3);
    assert_eq!(state.offset_from_utf16(3), 5);
    assert_eq!(state.utf16_length(), 4);
}

#[test]
fn composition_marks_the_inserted_text() {
    let mut state = state_with("ab", InputKind::Text, 0);
    state.move_to(1);
    state.replace_and_mark(None, "ñ", None);
    assert_eq!(state.content(), "añb");
    assert_eq!(state.marked(), Some(1..3));
    state.replace(None, "n");
    assert_eq!(state.content(), "anb");
    assert_eq!(state.marked(), None);
}
