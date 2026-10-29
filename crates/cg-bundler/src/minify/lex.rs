//! A lexer for Rust source, as far as the minifier needs: the tokens of a
//! text with their exact spelling, leaving out comments and whitespace.
//!
//! It follows rustc's lexer for the 2021 edition. What it cannot lex is an
//! error, never a guess: an unterminated literal or comment, a quote that
//! starts neither a character nor a lifetime, an unknown character.
//! Multi-character punctuation is read greedily from a fixed set, so `&&`
//! is one token and `& &` are two.

/// What a token is. Keywords are identifiers here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An identifier or a keyword, a raw identifier (`r#type`) included.
    Ident,
    /// A lifetime or a label: `'a`, `'static`, `'_`.
    Lifetime,
    /// A character or byte character literal: `'x'`, `b'\n'`.
    Char,
    /// A string literal of any kind: `"…"`, `b"…"`, `c"…"`, `r#"…"#`,
    /// `br"…"`, `cr"…"`.
    Str,
    /// A number, with its base prefix, exponent and suffix: `0xFFu8`,
    /// `2.5E-3f32`.
    Number,
    /// One punctuation token: `;`, `::`, `..=`, `>>=`.
    Punct,
}

/// A token, spelled as in the source.
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub text: String,
    /// The line where the token starts, counted from 1.
    pub line: usize,
}

impl Token {
    /// Whether the two tokens have the same kind and text (lines aside).
    pub fn same(&self, other: &Token) -> bool {
        self.kind == other.kind && self.text == other.text
    }
}

/// The index of the first token where `a` and `b` differ, which is the
/// length of the shorter when one is a prefix of the other, or `None` when
/// they have the same kinds and texts.
pub fn first_difference(a: &[Token], b: &[Token]) -> Option<usize> {
    let common = a.len().min(b.len());
    (0..common)
        .find(|&i| !a[i].same(&b[i]))
        .or((a.len() != b.len()).then_some(common))
}

/// The tokens of `source`, in order, without comments and whitespace.
pub fn tokens(source: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer {
        chars: source.chars().collect(),
        pos: 0,
        line: 1,
    };
    let mut out = Vec::new();
    while let Some(c) = lexer.peek(0) {
        if is_space(c) {
            lexer.bump();
        } else if c == '/' && lexer.peek(1) == Some('/') {
            while lexer.peek(0).is_some_and(|c| c != '\n') {
                lexer.bump();
            }
        } else if c == '/' && lexer.peek(1) == Some('*') {
            lexer.block_comment()?;
        } else {
            let (start, line) = (lexer.pos, lexer.line);
            let kind = lexer.token(c)?;
            out.push(Token {
                kind,
                text: lexer.chars[start..lexer.pos].iter().collect(),
                line,
            });
        }
    }
    Ok(out)
}

/// Punctuation of three characters, then of two, longest first.
const PUNCT_3: [&str; 4] = ["<<=", ">>=", "...", "..="];
const PUNCT_2: [&str; 21] = [
    "::", "->", "<-", "=>", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "^=",
    "&=", "|=", "<<", ">>", "..",
];
/// Punctuation of one character.
const PUNCT_1: &str = "!#$%&()*+,-./:;<=>?@[]^{|}~";

struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    fn peek(&self, ahead: usize) -> Option<char> {
        self.chars.get(self.pos + ahead).copied()
    }

    /// Moves past one character, if any.
    fn bump(&mut self) {
        if self.peek(0) == Some('\n') {
            self.line += 1;
        }
        self.pos = (self.pos + 1).min(self.chars.len());
    }

    fn bump_n(&mut self, n: usize) {
        for _ in 0..n {
            self.bump();
        }
    }

    fn error<T>(line: usize, what: &str) -> Result<T, String> {
        Err(format!("line {line}: {what}"))
    }

    /// Skips a block comment, nested ones included; `/*` is next.
    fn block_comment(&mut self) -> Result<(), String> {
        let line = self.line;
        let mut depth = 0;
        while let Some(c) = self.peek(0) {
            match (c, self.peek(1)) {
                ('/', Some('*')) => {
                    depth += 1;
                    self.bump_n(2);
                }
                ('*', Some('/')) => {
                    depth -= 1;
                    self.bump_n(2);
                    if depth == 0 {
                        return Ok(());
                    }
                }
                _ => self.bump(),
            }
        }
        Self::error(line, "unterminated block comment")
    }

    /// Reads the token that starts with `c` and returns its kind.
    fn token(&mut self, c: char) -> Result<Kind, String> {
        let line = self.line;
        match c {
            '"' => self.string().map(|()| Kind::Str),
            '\'' => self.quote(),
            '0'..='9' => {
                self.number();
                Ok(Kind::Number)
            }
            _ if is_ident_start(c) => self.word(),
            _ => {
                let rest: String = (0..3).filter_map(|i| self.peek(i)).collect();
                let len = PUNCT_3
                    .iter()
                    .chain(&PUNCT_2)
                    .find(|p| rest.starts_with(**p))
                    .map_or(1, |p| p.len());
                if len == 1 && !PUNCT_1.contains(c) {
                    return Self::error(line, &format!("cannot lex the character {c:?}"));
                }
                self.bump_n(len);
                Ok(Kind::Punct)
            }
        }
    }

    /// An identifier, or a literal that starts like one: `b'x'`, `b"x"`,
    /// `c"x"`, `r"x"`, `r#"x"#`, `br"x"`, `cr#"x"#`, or a raw identifier.
    fn word(&mut self) -> Result<Kind, String> {
        let line = self.line;
        let first = self.peek(0);
        let second = self.peek(1);
        if matches!(first, Some('b' | 'c')) && second == Some('"') {
            self.bump();
            return self.string().map(|()| Kind::Str);
        }
        if first == Some('b') && second == Some('\'') {
            self.bump();
            return self.char_literal().map(|()| Kind::Char);
        }
        // A raw string starts at `r`, or after `b` or `c`.
        let raw = match (first, second) {
            (Some('r'), _) => Some(0),
            (Some('b' | 'c'), Some('r')) => Some(1),
            _ => None,
        };
        if let Some(offset) = raw {
            let mut hashes = 0;
            while self.peek(offset + 1 + hashes) == Some('#') {
                hashes += 1;
            }
            if self.peek(offset + 1 + hashes) == Some('"') {
                self.bump_n(offset + 2 + hashes);
                return self.raw_string(hashes, line).map(|()| Kind::Str);
            }
            if offset == 0 && hashes == 1 && self.peek(2).is_some_and(is_ident_start) {
                self.bump_n(2);
                self.ident_rest();
                return Ok(Kind::Ident);
            }
            if offset == 0 && hashes > 0 {
                return Self::error(
                    line,
                    "`r#` starts neither a raw string nor a raw identifier",
                );
            }
        }
        self.ident_rest();
        Ok(Kind::Ident)
    }

    fn ident_rest(&mut self) {
        while self.peek(0).is_some_and(is_ident_continue) {
            self.bump();
        }
    }

    /// A string literal; the opening quote is next.
    fn string(&mut self) -> Result<(), String> {
        let line = self.line;
        self.bump();
        while let Some(c) = self.peek(0) {
            match c {
                '\\' => self.bump_n(2),
                '"' => {
                    self.bump();
                    return Ok(());
                }
                _ => self.bump(),
            }
        }
        Self::error(line, "unterminated string literal")
    }

    /// The rest of a raw string, after its opening quote, that closes with
    /// `hashes` hashes.
    fn raw_string(&mut self, hashes: usize, line: usize) -> Result<(), String> {
        while let Some(c) = self.peek(0) {
            if c == '"' && (1..=hashes).all(|k| self.peek(k) == Some('#')) {
                self.bump_n(1 + hashes);
                return Ok(());
            }
            self.bump();
        }
        Self::error(line, "unterminated raw string literal")
    }

    /// A quote: a character literal or a lifetime.
    fn quote(&mut self) -> Result<Kind, String> {
        let line = self.line;
        match self.peek(1) {
            Some('\\') => self.char_literal().map(|()| Kind::Char),
            Some(c) if c != '\'' && self.peek(2) == Some('\'') => {
                self.char_literal().map(|()| Kind::Char)
            }
            Some(c) if is_ident_start(c) => {
                self.bump();
                // A raw lifetime: `'r#a`.
                if c == 'r' && self.peek(1) == Some('#') && self.peek(2).is_some_and(is_ident_start)
                {
                    self.bump_n(2);
                }
                self.ident_rest();
                Ok(Kind::Lifetime)
            }
            _ => Self::error(
                line,
                "a quote that starts neither a character nor a lifetime",
            ),
        }
    }

    /// A character literal; the opening quote is next.
    fn char_literal(&mut self) -> Result<(), String> {
        let line = self.line;
        self.bump();
        match self.peek(0) {
            Some('\\') => {
                self.bump();
                let escape = self.peek(0);
                self.bump();
                match escape {
                    Some('u') if self.peek(0) == Some('{') => {
                        while self.peek(0).is_some_and(|c| c != '}' && c != '\'') {
                            self.bump();
                        }
                        if self.peek(0) == Some('}') {
                            self.bump();
                        }
                    }
                    Some('x') => self.bump_n(2),
                    _ => {}
                }
            }
            Some(c) if c != '\'' => self.bump(),
            _ => {}
        }
        if self.peek(0) == Some('\'') {
            self.bump();
            Ok(())
        } else {
            Self::error(line, "a malformed character literal")
        }
    }

    /// A number; a digit is next. A fraction needs a dot that is not a
    /// range (`1..2`) or a method call (`1.max(2)`); the suffix is whatever
    /// identifier characters follow.
    fn number(&mut self) {
        let base = match (self.peek(0), self.peek(1)) {
            (Some('0'), Some(b @ ('b' | 'o' | 'x'))) => Some(b),
            _ => None,
        };
        if let Some(base) = base {
            let radix = if base == 'x' { 16 } else { 10 };
            self.bump_n(2);
            while self.peek(0).is_some_and(|c| c == '_' || c.is_digit(radix)) {
                self.bump();
            }
        } else {
            self.digits();
            if self.peek(0) == Some('.')
                && self.peek(1) != Some('.')
                && !self.peek(1).is_some_and(is_ident_start)
            {
                self.bump();
                self.digits();
            }
            self.exponent();
        }
        self.ident_rest();
    }

    fn digits(&mut self) {
        while self.peek(0).is_some_and(|c| c == '_' || c.is_ascii_digit()) {
            self.bump();
        }
    }

    /// An exponent (`e5`, `E-3`, `e+_1`), when digits follow the `e`.
    fn exponent(&mut self) {
        if !matches!(self.peek(0), Some('e' | 'E')) {
            return;
        }
        let mut ahead = 1;
        if matches!(self.peek(ahead), Some('+' | '-')) {
            ahead += 1;
        }
        while self.peek(ahead) == Some('_') {
            ahead += 1;
        }
        if self.peek(ahead).is_some_and(|c| c.is_ascii_digit()) {
            self.bump_n(ahead);
            self.digits();
        }
    }
}

/// Rust's pattern whitespace.
fn is_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n'
            | '\u{B}'
            | '\u{C}'
            | '\r'
            | ' '
            | '\u{85}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{2028}'
            | '\u{2029}'
    )
}

/// Whether `c` can start an identifier. Non-ASCII letters count, though
/// rustc's rule (XID_Start) is narrower: the compiler has the last word.
pub(super) fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic() || (!c.is_ascii() && c.is_alphabetic())
}

/// Whether `c` can continue an identifier.
pub(super) fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric() || (!c.is_ascii() && c.is_alphanumeric())
}

#[cfg(test)]
mod tests;
