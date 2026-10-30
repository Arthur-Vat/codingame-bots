# 0046. One minifier, with names outside ASCII and aliases

- Status: accepted (owner, 2026-10-30)
- Date: 2026-10-30
- Supersedes: [0045](0045-minified-releases.md), decision 2 (its three levels) and decision 3
- Scope: framework

## Context

ADR 0045 made the minifier a stage of the bundler in three levels, each landed in its own pull request: level 1 (whitespace), level 2 (unused items removed) and level 3 (names renamed). They were merged on 2026-10-29 and 2026-10-30, level 3 being the default for new releases. The levels then served only those pull requests: `--minify-level` let a run stop early, the studio bundled at level 1 so as not to need rustc 1.90.0, and the `cg-tokens` program compared two files' tokens, which only holds between the readable bundle and level 1.

On 2026-10-30 the owner asked for more savings at once ("I'd rather do step A now"). Measured that day on the sources of `ice-and-fire-v011` (56,733 characters at level 3): new names taken from CJK ideographs once the single ASCII letters are used, each one UTF-16 unit as CodinGame counts (ADR 0037), save about 3,600 characters with no new warning; a short name for `self` in methods that use it often, aliases such as `type U=usize;` per module, std paths' middle segments kept only where they are, and a few smaller changes save about 3,000 more. The owner chose one pull request for all of it ("One PR", "All of step A"), the readable bundle for the studio ("Readable bundle"), the removal of `cg-tokens` ("Remove it"), and no separate test of pasting such names into CodinGame ("Skip A1"): the first release that has them is the test.

## Decision

1. **There is one minifier, with no levels to choose.** The bundler writes the minified bundle, its default, which needs rustc 1.90.0, or the readable bundle (`--keep-comments`), which does not. The minifier's stages (unused items removed, names and words shortened, whitespace removed) each keep their own check, and the whole is compiled at the end, but none can be run alone from the bundler.
2. **The renaming stage shortens more than names, each change undone alone when rustc rejects it.** Once a case's single ASCII letters are used, new names are CJK ideographs from U+4E00, one UTF-16 unit each, compared with the old names by UTF-16 length. A method that uses `self` often enough starts with `let s=self;` (`let mut` for `mut self`) and uses `s`, except in `self::` paths and in bodies that hold an item with a `self` of its own. A module whose uses of a common type save more than the declaration costs declares one alias per type (`type U=usize;`, `type V<T>=Vec<T>;`), except in attributes, `use` declarations, `macro_rules!` bodies, text macros, a path's later segments, members, fields and the names of items. A std path's middle segments keep their names only there, a name after `::` is kept for std only along a path that starts with a name leading into std, and numbers lose their digit separators. The rules that keep std's names and the answers check of ADR 0045 (decision 4) stay as they are.
3. **The minified bundle carries no `allow` for unused code.** The minifier removes what rustc reports as unused; what it must keep (enum variants, items whose removal does not compile) may give rustc warnings, which are not errors; the first release that has them also shows how CodinGame treats them (decision 6).
4. **Tools that only compile a bot use the readable bundle.** The studio is the first.
5. **`cg-tokens` is removed.** The minifier checks its own stages.
6. **Names outside ASCII get no separate test on CodinGame.** The first release that has them is the test: if CodinGame rejects them, or the warnings of decision 3, a new decision changes decision 2 or 3.

## Consequences

- A change to the minifier lands as one change, checked as a whole on every bot: each stage's check, the tokens mapped back one to one from the renamed code to the pruned code, and the answers check of each release.
- Minified releases get smaller (`ice-and-fire-v011`'s sources from 56,733 to about 50,300 characters) and harder to read: a question about one is answered from its readable copy, as before.
- A release file's size in bytes grows, since a CJK ideograph takes three bytes in UTF-8; CodinGame counts UTF-16 units (ADR 0037).
- A bot's debug output may show the new names, CJK ideographs included, where it prints a type's debug form (ADR 0045, decision 5).
- The release that follows is the first with CJK names; whether CodinGame accepts them is known once the owner pastes it.
- To be done in the same change: the bundler's options and tests, the studio, the removal of `cg-tokens`, and the docs that name the levels (`docs/ARCHITECTURE.md`, the scripts' comments, the roadmap).
