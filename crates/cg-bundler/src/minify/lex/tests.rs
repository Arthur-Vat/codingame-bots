use super::Kind::{Char, Ident, Lifetime, Number, Str};
use super::{first_difference, tokens, Kind};

/// The kinds and texts of the tokens of `source`.
fn lex(source: &str) -> Vec<(Kind, String)> {
    tokens(source)
        .unwrap_or_else(|err| panic!("{source:?}: {err}"))
        .into_iter()
        .map(|t| (t.kind, t.text))
        .collect()
}

/// The texts of the tokens of `source`.
fn texts(source: &str) -> Vec<String> {
    lex(source).into_iter().map(|(_, text)| text).collect()
}

/// `source` must lex as one token of `kind` with the whole text.
fn single(kind: Kind, source: &str) {
    assert_eq!(lex(source), vec![(kind, source.to_string())], "{source}");
}

#[test]
fn identifiers_keywords_and_raw_identifiers() {
    assert_eq!(
        lex("fn r#type _x é2 br cr rb"),
        [
            (Ident, "fn"),
            (Ident, "r#type"),
            (Ident, "_x"),
            (Ident, "é2"),
            (Ident, "br"),
            (Ident, "cr"),
            (Ident, "rb"),
        ]
        .map(|(kind, text)| (kind, text.to_string()))
    );
}

#[test]
fn lifetimes_and_labels_are_not_characters() {
    single(Lifetime, "'a");
    single(Lifetime, "'static");
    single(Lifetime, "'_");
    assert_eq!(texts("&'a str"), ["&", "'a", "str"]);
    assert_eq!(texts("<'a,'b>"), ["<", "'a", ",", "'b", ">"]);
    assert_eq!(texts("'outer: loop"), ["'outer", ":", "loop"]);
}

#[test]
fn character_and_byte_character_literals() {
    for source in [
        "'a'",
        "'é'",
        "' '",
        "'\"'",
        "'\\n'",
        "'\\''",
        "'\\\\'",
        "'\\x41'",
        "'\\u{1F600}'",
        "'😀'",
    ] {
        single(Char, source);
    }
    for source in ["b'x'", "b'\\n'", "b'\\''", "b'\\xFF'", "b'/'"] {
        single(Char, source);
    }
    assert_eq!(
        lex("'a' 'a"),
        [(Char, "'a'".to_string()), (Lifetime, "'a".to_string())]
    );
}

