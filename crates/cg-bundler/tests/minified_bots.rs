//! Minifies the bundle of every bot of the workspace and checks each stage
//! of the minifier (ADR 0045, ADR 0046): the pruned code compiles with rustc
//! 1.90.0 and has no more tokens than the joined code; the renamed code
//! compiles and has the pruned code's tokens with only names, `self`, the
//! aliased types and the digit separators of numbers changed, each old name
//! to one new name (except at the places kept), plus the declarations of
//! `self`'s name and of the aliases, and the captures of format strings
//! that became arguments; the minified code reads back as the
//! renamed code's tokens, is what the bundler writes, and is smaller than
//! the readable bundle.

use std::collections::{BTreeMap, BTreeSet};

use cg_bundler::minify::{first_difference, tokens, Kind, Token};
use cg_bundler::rename::{self, StdNames, ALIASED};
use cg_bundler::{bundle, bundle_package, bundle_report, cg_size, joined, minify, plan, prune};
use cg_bundler::{Options, SIZE_LIMIT};

/// The package names of the bots in `games/*/bots/*`.
fn bot_packages() -> Vec<String> {
    let games = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../games");
    let mut packages = Vec::new();
    for game in std::fs::read_dir(games).unwrap() {
        let Ok(bots) = std::fs::read_dir(game.unwrap().path().join("bots")) else {
            continue;
        };
        for bot in bots {
            let Ok(manifest) = std::fs::read_to_string(bot.unwrap().path().join("Cargo.toml"))
            else {
                continue;
            };
            let name = manifest
                .lines()
                .find_map(|line| line.strip_prefix("name = \""))
                .and_then(|rest| rest.strip_suffix('"'))
                .expect("a bot manifest names its package");
            packages.push(name.to_string());
        }
    }
    packages.sort();
    packages
}

#[test]
fn every_bot_minifies_and_each_stage_checks() {
    let packages = bot_packages();
    assert!(
        packages.iter().any(|p| p == "uttt-bot-random")
            && packages.iter().any(|p| p == "ice-and-fire-bot-plan"),
        "bots found: {packages:?}"
    );
    let rustc = prune::Rustc::find().expect("these tests need rustc 1.90.0");
    // Four threads: the compiles dominate.
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| loop {
                let at = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(package) = packages.get(at) else {
                    break;
                };
                check_bot(package, &rustc);
            });
        }
    });
}

