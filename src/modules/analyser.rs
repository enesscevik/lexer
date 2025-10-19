#[derive(Debug)]
pub enum CharKind {
    Whitespace,
    Char(char),
    Newline,
}

#[derive(Debug)]
pub struct AnalyzedChar {
    pub kind: CharKind,
    pub line: usize,
    pub column: usize,
}

pub fn analyze_chars(input: &str) -> Vec<AnalyzedChar> {
    let mut result = Vec::new();
    let mut line = 1;
    let mut column = 1;
    //let mut comment_one = false;
    for ch in input.chars() {
        let kind = match ch {
            '\n' => CharKind::Newline,
            c if c.is_whitespace() => CharKind::Whitespace,
            c => CharKind::Char(c),
        };
        result.push(AnalyzedChar { kind, line, column });
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    result
}
