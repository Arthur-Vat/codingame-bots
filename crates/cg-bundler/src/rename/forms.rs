//! The places where `self` and some types may be written shorter (see the
//! documentation of [`super`]): the methods whose `self` may get a one-letter
//! name, and the uses of the types that may get an alias, by module.

use std::collections::{BTreeMap, BTreeSet};

use super::kept::text_macros;
use super::{braced_words, group_end};
use crate::minify::{Kind, Token};

/// The types that may get an alias, and whether they take a type parameter.
pub const ALIASED: &[(&str, bool)] = &[
    ("usize", false),
    ("isize", false),
    ("u8", false),
    ("u16", false),
    ("u32", false),
    ("u64", false),
    ("u128", false),
    ("i8", false),
    ("i16", false),
    ("i32", false),
    ("i64", false),
    ("i128", false),
    ("f32", false),
    ("f64", false),
    ("bool", false),
    ("char", false),
    ("String", false),
    ("Instant", false),
    ("Vec", true),
    ("Option", true),
];

/// A method that takes `self`, with a body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    /// The token before which `let s=self;` goes: the first of its body
    /// after the body's inner attributes.
    pub at: usize,
    /// Whether it takes `mut self`, so that the new name must be `mut`.
    pub mutable: bool,
    /// The indices of the `self` tokens of its body (not those of
    /// `self::` paths).
    pub selves: Vec<usize>,
}

/// The uses of a type that may get an alias.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uses {
    pub ty: &'static str,
    /// Whether the type takes a parameter (`Vec<T>`).
    pub generic: bool,
    /// The indices of the tokens of its uses, by module (an index in
    /// [`Forms::modules`]).
    pub by_module: BTreeMap<usize, Vec<usize>>,
}

/// What [`forms`] finds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Forms {
    /// The methods whose `self` may get another name.
    pub methods: Vec<Method>,
    /// For each module, the token before which its aliases go; the first
    /// is the bundle's root.
    pub modules: Vec<usize>,
    /// The uses of the types of [`ALIASED`] that may be written with an
    /// alias, for the types that have some.
    pub uses: Vec<Uses>,
}

/// The methods, the modules and the uses of aliased types in `tokens`.
pub fn forms(tokens: &[Token]) -> Forms {
    let blocked = blocked(tokens);
    let mut forms = Forms {
        methods: methods(tokens, &blocked),
        modules: vec![after_inner_attributes(tokens, 0)],
        uses: Vec::new(),
    };

    // The module of each token: the innermost `mod name { ... }` around it.
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let mut owner = vec![0; tokens.len()];
    for at in 0..tokens.len() {
        if text(at) != Some("mod")
            || tokens.get(at + 1).map(|t| t.kind) != Some(Kind::Ident)
            || text(at + 2) != Some("{")
        {
            continue;
        }
        let Some(end) = group_end(tokens, at + 2) else {
            continue;
        };
        let module = forms.modules.len();
        forms.modules.push(after_inner_attributes(tokens, at + 3));
        // Modules come in the order of the source, so an inner one is
        // marked after the one around it.
        for owned in &mut owner[at + 3..end - 1] {
            *owned = module;
        }
    }

    for &(ty, generic) in ALIASED {
        let mut by_module: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (at, token) in tokens.iter().enumerate() {
            if token.kind != Kind::Ident || token.text != ty || blocked[at] {
                continue;
            }
            let before = at.checked_sub(1).and_then(text);
            let after = text(at + 1);
            // Not a path's later segment (`std::time::Instant`), a member, a
            // macro, a field or a binding of that name, or the name of an
            // item.
            let defines = matches!(
                before,
                Some(
                    "::" | "."
                        | "fn"
                        | "struct"
                        | "enum"
                        | "union"
                        | "type"
                        | "trait"
                        | "mod"
                        | "let"
                        | "const"
                        | "static"
                        | "mut"
                        | "ref"
                )
            );
            if defines || matches!(after, Some("!" | ":")) {
                continue;
            }
            if generic && !matches!(after, Some("<" | "::")) {
                continue;
            }
            by_module.entry(owner[at]).or_default().push(at);
        }
        if !by_module.is_empty() {
            forms.uses.push(Uses {
                ty,
                generic,
                by_module,
            });
        }
    }
    forms
}