/// The checks of `every_bot_minifies_and_each_stage_checks` for one bot.
fn check_bot(package: &str, rustc: &prune::Rustc) {
    let plan = plan::plan(package, None).unwrap();
    let readable = bundle(
        &plan,
        Options {
            format: true,
            minify: false,
        },
    )
    .unwrap();
    let bundled = bundle_report(&plan, Options::default()).unwrap();
    let joined = joined(&plan, Options::default()).unwrap();
    assert!(
        !joined.contains("allow(dead_code"),
        "{package}: the minifier starts without the readable bundle's `allow`"
    );

    // The bundler's own stages, checked here rather than run a second time:
    // the stages are where the compiles go.
    let stages = bundled
        .stages
        .as_ref()
        .expect("a minified bundle has stages");
    let (pruned, renamed) = (stages.pruned.as_str(), stages.renamed.as_str());

    // Unused items removed.
    assert!(bundled.pruned.is_some(), "{package}");
    prune::check(pruned, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    // The pruned code is the joined code with items removed: its tokens are
    // the joined code's, in order, with some left out.
    let (kept, all) = (tokens(pruned).unwrap(), tokens(&joined).unwrap());
    let mut rest = all.iter();
    for token in &kept {
        assert!(
            rest.any(|t| t.text == token.text),
            "{package}: the pruned code's `{}` is not in the joined code, in order",
            token.text
        );
    }

    // Names, `self`, types and numbers shortened.
    let report = bundled
        .renamed
        .as_ref()
        .expect("a minified bundle has a renaming report");
    prune::check(renamed, rustc).unwrap_or_else(|err| panic!("{package}: {err}"));
    check_renaming(package, pruned, renamed, report);

    // Only the whitespace Rust needs.
    let minified = minify::minify(renamed).unwrap();
    assert_eq!(
        first_difference(&tokens(renamed).unwrap(), &tokens(&minified).unwrap()),
        None,
        "{package}"
    );
    let (header, code) = bundled.code.split_once('\n').unwrap();
    assert_eq!(
        header,
        format!("// Bundled by cg-bundler from the `{package}` package. Minified (ADR 0045).")
    );
    assert_eq!(code, minified, "{package}");
    assert!(
        cg_size(&bundled.code) < cg_size(&readable),
        "{package}: {} is not smaller than {}",
        cg_size(&bundled.code),
        cg_size(&readable)
    );
    assert!(cg_size(&bundled.code) < SIZE_LIMIT, "{package}");
}

/// Checks that `renamed` is `pruned` with only the changes that `report`
/// names: each old name to one new name and no two old names to one, except
/// at the places [`rename::kept`] keeps; `self` to `report.self_name`, after
/// a declaration of that name in each method that uses it; each aliased type
/// to its alias, after the alias's declarations; numbers without their digit
/// separators; format strings whose captured names became arguments, which
/// [`check_format_string`] reads on its own: `rename::calls` only says
/// where the new arguments are, to line the tokens up.
fn check_renaming(package: &str, pruned: &str, renamed: &str, report: &rename::Report) {
    let (old, new) = (tokens(pruned).unwrap(), tokens(renamed).unwrap());
    let kept = rename::kept(&old, StdNames::get());
    let calls: BTreeMap<usize, rename::Call> = rename::calls(&old)
        .into_iter()
        .map(|call| (call.string, call))
        .collect();
    // The number of `,new` expected before an old token; the new index of
    // each old token, the new tokens that are none of them, and the strings
    // that changed.
    let mut arguments: BTreeMap<usize, usize> = BTreeMap::new();
    let mut aligned: BTreeMap<usize, usize> = BTreeMap::new();
    let mut inserted = BTreeSet::new();
    let mut changed = Vec::new();
    let self_name = report.self_name.as_deref();
    let aliases: BTreeMap<&str, &str> = report
        .aliases
        .iter()
        .map(|(ty, alias, _)| (alias.as_str(), ty.as_str()))
        .collect();
    let text = |tokens: &[Token], at: usize| tokens.get(at).map(|t| t.text.clone());
    let starts_with = |at: usize, expected: &str| {
        let expected = tokens(expected).unwrap();
        (0..expected.len())
            .all(|k| new.get(at + k).is_some_and(|t| t.same(&expected[k])))
            .then_some(expected.len())
    };
    let (mut i, mut j) = (0, 0);
    let (mut declared_selves, mut declared_aliases) = (0, 0);
    let (mut new_of, mut old_of) = (BTreeMap::new(), BTreeMap::new());
    while j < new.len() {
        if let Some(name) = self_name {
            if let Some(length) = [false, true]
                .into_iter()
                .find_map(|mutable| starts_with(j, &rename::self_declaration(name, mutable)))
            {
                declared_selves += 1;
                j += length;
                continue;
            }
        }
        if text(&new, j).as_deref() == Some("type") {
            let alias = text(&new, j + 1).unwrap_or_default();
            if let Some(ty) = aliases.get(alias.as_str()) {
                let generic = ALIASED.iter().any(|&(t, generic)| t == *ty && generic);
                let length = starts_with(j, &rename::alias_declaration(&alias, ty, generic))
                    .unwrap_or_else(|| panic!("{package}: a wrong declaration of {alias}"));
                declared_aliases += 1;
                j += length;
                continue;
            }
        }
        for _ in 0..arguments.remove(&i).unwrap_or(0) {
            assert_eq!(text(&new, j).as_deref(), Some(","), "{package}");
            inserted.extend([j, j + 1]);
            j += 2;
        }
        let (o, n) = (&old[i], &new[j]);
        aligned.insert(i, j);
        if let (Some(call), true) = (calls.get(&i), o.text != n.text) {
            // Where the new arguments are: a capture whose name the new
            // text lacks went out.
            let (mut copied, mut at) = (0, 0);
            let mut out = BTreeSet::new();
            for capture in &call.captures {
                let before = &o.text[copied..capture.start];
                assert!(
                    n.text[at..].starts_with(before),
                    "{package}: `{}` became `{}`",
                    o.text,
                    n.text
                );
                at += before.len();
                copied = capture.start;
                if !n.text[at..].starts_with(&capture.name) {
                    out.insert(capture.start);
                    *arguments.entry(call.slots[capture.slot]).or_default() += 1;
                    copied = capture.end;
                }
            }
            assert_eq!(
                call.string_text(&o.text, |c| out.contains(&c.start)),
                n.text,
                "{package}"
            );
            changed.push(i);
            i += 1;
            j += 1;
            continue;
        }
        assert_eq!(
            o.kind, n.kind,
            "{package}: `{}` became `{}`",
            o.text, n.text
        );
        match o.kind {
            Kind::Number => assert_eq!(o.text.replace('_', ""), n.text, "{package}"),
            Kind::Ident if Some(n.text.as_str()) == self_name => {
                assert_eq!(
                    o.text, "self",
                    "{package}: `{}` became `{}`",
                    o.text, n.text
                )
            }
            Kind::Ident if aliases.contains_key(n.text.as_str()) => {
                assert_eq!(o.text, aliases[n.text.as_str()], "{package}")
            }
            Kind::Ident if kept.positions.contains(&i) => {
                assert_eq!(o.text, n.text, "{package}: a kept place changed")
            }
            Kind::Ident => {
                let new_name = new_of.entry(o.text.clone()).or_insert(n.text.clone());
                assert_eq!(
                    *new_name, n.text,
                    "{package}: `{}` has two new names",
                    o.text
                );
                let old_name = old_of.entry(n.text.clone()).or_insert(o.text.clone());
                assert_eq!(
                    *old_name, o.text,
                    "{package}: `{}` is the new name of two",
                    n.text
                );
            }
            _ => assert_eq!(o.text, n.text, "{package}"),
        }
        i += 1;
        j += 1;
    }
    assert_eq!(i, old.len(), "{package}: tokens are missing");
    assert!(arguments.is_empty(), "{package}: arguments are missing");
    let mut used = BTreeSet::new();
    for at in changed {
        let strings = Strings {
            old: &old,
            new: &new,
            aligned: &aligned,
            inserted: &inserted,
        };
        let (made, left) = strings
            .check_format_string(at, &mut used)
            .unwrap_or_else(|err| panic!("{package}: `{}`: {err}", old[at].text));
        for (name, argument) in made {
            assert!(
                new_of.get(&name) == Some(&argument) && name != argument,
                "{package}: the capture `{name}` became the argument `{argument}`"
            );
        }
        for name in left {
            assert!(
                new_of.get(&name).is_none_or(|new| *new == name),
                "{package}: `{name}` was renamed but a string still captures it"
            );
        }
    }
    assert_eq!(used, inserted, "{package}: new tokens that are no argument");
    assert_eq!(declared_selves, report.methods, "{package}");
    let declarations: usize = report.aliases.iter().map(|(_, _, n)| n).sum();
    assert_eq!(declared_aliases, declarations, "{package}");
    let changed = new_of.iter().filter(|(old, new)| old != new).count();
    assert_eq!(changed, report.renamed, "{package}");
}

#[test]
fn the_same_bundle_comes_out_every_time() {
    let options = Options::default();
    let first = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    let second = bundle_package("ice-and-fire-bot-plan", None, options).unwrap();
    assert_eq!(first, second);
}

/// The formatting macros of std, with the place of their format string,
/// written here again so that the check does not rely on the renaming's.
const FORMAT_MACROS: &[(&str, usize)] = &[
    ("format", 0),
    ("format_args", 0),
    ("print", 0),
    ("println", 0),
    ("eprint", 0),
    ("eprintln", 0),
    ("panic", 0),
    ("unreachable", 0),
    ("todo", 0),
    ("unimplemented", 0),
    ("write", 1),
    ("writeln", 1),
    ("assert", 1),
    ("debug_assert", 1),
    ("assert_eq", 2),
    ("assert_ne", 2),
    ("debug_assert_eq", 2),
    ("debug_assert_ne", 2),
];

/// The tokens before and after the renaming, lined up.
struct Strings<'a> {
    old: &'a [Token],
    new: &'a [Token],
    /// The new index of each old token.
    aligned: &'a BTreeMap<usize, usize>,
    /// The new tokens that stand for no old one.
    inserted: &'a BTreeSet<usize>,
}

