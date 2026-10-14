# 0037. Count a bot's size in characters, as UTF-16 units

- Status: accepted (owner, 2026-10-14)
- Date: 2026-10-14
- Scope: framework

## Context

A paste-ready bot had to stay under 100,000 bytes, the limit players reported on the forum ([CODINGAME.md](../CODINGAME.md#submission-limits)). A top Ultimate Tic-Tac-Toe player's write-up of 2026-09-20 says the limit counts characters, not bytes. A character outside ASCII then counts as one, though it takes 2 or 3 bytes in UTF-8. Packing data into such characters carries about 15.9 bits per character, against 6 for base64 ([research/utf16-packing.md](../research/utf16-packing.md)). That is room for networks several times larger than a bot holds in base64.

On 2026-10-14 the owner tested this on CodinGame with a probe Claude built: `uttt-v010` plus packed data, 95,713 characters and 125,475 bytes. It compiled, played in the IDE and was accepted at submission, with its data intact. All of its characters were below U+FFFF, so it did not show how CodinGame counts the characters above, which take two UTF-16 units. Nor did it show the exact boundary.

The owner chose, among three options, to count UTF-16 units with the limit at 100,000. The others were the same count with a margin, and keeping bytes.

## Decision

1. **A paste-ready bot is at most 100,000 characters, counted as UTF-16 units.** A character above U+FFFF counts as two, the cautious reading of what the probe left open. Bytes no longer count.
2. **`scripts/cg-check.sh` enforces it, and rejects a file that is not valid UTF-8.** The bundler warns by the same count. A test of the check runs in CI.
3. **Every tool of the packing is Rust (owner, 2026-10-14).** The encoder and the decoder are written in Rust, like the rest of the framework ([ADR 0003](0003-rust.md)).

## Consequences

- Bots may carry packed data: about 190 KB in a file of 100,000 characters, less the bot's own code, against about 75 KB as base64.
- A bot's code costs as much as before, one character per byte of ASCII. Shorter bundled code leaves more room for data.
- The first release that holds more packed data than the probe did should first be tried in the IDE and at submission, then recorded in [CODINGAME.md](../CODINGAME.md).
- `CLAUDE.md`, the playbook, the reviewer agent, the skills and the game model template state the limit in characters.
