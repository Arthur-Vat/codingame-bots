# 0045. Minified releases, each with a readable copy

- Status: accepted (owner, 2026-10-29)
- Date: 2026-10-29
- Completes: [0012](0012-evaluation.md), decision 9 (what a release file is and how CI checks it); [0016](0016-self-play-training.md), decision 4 (saving room by stripping comments and indentation)
- Scope: framework

## Context

A paste-ready bot is one file of at most 100,000 characters (ADR 0037). The bundler joins a bot and the workspace crates it uses into that file, then removes comments, indentation and blank lines (ADR 0016, decision 4), and the result is the release (ADR 0012, decision 9). Releases of A Code of Ice and Fire reached that limit: `ice-and-fire-v008` had 27 characters to spare and `ice-and-fire-v009` 934, and both needed trims of the code by hand before they fit, each checked by hand to play as before. On 2026-10-29 the owner named this a recurring cost and asked for a tool for all games that minifies release code, "not readable by a human" and using "as little characters as possible": remove unused code and shorten what remains. The owner also asked for two versions of each release: the minified one, pasted into CodinGame, and one that is not minified, kept for the future.

A measure that day on `ice-and-fire-v009` (99,066 characters) found that names and keywords take about 64% of the file and whitespace about 14%; a sketch of the token rewrite and of renaming the names the bundle defines brought it to about 64,500 characters, before removing the 48 items rustc reports as never used. Ultimate Tic-Tac-Toe gains less, since much of its file (about 40%) is a packed network that must not change.

The owner chose, in the conversation of 2026-10-29: all three levels of minification, landed in stages ("All three, in stages (Recommended)"); the debug output to stderr kept ("Keep it (Recommended)"); a full readable bundle kept beside each release ("Full bundle, comments kept (Recommended)"); and to start the tool at once.

## Decision

1. **Each new release is two frozen files: the minified paste-ready file and its readable copy.** The release stays `games/<game>/releases/<game>-vNNN.rs`, the file the owner pastes and the one every evaluation, the studio and the GitHub release use. Its readable copy is `games/<game>/readable/<game>-vNNN.rs`: the same bundle with its comments, indentation and real names (the bundler's `--keep-comments` output). `scripts/new-release.sh` writes both, and both never change once merged. The workspace sources remain the code that is developed.
2. **The minifier is a stage of the bundler, for every game, in three levels.** Level 1 rewrites the bundle token by token, keeping only the whitespace Rust needs between tokens. Level 2 removes the items that rustc reports as never used and compiles again, undoing a removal that rustc rejects, until no removal is left that compiles. Level 3 gives every name the bundle defines the shortest name not otherwise used in it; a renaming that rustc rejects is undone. String, byte-string and character literals are never changed, so packed data (ADR 0037) passes through as it is. The output depends only on the bundle and on rustc 1.90.0, CodinGame's version (ADR 0010), which the minifier runs whatever the session's default compiler, so a fresh minification of a fresh bundle reproduces a release exactly.
3. **Each level lands in its own pull request, checked before the next.** Each says how much it saves on the code of the latest release of each game. No such pull request makes a release: each game's first minified release is its next one.
4. **A minified release must be shown to play exactly like its readable copy.** Both compile with rustc 1.90.0 alone. Level 1's output must read back as exactly the tokens of its input. Both copies then play the same games, with a fixed seed and a fixed number of iterations, and must send the same answers every turn. CI checks this for every new release, together with its size, its standalone build and its determinism, and checks that the release and its readable copy are those that the bundler and the minifier write from the sources.
5. **The debug output stays.** Lines a bot writes to stderr are kept in the minified release, so that games played on CodinGame can be analysed; they may show minified names where they print a type's debug form.
6. **The first line of a release stays a comment naming the bundled package.** The release checks read it.
7. **Released files keep their form.** Releases made before the first level and the new release flow land are neither minified nor given a readable copy; from then on, every new release follows decision 1.

## Consequences

- Releases get room to grow: `ice-and-fire-v009`'s code would take about two thirds of the limit, and hand trims before a release stop being needed.
- A release file is no longer readable; its readable copy is, line by line, the code that was minified, and the answers check of decision 4 ties the two. A question about what a release does is answered from the readable copy or the sources.
- The repository stores two files per new release.
- Making a release takes longer: the minifier compiles the bundle several times, and CI plays the games of decision 4.
- A bot must reproduce its play from a seed and a fixed number of iterations, as ADR 0025 (decision 5) asks of every release; a bot that does not cannot be released minified until it does.
- A minification that changes behaviour without failing to compile would be caught by decision 4's games, not by rustc. Level 2 can do it by removing an enum variant, which renumbers the variants after it where code converts them to numbers; level 3 by giving a name that std also uses. Each level's pull request must show how it avoids this.
- To be done: the three levels, the answers check, the new release flow (`scripts/new-release.sh`, `scripts/check-releases.sh`, the release and SPRT workflows), and the docs that say a release is a readable file (`docs/ARCHITECTURE.md`, `CLAUDE.md`, `docs/WORKFLOW.md`), each with the level or step that changes it ([roadmap](../ROADMAP.md#outside-the-phases)).