impl Strings<'_> {
    /// Checks that the string at old index `at`, which changed, is the
    /// format string of a call of std's formatting macros in code, and
    /// that each of its placeholders reads, after the renaming, the same
    /// argument with the same spec: an old argument at its new place, a
    /// captured name either still captured or an inserted argument. Gives
    /// the captures made arguments, with their argument, and those left;
    /// adds the inserted tokens it accounts for to `used`.
    #[allow(clippy::type_complexity)]
    fn check_format_string(
        &self,
        at: usize,
        used: &mut BTreeSet<usize>,
    ) -> Result<(Vec<(String, String)>, Vec<String>), String> {
        let (old, new) = (self.old, self.new);
        let new_at = self.aligned[&at];
        // The macros around the string: the innermost a formatting macro
        // with the string at its place, none a definition, one of the
        // bundle or one that turns its tokens into text.
        let own: BTreeSet<&str> = (0..old.len())
            .filter(|&k| {
                old[k].text == "macro_rules" && old.get(k + 1).is_some_and(|t| t.text == "!")
            })
            .filter_map(|k| old.get(k + 2).map(|t| t.text.as_str()))
            .collect();
        let groups = enclosing(old, at);
        let innermost = *groups.first().ok_or("in no group")?;
        let macro_name = |open: usize| {
            (open >= 2 && old[open - 1].text == "!").then(|| old[open - 2].text.as_str())
        };
        let (_, place) = macro_name(innermost)
            .and_then(|name| FORMAT_MACROS.iter().find(|(n, _)| *n == name))
            .ok_or("not in a call of a formatting macro of std")?;
        for &open in &groups {
            let definition =
                open >= 3 && old[open - 2].text == "!" && old[open - 3].text == "macro_rules";
            let name = macro_name(open);
            if definition
                || name.is_some_and(|n| {
                    own.contains(n) || matches!(n, "stringify" | "concat" | "module_path")
                })
            {
                return Err(
                    "in a macro definition or the arguments of a macro that may read tokens".into(),
                );
            }
        }
        // The whole argument at its place: after the opening bracket or a
        // comma, before a comma or the closing bracket.
        let starts = at - 1 == innermost || old[at - 1].text == ",";
        let ends = old
            .get(at + 1)
            .is_some_and(|t| matches!(t.text.as_str(), "," | ")" | "]" | "}"));
        if top_level_commas(old, innermost + 1, at) != *place || !starts || !ends {
            return Err("not the format string of its call".into());
        }
        let (old_pieces, new_pieces) = (pieces(&old[at].text)?, pieces(&new[new_at].text)?);
        let (old_args, new_args) = (arguments_after(old, at), arguments_after(new, new_at));
        if old_pieces.len() != new_pieces.len() {
            return Err("another number of pieces".into());
        }
        let (mut next_old, mut next_new) = (0, 0);
        let (mut made, mut left) = (Vec::new(), Vec::new());
        for (o, n) in old_pieces.iter().zip(&new_pieces) {
            match (o, n) {
                (Piece::Text(a), Piece::Text(b)) if a == b => {}
                (Piece::Arg(oa, os), Piece::Arg(na, ns)) if os == ns => {
                    if os.contains(['$', '*']) {
                        return Err("a count changed string".into());
                    }
                    let argument = |next: &mut usize, args: &[(usize, usize)]| {
                        let arg = args.get(*next).copied();
                        *next += 1;
                        arg.ok_or("a placeholder without its argument")
                    };
                    match (oa.as_str(), na.as_str()) {
                        ("", "") => {
                            let (from, to) = argument(&mut next_old, &old_args)?;
                            let (new_from, new_to) = argument(&mut next_new, &new_args)?;
                            let same = (from..to)
                                .all(|k| self.aligned.get(&k) == Some(&(new_from + k - from)))
                                && new_to - new_from == to - from;
                            if !same {
                                return Err("a placeholder reads another argument".into());
                            }
                        }
                        (name, "") if is_name(name) => {
                            let (from, to) = argument(&mut next_new, &new_args)?;
                            if to != from + 1
                                || !self.inserted.contains(&from)
                                || new[from].kind != Kind::Ident
                            {
                                return Err(format!("`{name}` did not become an inserted name"));
                            }
                            used.extend([from - 1, from]);
                            made.push((name.to_string(), new[from].text.clone()));
                        }
                        (name, other) if name == other && is_name(name) => {
                            left.push(name.to_string())
                        }
                        _ => return Err(format!("`{{{oa}}}` became `{{{na}}}`")),
                    }
                }
                _ => return Err(format!("{o:?} became {n:?}")),
            }
        }
        if next_old != old_args.len() || next_new != new_args.len() {
            return Err("arguments that no placeholder reads".into());
        }
        Ok((made, left))
    }
}

