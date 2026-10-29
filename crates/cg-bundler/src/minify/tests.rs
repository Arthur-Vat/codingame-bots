use super::{first_difference, minify, tokens, HARD_LINE, SOFT_LINE};

/// The minified `source`, without its final newline.
fn min(source: &str) -> String {
    let out = minify(source).unwrap_or_else(|err| panic!("{source:?}: {err}"));
    assert!(out.ends_with('\n'), "{out:?}");
    out.trim_end_matches('\n').to_string()
}

#[test]
fn only_the_needed_whitespace_stays() {
    let source = "//! Crate doc.\n\
                  \n\
                  /// An item doc.\n\
                  fn main() {\n    \
                      // A comment line.\n    \
                      let x = 1; // A trailing comment.   \n\
                  \n    \
                      println!(\"{x}\");\t\n\
                  }\n";
    assert_eq!(min(source), "fn main(){let x=1;println!(\"{x}\");}");
    assert_eq!(min("let a/* x /* y */ z */= b;"), "let a=b;");
    assert_eq!(min("a/* c */b"), "a b");
    assert_eq!(
        min("fn f<T: Copy>(x: T) -> T { x }"),
        "fn f<T:Copy>(x:T)->T{x}"
    );
    assert_eq!(
        min("use std::io::{self, Write};"),
        "use std::io::{self,Write};"
    );
}

#[test]
fn literals_are_copied_byte_for_byte() {
    let cases = [
        r#"let url = "https://example.com // not a comment";"#,
        r#"let s = "a \" quote /* and */ more";"#,
        r##"let raw = r#"raw " // /* "#;"##,
        r#"let raw = r"C:\path // x";"#,
        r##"let bytes = br#"b // c"#;"##,
        r#"let bytes = b"x // y";"#,
        r#"let c = c"x";"#,
        r"let slash = '/'; let quote = '\''; let backslash = '\\';",
        r#"let dq = '"'; let s = "after";"#,
        r"let star = '*'; let unicode = '\u{1F600}'; let e = 'é';",
        r#"let b = b'/'; let b2 = b'"';"#,
        "let packed = \"\u{4E00}\u{1F600}\u{7FF} \u{0} \u{FFFD}\";",
    ];
    for case in cases {
        let literals = |text: &str| -> Vec<String> {
            tokens(text)
                .unwrap()
                .into_iter()
                .filter(|t| matches!(t.kind, super::Kind::Str | super::Kind::Char))
                .map(|t| t.text)
                .collect()
        };
        let out = min(&format!("{case} // gone"));
        assert_eq!(literals(&out), literals(case), "{case}");
        for literal in literals(case) {
            assert!(out.contains(&literal), "{literal} in {out}");
        }
    }
}

#[test]
fn literals_spanning_lines_stay_as_they_are() {
    let source = "let s = \"first\n    second // kept\n\n  third\";\n    next();\n";
    assert_eq!(
        min(source),
        "let s=\"first\n    second // kept\n\n  third\";next();"
    );
    let raw = "let r = r#\"\n  /* kept */\n\"#;\n";
    assert_eq!(min(raw), "let r=r#\"\n  /* kept */\n\"#;");
}

#[test]
fn lifetimes_labels_and_raw_identifiers_are_code() {
    let source = "fn f<'a>(x: &'a str) -> &'a str { 'outer: loop { break 'outer; } x } // c\n\
                  let r#type = r#match; let rb = br; let cr = c;\n";
    assert_eq!(
        min(source),
        "fn f<'a>(x:&'a str)->&'a str{'outer:loop{break 'outer;}x}\
         let r#type=r#match;let rb=br;let cr=c;"
    );
    assert_eq!(min("'a: loop { break 'a; }"), "'a:loop{break 'a;}");
    assert_eq!(min("f('a', 'b, 'c')"), "f('a','b,'c')");
}

#[test]
fn numbers_keep_what_makes_them_one_token() {
    assert_eq!(min("for i in 1 .. 2 {}"), "for i in 1..2{}");
    assert_eq!(min("1 .. = 2"), "1.. =2");
    assert_eq!(min("let a = 1.max(2);"), "let a=1 .max(2);");
    assert_eq!(min("x.0.1"), "x.0.1");
    assert_eq!(min("let a = 1. ;"), "let a=1.;");
    assert_eq!(
        min("let a = [0x10u8, 0b1, 1_000u64, 2.5E-3f32, 1e5];"),
        "let a=[0x10u8,0b1,1_000u64,2.5E-3f32,1e5];"
    );
    assert_eq!(min("a - 1"), "a-1");
    assert_eq!(min("1 as u8"), "1 as u8");
}

#[test]
fn reserved_prefixes_and_suffixes_are_kept_apart() {
    assert_eq!(min("return \"x\""), "return \"x\"");
    assert_eq!(min("b \"x\""), "b \"x\"");
    assert_eq!(min("b 'x'"), "b 'x'");
    assert_eq!(min("r \"x\""), "r \"x\"");
    assert_eq!(min("r #\"x\"#"), "r #\"x\"#");
    assert_eq!(min("cr #x"), "cr #x");
    assert_eq!(min("1 \"x\""), "1 \"x\"");
    assert_eq!(min("\"x\" y"), "\"x\" y");
    assert_eq!(min("'a' y"), "'a' y");
    assert_eq!(min("\"x\" 'a"), "\"x\"'a");
    assert_eq!(min("f(\"x\", 'a')"), "f(\"x\",'a')");
    assert_eq!(
        min("if x == \"a\" { 'b' } else { 'c' }"),
        "if x==\"a\"{'b'}else{'c'}"
    );
    assert_eq!(min("# [derive(Debug)]"), "#[derive(Debug)]");
}

