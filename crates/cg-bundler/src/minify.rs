//! Shrinks a bundle by removing what the compiler ignores: comments,
//! indentation, trailing spaces and blank lines (ADR 0016).
//!
//! The text is scanned token by token so that string, byte string, raw
//! string and character literals are copied unchanged, even when they span
//! several lines or contain `//`. Each remaining line of code keeps its own
//! line, so compiler errors still point at a readable line.

/// `source` without comments (doc comments included), indentation,
/// trailing spaces or blank lines. Literals are left as they are.
pub fn strip(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    // Where the current output line starts.
    let mut line_start = 0;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        match c {
            '/' if next == Some('/') => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            '/' if next == Some('*') => {
                i = block_comment_end(&chars, i);
                // Keep the tokens on either side apart.
                if out.len() > line_start && !out.ends_with([' ', '\t']) {
                    out.push(' ');
                }
            }
            '\n' => {
                end_line(&mut out, &mut line_start);
                i += 1;
            }
            ' ' | '\t' | '\r' => {
                if out.len() > line_start {
                    out.push(c);
                }
                i += 1;
            }
            '"' => i = copy(&chars, i, string_end(&chars, i), &mut out),
            '\'' => match char_literal_end(&chars, i) {
                Some(end) => i = copy(&chars, i, end, &mut out),
                // A lifetime or a label.
                None => {
                    out.push(c);
                    i += 1;
                }
            },
            'r' | 'b' | 'c' if starts_token(&chars, i) => match raw_string_end(&chars, i) {
                Some(end) => i = copy(&chars, i, end, &mut out),
                None => {
                    out.push(c);
                    i += 1;
                }
            },
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    end_line(&mut out, &mut line_start);
    out
}

/// Ends the current output line: drops its trailing spaces, and the line
/// itself when nothing is left on it.
fn end_line(out: &mut String, line_start: &mut usize) {
    let kept = out.trim_end_matches([' ', '\t', '\r']).len();
    out.truncate(kept.max(*line_start));
    if out.len() > *line_start {
        out.push('\n');
        *line_start = out.len();
    }
}

/// Copies `chars[start..end]` to `out` and returns `end`.
fn copy(chars: &[char], start: usize, end: usize, out: &mut String) -> usize {
    out.extend(&chars[start..end]);
    end
}

/// Whether `chars[i]` cannot continue an identifier or a number.
fn starts_token(chars: &[char], i: usize) -> bool {
    i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_')
}

/// The index just past the block comment starting at `start`, with nested
/// comments, or the end of the text.
fn block_comment_end(chars: &[char], start: usize) -> usize {
    let mut depth = 0;
    let mut i = start;
    while i < chars.len() {
        match (chars[i], chars.get(i + 1)) {
            ('/', Some('*')) => {
                depth += 1;
                i += 2;
            }
            ('*', Some('/')) => {
                depth -= 1;
                i += 2;
                if depth == 0 {
                    return i;
                }
            }
            _ => i += 1,
        }
    }
    chars.len()
}

/// The index just past the string literal whose opening quote is at
/// `start`, or the end of the text.
fn string_end(chars: &[char], start: usize) -> usize {
    let mut i = start + 1;
    while i < chars.len() {
        match chars[i] {
            '\\' => i += 2,
            '"' => return i + 1,
            _ => i += 1,
        }
    }
    chars.len()
}

/// The index just past the character literal starting at `start`, or
/// `None` when the quote starts a lifetime or a label.
fn char_literal_end(chars: &[char], start: usize) -> Option<usize> {
    match chars.get(start + 1)? {
        '\\' => {
            // An escape: `'\n'`, `'\''`, `'\u{1F600}'`.
            let mut i = start + 3;
            while i < chars.len() {
                if chars[i] == '\'' {
                    return Some(i + 1);
                }
                i += 1;
            }
            Some(chars.len())
        }
        _ if chars.get(start + 2) == Some(&'\'') => Some(start + 3),
        _ => None,
    }
}

/// The index just past the raw string (`r"…"`, `r#"…"#`, `br"…"`, `cr"…"`)
/// starting at `start`, or `None` when no raw string starts there, as in
/// `r#type` or `bytes`.
fn raw_string_end(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    if chars[i] != 'r' {
        i += 1;
        if chars.get(i) != Some(&'r') {
            return None;
        }
    }
    i += 1;
    let mut hashes = 0;
    while chars.get(i) == Some(&'#') {
        hashes += 1;
        i += 1;
    }
    if chars.get(i) != Some(&'"') {
        return None;
    }
    i += 1;
    while i < chars.len() {
        if chars[i] == '"' && (1..=hashes).all(|k| chars.get(i + k) == Some(&'#')) {
            return Some(i + 1 + hashes);
        }
        i += 1;
    }
    Some(chars.len())
}

#[cfg(test)]
mod tests;