/// The tokens where no alias goes and whose `self` keeps its name:
/// attributes, `macro_rules!` definitions, `use` declarations, the
/// arguments of macros that turn their tokens into text, and those of the
/// bundle's own macros, whose rules may match the very words (`(usize) =>`,
/// `(self) =>`) that an alias or the new name of `self` would change.
fn blocked(tokens: &[Token]) -> Vec<bool> {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let text_macros = text_macros(tokens);
    let own_macros: BTreeSet<&str> = (0..tokens.len())
        .filter(|&at| {
            text(at) == Some("macro_rules")
                && text(at + 1) == Some("!")
                && tokens.get(at + 2).map(|t| t.kind) == Some(Kind::Ident)
        })
        .map(|at| tokens[at + 2].text.as_str())
        .collect();
    let mut blocked = vec![false; tokens.len()];
    let mut block = |from: usize, to: usize| {
        for flag in &mut blocked[from..to.min(tokens.len())] {
            *flag = true;
        }
    };
    for (at, token) in tokens.iter().enumerate() {
        match token.text.as_str() {
            "#" => {
                let open = if text(at + 1) == Some("!") {
                    at + 2
                } else {
                    at + 1
                };
                if text(open) == Some("[") {
                    block(at, group_end(tokens, open).unwrap_or(tokens.len()));
                }
            }
            "macro_rules" if text(at + 1) == Some("!") => {
                block(at, group_end(tokens, at + 3).unwrap_or(tokens.len()));
            }
            "use" if token.kind == Kind::Ident => {
                let end = (at..tokens.len())
                    .find(|&i| text(i) == Some(";"))
                    .unwrap_or(tokens.len());
                block(at, end);
            }
            name if (text_macros.contains(name) || own_macros.contains(name))
                && text(at + 1) == Some("!") =>
            {
                block(at, group_end(tokens, at + 2).unwrap_or(tokens.len()));
            }
            _ => {}
        }
    }
    blocked
}

/// The methods that take `self` and have a body with no item that could
/// have a `self` of its own (`fn`, `impl`, `macro_rules!`), no `self` in a
/// blocked place, and no string that a formatting macro may read `self`
/// from (`"{self:?}"`): that `self` would stay, and in a `mut self` method
/// of a `Copy` type it would be a stale copy, without an error.
fn methods(tokens: &[Token], blocked: &[bool]) -> Vec<Method> {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    let mut methods = Vec::new();
    'items: for at in 0..tokens.len() {
        if text(at) != Some("fn") || tokens.get(at + 1).map(|t| t.kind) != Some(Kind::Ident) {
            continue;
        }
        let mut open = at + 2;
        if text(open) == Some("<") {
            let Some(end) = angle_end(tokens, open) else {
                continue;
            };
            open = end;
        }
        if text(open) != Some("(") {
            continue;
        }
        let Some(params_end) = group_end(tokens, open) else {
            continue;
        };
        // The first parameter: `self`, `mut self`, `&self`, `&'a mut self`.
        let mut first = open + 1;
        let mut mutable = false;
        if text(first) == Some("&") {
            first += 1;
            if tokens.get(first).map(|t| t.kind) == Some(Kind::Lifetime) {
                first += 1;
            }
            if text(first) == Some("mut") {
                first += 1;
            }
        } else if text(first) == Some("mut") {
            mutable = true;
            first += 1;
        }
        if text(first) != Some("self") || !matches!(text(first + 1), Some("," | ")" | ":")) {
            continue;
        }
        // The body: the first `{` after the parameters, outside groups.
        let mut body = params_end;
        loop {
            match text(body) {
                Some("{") => break,
                Some("(" | "[") => match group_end(tokens, body) {
                    Some(end) => body = end,
                    None => continue 'items,
                },
                Some(";") | None => continue 'items,
                _ => body += 1,
            }
        }
        let Some(end) = group_end(tokens, body) else {
            continue;
        };
        let mut selves = Vec::new();
        for (i, &blocked) in blocked.iter().enumerate().take(end - 1).skip(body + 1) {
            match text(i) {
                Some("fn" | "impl" | "macro_rules") => continue 'items,
                _ if tokens[i].kind == Kind::Str
                    && braced_words(&tokens[i].text).contains(&"self") =>
                {
                    continue 'items
                }
                Some("self") if text(i + 1) != Some("::") && text(i - 1) != Some("::") => {
                    if blocked {
                        continue 'items;
                    }
                    selves.push(i);
                }
                _ => {}
            }
        }
        if !selves.is_empty() {
            methods.push(Method {
                at: after_inner_attributes(tokens, body + 1),
                mutable,
                selves,
            });
        }
    }
    methods
}

/// The index after the generic parameters that open with the `<` at
/// `open`, or `None` when they do not close.
fn angle_end(tokens: &[Token], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut at = open;
    while let Some(token) = tokens.get(at) {
        match token.text.as_str() {
            "<" => depth += 1,
            "<<" => depth += 2,
            ">" => depth -= 1,
            ">>" => depth -= 2,
            "(" | "[" => {
                at = group_end(tokens, at)?;
                continue;
            }
            ";" | "{" => return None,
            _ => {}
        }
        at += 1;
        if depth <= 0 {
            return (depth == 0).then_some(at);
        }
    }
    None
}

/// The index of the first token at or after `at` that is not in an inner
/// attribute (`#![...]`).
fn after_inner_attributes(tokens: &[Token], mut at: usize) -> usize {
    let text = |at: usize| tokens.get(at).map(|t| t.text.as_str());
    while text(at) == Some("#") && text(at + 1) == Some("!") && text(at + 2) == Some("[") {
        match group_end(tokens, at + 2) {
            Some(end) => at = end,
            None => break,
        }
    }
    at
}
