pub fn tokenize(text: &str) -> Vec<&str> {
    text.split(Splitter::is_match)
        .filter(|s| !s.is_empty())
        .collect()
}

struct Splitter;

impl Splitter {
    fn is_match(c: char) -> bool {
        matches!(
            c,
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' | '"' | ':' | '\t' | '\n' | '(' | ')' | '-'
        )
    }
}