/// The groups around the token at `at`, innermost first, by the index of
/// their opening bracket.
fn enclosing(tokens: &[Token], at: usize) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut depth = 0usize;
    for k in (0..at).rev() {
        match tokens[k].text.as_str() {
            ")" | "]" | "}" => depth += 1,
            "(" | "[" | "{" if depth > 0 => depth -= 1,
            "(" | "[" | "{" => groups.push(k),
            _ => {}
        }
    }
    groups
}

/// The commas between `from` and `to` outside brackets.
fn top_level_commas(tokens: &[Token], from: usize, to: usize) -> usize {
    let mut depth = 0usize;
    let mut commas = 0;
    for token in &tokens[from..to] {
        match token.text.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth -= 1,
            "," if depth == 0 => commas += 1,
            _ => {}
        }
    }
    commas
}

/// The token ranges of the arguments after the string at `at`, up to the
/// end of its group, a trailing comma left out.
fn arguments_after(tokens: &[Token], at: usize) -> Vec<(usize, usize)> {
    let mut arguments = Vec::new();
    let mut depth = 0usize;
    let mut start = at + 1;
    for k in at + 1..tokens.len() {
        match tokens[k].text.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" if depth > 0 => depth -= 1,
            ")" | "]" | "}" | "," if depth == 0 => {
                if k > start + 1 || (k == start + 1 && tokens[start].text != ",") {
                    arguments.push((start + 1, k));
                }
                if tokens[k].text != "," {
                    break;
                }
                start = k;
            }
            _ => {}
        }
    }
    arguments
}