#[test]
fn strings_of_every_kind() {
    for source in [
        r#""""#,
        r#""a \" // not a comment""#,
        "\"two\nlines\"",
        "\"a\\\n   continued\"",
        r#"b"bytes""#,
        r#"c"cstr""#,
        r#"r"raw \ ""#,
        r##"r#"raw " quote"#"##,
        r###"r##"a "# b"##"###,
        r###"br##"x"##"###,
        r#"br"x""#,
        r#"cr"x""#,
        r##"cr#"x"#"##,
        "\"packed \u{4E00}\u{1F600}\u{0}\"",
    ] {
        single(Str, source);
    }
    assert_eq!(texts(r#""a" "b""#), [r#""a""#, r#""b""#]);
    assert_eq!(
        lex("b \"x\""),
        [(Ident, "b".to_string()), (Str, "\"x\"".to_string())]
    );
}

#[test]
fn numbers() {
    for source in [
        "1",
        "1_000u64",
        "0xFF",
        "0xFF_u8",
        "0b1",
        "0b1010_1010u8",
        "0o777",
        "1.5",
        "1.",
        "1e5",
        "1E+5",
        "2.5E-3f32",
        "1_0.0_1e1_0f64",
        "0.1",
        "7usize",
        "1f32",
    ] {
        single(Number, source);
    }
}

#[test]
fn ranges_methods_and_tuple_indices_are_not_fractions() {
    assert_eq!(texts("1..2"), ["1", "..", "2"]);
    assert_eq!(texts("1..=2"), ["1", "..=", "2"]);
    assert_eq!(texts("1.max(2)"), ["1", ".", "max", "(", "2", ")"]);
    assert_eq!(texts("1.0.max(2.0)"), ["1.0", ".", "max", "(", "2.0", ")"]);
    assert_eq!(texts("x.0.1"), ["x", ".", "0.1"]);
    assert_eq!(
        texts("x.0 .max(1)"),
        ["x", ".", "0", ".", "max", "(", "1", ")"]
    );
    assert_eq!(texts("1. ;"), ["1.", ";"]);
    assert_eq!(texts("1.;"), ["1.", ";"]);
    assert_eq!(texts("1e"), ["1e"]);
}

#[test]
fn punctuation_is_greedy() {
    assert_eq!(
        texts(":: -> <- => == != <= >= && || += -= *= /= %= ^= &= |= << >> <<= >>= .. ... ..="),
        [
            "::", "->", "<-", "=>", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=",
            "%=", "^=", "&=", "|=", "<<", ">>", "<<=", ">>=", "..", "...", "..="
        ]
    );
    assert_eq!(texts("& &x"), ["&", "&", "x"]);
    assert_eq!(texts("&&x"), ["&&", "x"]);
    assert_eq!(texts("> >"), [">", ">"]);
    assert_eq!(texts(">>="), [">>="]);
    assert_eq!(texts("- -x"), ["-", "-", "x"]);
    assert_eq!(texts("<-x"), ["<-", "x"]);
    assert_eq!(texts("#![a]"), ["#", "!", "[", "a", "]"]);
    assert_eq!(texts("$x:expr"), ["$", "x", ":", "expr"]);
    assert_eq!(
        texts("$($x:tt)*"),
        ["$", "(", "$", "x", ":", "tt", ")", "*"]
    );
}

#[test]
fn comments_and_whitespace_are_skipped() {
    let source =
        "//! Crate doc.\n/// Item doc.\nfn /* a /* nested */ b */ main() // end\n{}\r\n\t/**/ x";
    assert_eq!(texts(source), ["fn", "main", "(", ")", "{", "}", "x"]);
    assert_eq!(texts("a/**/b"), ["a", "b"]);
    assert_eq!(texts("a //b\nc"), ["a", "c"]);
    assert_eq!(texts("\"//\" '/' b'/'"), ["\"//\"", "'/'", "b'/'"]);
    assert_eq!(texts("/ /"), ["/", "/"]);
}

#[test]
fn empty_and_comment_only_inputs_have_no_tokens() {
    assert!(lex("").is_empty());
    assert!(lex("  \n\t").is_empty());
    assert!(lex("// only\n\n/* a */ /// b\n//! c").is_empty());
    assert!(lex("// no newline at the end").is_empty());
}

#[test]
fn lines_are_counted_from_one_where_tokens_start() {
    let found = tokens("a\n/* x\n y */ \"s\n\" b\n\nc").unwrap();
    let lines: Vec<(&str, usize)> = found.iter().map(|t| (t.text.as_str(), t.line)).collect();
    assert_eq!(lines, [("a", 1), ("\"s\n\"", 3), ("b", 4), ("c", 6)]);
}

#[test]
fn what_cannot_be_lexed_is_an_error() {
    for source in [
        "\"open",
        "\"open \\",
        "r#\"open\"",
        "r##\"open\"#",
        "br\"open",
        "/* open",
        "/* /* inner */",
        "'",
        "'ab' ",
        "'1",
        "'\\n",
        "b'ab'",
        "b'",
        "r#",
        "r##x",
        "x \\ y",
        "a ` b",
    ] {
        assert!(tokens(source).is_err(), "{source:?} should not lex");
    }
    let err = tokens("a\n\n\"open").unwrap_err();
    assert!(err.contains("line 3"), "{err}");
}

#[test]
fn first_difference_names_the_index() {
    let a = tokens("a b c").unwrap();
    assert_eq!(
        first_difference(&a, &tokens("a  /* x */ b c").unwrap()),
        None
    );
    assert_eq!(first_difference(&a, &tokens("a x c").unwrap()), Some(1));
    assert_eq!(first_difference(&a, &tokens("a b").unwrap()), Some(2));
    assert_eq!(first_difference(&a, &tokens("a b c d").unwrap()), Some(3));
    // A character is not a lifetime.
    assert_eq!(
        first_difference(&tokens("'a'").unwrap(), &tokens("'a").unwrap()),
        Some(0)
    );
}
