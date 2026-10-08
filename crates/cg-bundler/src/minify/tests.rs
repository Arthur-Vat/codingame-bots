use super::strip;

#[test]
fn comments_indentation_and_blank_lines_go() {
    let source = "//! Crate doc.\n\
                  \n\
                  /// An item doc.\n\
                  fn main() {\n    \
                      // A comment line.\n    \
                      let x = 1; // A trailing comment.   \n\
                  \n    \
                      println!(\"{x}\");\t\n\
                  }\n";
    assert_eq!(
        strip(source),
        "fn main() {\nlet x = 1;\nprintln!(\"{x}\");\n}\n"
    );
}

#[test]
fn block_comments_go_even_nested_and_keep_tokens_apart() {
    assert_eq!(strip("let a/* x /* y */ z */= b;\n"), "let a = b;\n");
    assert_eq!(strip("a/* c */b\n"), "a b\n");
    assert_eq!(strip("    /* leading */ x\n"), "x\n");
    assert_eq!(strip("x /* trailing */\n"), "x\n");
    assert_eq!(strip("/*\n * many\n * lines\n */\ny\n"), "y\n");
}

#[test]
fn literals_are_copied_unchanged() {
    let cases = [
        r#"let url = "https://example.com // not a comment";"#,
        r#"let s = "a \" quote /* and */ more";"#,
        r##"let raw = r#"raw " // /* "#;"##,
        r#"let raw = r"C:\path // x";"#,
        r##"let bytes = br#"b // c"#;"##,
        r#"let bytes = b"x // y";"#,
        r"let slash = '/'; let quote = '\''; let backslash = '\\';",
        r#"let dq = '"'; let s = "after";"#,
        r"let star = '*'; let unicode = '\u{1F600}'; let e = 'é';",
        r#"let b = b'/'; let b2 = b'"';"#,
    ];
    for case in cases {
        let source = format!("{case} // gone\n");
        assert_eq!(strip(&source), format!("{case}\n"), "{case}");
    }
}

#[test]
fn literals_spanning_lines_keep_their_spaces() {
    let source = "let s = \"first\n    second // kept\n\n  third\";\n    next();\n";
    assert_eq!(
        strip(source),
        "let s = \"first\n    second // kept\n\n  third\";\nnext();\n"
    );
    let raw = "let r = r#\"\n  /* kept */\n\"#;\n";
    assert_eq!(strip(raw), raw);
}

#[test]
fn lifetimes_labels_and_raw_identifiers_are_code() {
    let source = "fn f<'a>(x: &'a str) -> &'a str { 'outer: loop { break 'outer; } x } // c\n\
                  let r#type = r#match; let rb = br; let cr = c;\n";
    assert_eq!(
        strip(source),
        "fn f<'a>(x: &'a str) -> &'a str { 'outer: loop { break 'outer; } x }\n\
         let r#type = r#match; let rb = br; let cr = c;\n"
    );
}

#[test]
fn identifiers_ending_in_r_b_or_c_are_not_raw_strings() {
    let source = "let bar = \"//\"; let x = bar; // c\n";
    assert_eq!(strip(source), "let bar = \"//\"; let x = bar;\n");
}

#[test]
fn stripping_twice_changes_nothing() {
    let source = "/// Doc.\nfn main() {\n    let s = \"// a\n  b\"; // c\n}\n";
    let once = strip(source);
    assert_eq!(strip(&once), once);
}

#[test]
fn empty_and_comment_only_texts_become_empty() {
    assert_eq!(strip(""), "");
    assert_eq!(strip("// only\n\n/* a */\n"), "");
}