/// A piece of a format string as rustc reads it.
#[derive(Debug, PartialEq)]
enum Piece {
    Text(String),
    /// A placeholder: what is before its `:`, and its spec.
    Arg(String, String),
}

/// The pieces of a plain string literal's value, escapes decoded.
fn pieces(literal: &str) -> Result<Vec<Piece>, String> {
    let value = unescape(literal)?;
    let mut pieces = Vec::new();
    let mut text = String::new();
    let mut chars = value.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                text.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                text.push('}');
            }
            '{' => {
                let inside: String = chars.by_ref().take_while(|&c| c != '}').collect();
                let (arg, spec) = inside.split_once(':').unwrap_or((&inside, ""));
                pieces.push(Piece::Text(std::mem::take(&mut text)));
                pieces.push(Piece::Arg(arg.to_string(), spec.to_string()));
            }
            '}' => return Err("a lone `}`".into()),
            c => text.push(c),
        }
    }
    pieces.push(Piece::Text(text));
    Ok(pieces)
}

/// The value of a plain string literal.
fn unescape(literal: &str) -> Result<String, String> {
    let inner = literal
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .ok_or("not a plain string literal")?;
    let mut value = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            value.push(c);
            continue;
        }
        match chars.next().ok_or("a lone backslash")? {
            'n' => value.push('\n'),
            'r' => value.push('\r'),
            't' => value.push('\t'),
            '0' => value.push('\0'),
            c @ ('\\' | '\'' | '"') => value.push(c),
            'x' => {
                let hex: String = chars.by_ref().take(2).collect();
                let code = u8::from_str_radix(&hex, 16).map_err(|e| e.to_string())?;
                value.push(char::from(code));
            }
            'u' => {
                let hex: String = chars.by_ref().skip(1).take_while(|&c| c != '}').collect();
                let code =
                    u32::from_str_radix(&hex.replace('_', ""), 16).map_err(|e| e.to_string())?;
                value.push(char::from_u32(code).ok_or("not a character")?);
            }
            '\n' => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            other => return Err(format!("an unknown escape `\\{other}`")),
        }
    }
    Ok(value)
}

/// Whether `word` is a name.
fn is_name(word: &str) -> bool {
    let mut chars = word.chars();
    chars.next().is_some_and(|c| c.is_alphabetic() || c == '_')
        && chars.all(|c| c.is_alphanumeric() || c == '_')
}
