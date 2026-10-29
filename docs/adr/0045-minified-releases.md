# 0045. Minified releases, each with a readable copy

- Status: accepted (owner, 2026-10-29)
- Date: 2026-10-29
- Completes: [0012](0012-evaluation.md), decision 9 (what a release file is and how CI checks it); [0016](0016-self-play-training.md), decision 4 (saving room by stripping comments and indentation)
- Scope: framework

## Context

A paste-ready bot is one file of at most 100,000 characters (ADR 0037). The bundler joins a bot and the workspace crates it uses into that file, then removes comments, indentation and blank lines (ADR 0016, decision 4), and the result is the release (ADR 0012, decision 9). Releases of A Code of Ice and Fire reached that limit: `ice-and-fire-v008` had 27 characters to spare and `ice-and-fire-v009` 934, and both needed trims of the code by hand, each checked to change no move, before they fit. On 2026-10-29 the owner named this a recurring cost and asked for a tool for all games that minifies release code, "not readable by a human" and using "as little characters as possible": remove unused code and shorten what remains. The owner also asked for two versions of each release: the minified one, pasted into CodinGame, and one that is not minified, kept for the future.

A measure that day on `ice-and-fire-v009` (99,066 characters) found that names take about 64% of the file and whitespace about 14%; a sketch of the token rewrite and of renaming the names the bundle defines brought it to about 64,500 characters, before removing the 48 items rustc reports as never used. Ultimate Tic-Tac-Toe gains less, since most of its file is a packed network that must not change.

The owner chose, in the conversation of 2026-10-29: all three levels of minification, landed in stages ("All three, in stages (Recommended)"); the debug output to stderr kept ("Keep it (Recommended)"); a full readable bundle kept beside each release ("Full bundle, comments kept (Recommended)"); and to start the tool at once.

## Decision

1. **Each new release is two frozen files: the minified paste-ready file and its readable copy.** The release stays `games/<game>/releases/<game>-vNNN.rs`, the file the owner pastes and the one every evaluation, the studio and the GitHub release use. Its readable copy is `games/<game>/readable/<game>-vNNN.rs`: the same bundle with its comments, indentation and real names (the bundler's `--keep-comments` output). `scripts/new-release.sh` writes both, and both never change once merged. The workspace sources remain the code that is developed.
2. **The minifier is a stage of the bundler, for every game, in three levels.** Level 1 rewrites the bundle token by token, keeping only the whitespace Rust needs between tokens. Level 2 removes the items that rustc reports as never used, compiling again until none is left. Level 3 gives every name the bundle defines the shortest name not otherwise used in it; a renaming that rustc rejects is undone. String, byte-string and character literals are never changed, so packed data (ADR 0037) passes through as it is. The output depends only on the bundle, so a fresh minification of a fresh bundle reproduces a release exactly.
3. **Each level lands in its own pull request, checked before the next.** Each says how much it saves on the code of the latest release of each game. No such pull request makes a release: each game's first minified release is its next one.
4. **A minified release must be shown to play exactly like its readable copy.** Both compile with rustc 1.90.0 alone. Level 1's output must read back as exactly the tokens of its input. Both copies then play the same games, with a fixed seed and a fixed number of iterations, and must send the same answers every turn. CI checks this for every new release, beside the existing checks (size, standalone build, determinism), and checks that the release and its readable copy are those that the bundler and the minifier write from the sources.
5. **The debug output stays.** Lines a bot writes to stderr are kept in the minified release, so that games played on CodinGame can be analysed; they may show minified names where they print a type's debug form.
6. **The first line of a release stays a comment naming the bundled package.** The release checks read it.
7. **Released files keep their form.** Releases made before this decision are neither minified nor given a readable copy.

## Consequences

- Releases get room to grow: `ice-and-fire-v009`'s code would take about two thirds of the limit, and hand trims before a release stop being needed.
- A release file is no longer readable; its readable copy is, line by line, the code that was minified, and the answers check of decision 4 ties the two. A question about what a release does is answered from the readable copy or the sources.
- The repository stores two files per new release.
- Making a release takes longer: the minifier compiles the bundle several times, and CI plays the games of decision 4.
- A bot must reproduce its play from a seed and a fixed number of iterations, the condition the determinism check already sets (ADR 0025); a bot that does not cannot be released minified until it does.
- A renaming that changes behaviour without failing to compile would be caught by decision 4's games, not by rustc; level 3 must avoid names that std also uses, and its pull request must show how.
- To be done: the three levels, the answers check, the new release flow (`scripts/new-release.sh`, `scripts/check-releases.sh`, the release and SPRT workflows), and the docs that say a release is a readable file (`docs/ARCHITECTURE.md`, `CLAUDE.md`, `docs/WORKFLOW.md`), each with the level or step that changes it ([roadmap](../ROADMAP.md#outside-the-phases)).