#[test]
fn punctuation_that_would_merge_stays_apart() {
    assert_eq!(min("a / *b"), "a/ *b");
    assert_eq!(min("a / /b"), "a/ /b");
    assert_eq!(min("a / *b / c"), "a/ *b/c");
    assert_eq!(min("& &x"), "& &x");
    assert_eq!(min("&&x"), "&&x");
    assert_eq!(min("x & &y"), "x& &y");
    assert_eq!(min("a > > b"), "a> >b");
    assert_eq!(min("Vec<Vec<u8> >"), "Vec<Vec<u8> >");
    assert_eq!(min("- -x"), "--x");
    assert_eq!(min("a < -x"), "a< -x");
    assert_eq!(min("x >> = 1"), "x>> =1");
    assert_eq!(min("a < = b"), "a< =b");
    assert_eq!(min("| | 1"), "| |1");
    assert_eq!(min("|a| |b| a"), "|a| |b|a");
    assert_eq!(min("a : : b"), "a: :b");
    assert_eq!(min("a = > b"), "a= >b");
    assert_eq!(min("x . . y"), "x. .y");
    assert_eq!(min("x .. = y"), "x.. =y");
    assert_eq!(min("x . . . y"), "x. . .y");
    assert_eq!(min("a & & b"), "a& &b");
}

#[test]
fn macros_keep_their_tokens() {
    let source = "macro_rules! m {\n    ($x:expr) => { $x + 1 };\n    ($($y:ident),* $(,)?) => { $( let $y = 0; )* };\n}\n";
    assert_eq!(
        min(source),
        "macro_rules!m{($x:expr)=>{$x+1};($($y:ident),*$(,)?)=>{$(let$y=0;)*};}"
    );
}

#[test]
fn empty_and_comment_only_texts_become_empty() {
    assert_eq!(minify("").unwrap(), "");
    assert_eq!(minify("// only\n\n/* a */\n").unwrap(), "");
    assert_eq!(minify("   \n").unwrap(), "");
}

#[test]
fn unlexable_text_fails() {
    assert!(minify("let s = \"open").unwrap_err().contains("line 1"));
    assert!(minify("fn f() {} /* open").is_err());
    assert!(minify("let c = 'ab';").is_err());
}

#[test]
fn minifying_twice_changes_nothing() {
    let source =
        "/// Doc.\nfn main() {\n    let s = \"// a\n  b\"; // c\n    let t = 1 .max(2);\n}\n";
    let once = minify(source).unwrap();
    assert_eq!(minify(&once).unwrap(), once);
}

#[test]
fn the_result_has_the_tokens_of_the_input() {
    let source = "fn f<'a, T: Into<&'a str>>(x: &'a [T; 3]) -> Option<&'a T> {\n\
                  let r#loop = 1.0e-3f32 + x.0.1 as f32; // c\n\
                  let y = &&x[..=2]; let z = -(-1); let w = a >> 1 >= b;\n\
                  match c { 'a'..='z' => b'\\n', _ => b'x' }\n}\n";
    let out = minify(source).unwrap();
    assert_eq!(
        first_difference(&tokens(source).unwrap(), &tokens(&out).unwrap()),
        None
    );
    assert!(out.len() < source.len());
}

#[test]
fn long_lines_wrap_at_whitespace_after_the_soft_limit() {
    // 100 statements of `let x = 1;`: whitespace is needed after `let` and
    // after `x` only.
    let source = "let x = 1;\n".repeat(100);
    let out = minify(&source).unwrap();
    assert!(out.lines().count() > 1, "{out}");
    for line in out.lines() {
        assert!(
            line.chars().count() < SOFT_LINE + "let x=1;".len() + 2,
            "{line}"
        );
    }
    assert!(out
        .lines()
        .all(|l| !l.starts_with(' ') && !l.ends_with(' ')));
    assert_eq!(
        first_difference(&tokens(&source).unwrap(), &tokens(&out).unwrap()),
        None
    );
    // A line that needs no whitespace is cut before the next token.
    let source = "f(".to_string() + &"a,".repeat(400) + ")";
    let out = minify(&source).unwrap();
    assert!(out.lines().count() > 2);
    for line in out.lines() {
        assert!(
            line.chars().count() <= HARD_LINE + 2,
            "{} chars",
            line.chars().count()
        );
    }
    assert_eq!(
        first_difference(&tokens(&source).unwrap(), &tokens(&out).unwrap()),
        None
    );
}

#[test]
fn short_texts_stay_on_one_line() {
    assert_eq!(
        minify("let x = 1;\nlet y = 2;\n").unwrap(),
        "let x=1;let y=2;\n"
    );
}

#[test]
fn a_literal_spanning_lines_resets_the_line_length() {
    let literal = format!("\"{}\n{}\"", "a".repeat(400), "b".repeat(10));
    let source = format!("x = {literal};y = 1;");
    assert_eq!(min(&source), format!("x={literal};y=1;"));
}
