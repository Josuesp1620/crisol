use std::ops::Range;

use super::CodeToken;

/// Colors Rust line by line: keywords, strings, comments, numbers and type names.
/// It is a quick tokenizer for reading generated code, not a parser.
pub struct RustHighlighter;

impl RustHighlighter {
    const KEYWORDS: [&'static str; 36] = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "use", "where",
    ];

    pub fn highlight(line: &str) -> Vec<(Range<usize>, CodeToken)> {
        let mut spans = Vec::new();
        let characters: Vec<(usize, char)> = line.char_indices().collect();
        let mut position = 0;
        while position < characters.len() {
            let (start, character) = characters[position];
            if character == '/'
                && characters
                    .get(position + 1)
                    .is_some_and(|next| next.1 == '/')
            {
                spans.push(CodeToken::Comment.span(start..line.len()));
                break;
            }
            if character == '"' {
                let end = Self::string_end(&characters, position);
                let end_byte = characters.get(end).map_or(line.len(), |found| found.0);
                spans.push(CodeToken::String.span(start..end_byte));
                position = end;
                continue;
            }
            if character.is_ascii_digit() {
                let end = Self::word_end(&characters, position);
                let end_byte = characters.get(end).map_or(line.len(), |found| found.0);
                spans.push(CodeToken::Number.span(start..end_byte));
                position = end;
                continue;
            }
            if character.is_alphabetic() || character == '_' {
                let end = Self::word_end(&characters, position);
                let end_byte = characters.get(end).map_or(line.len(), |found| found.0);
                let word = &line[start..end_byte];
                if Self::KEYWORDS.contains(&word) {
                    spans.push(CodeToken::Keyword.span(start..end_byte));
                } else if character.is_uppercase() {
                    spans.push(CodeToken::Type.span(start..end_byte));
                }
                position = end;
                continue;
            }
            position += 1;
        }
        spans
    }

    fn word_end(characters: &[(usize, char)], start: usize) -> usize {
        let mut end = start;
        while end < characters.len()
            && (characters[end].1.is_alphanumeric()
                || characters[end].1 == '_'
                || characters[end].1 == '.')
        {
            if characters[end].1 == '.' && !characters[start].1.is_ascii_digit() {
                break;
            }
            end += 1;
        }
        end
    }

    /// The position just after the closing quote, skipping escaped quotes.
    fn string_end(characters: &[(usize, char)], start: usize) -> usize {
        let mut end = start + 1;
        while end < characters.len() {
            match characters[end].1 {
                '\\' => end += 2,
                '"' => return end + 1,
                _ => end += 1,
            }
        }
        characters.len()
    }
}
