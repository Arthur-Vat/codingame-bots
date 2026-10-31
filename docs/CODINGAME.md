# CodinGame platform facts

Facts the code depends on, and where each one comes from. Re-check them when CodinGame announces a language update. Game-specific facts live in each game's `RULES.md` (rules, protocol and time limits) and README (for example [games/uttt/README.md](../games/uttt/README.md)).

## Compilers

| Language | Version | Source |
| --- | --- | --- |
| Rust | rustc 1.90.0. The 2021 edition compiles; the 2024 edition was not tested. | Probe in the CodinGame IDE, 2026-10-06 |
| C++ (reference only) | GCC 11.2.0 | Same probe |

No crates are available: a bot is one file using the standard library only.

### Re-running the probe

1. Open any bot programming game on CodinGame and enter the IDE.
2. Select **Bash**, paste the script below, click **Play my code**, and read the error console:

   ```bash
   rustc --version >&2 || /usr/local/bin/rustc --version >&2
   g++ --version | head -1 >&2
   ```

3. Select **Rust**, paste the program below and play again. It prints `edition 2021 OK: 7` only if the 2021 edition is used (`try_into` is not in the 2018 prelude):

   ```rust
   fn main() {
       let x: u8 = 300u16.try_into().unwrap_or(7);
       eprintln!("edition 2021 OK: {}", x);
   }
   ```

If the Rust version changes, update `CG_RUST` in every workflow of `.github/workflows/` that sets it (`grep -rn CG_RUST .github`) and `rust-version` in `Cargo.toml` together, and record it in a new ADR that supersedes [ADR 0010](adr/0010-codingame-rust-toolchain.md).

## API

CodinGame has a public REST API for coding agents at `https://www.codingame.com/api/v1/` ([llms.txt](https://www.codingame.com/api/v1/docs/llms.txt), read 2026-10-31). How sessions use it is [ADR 0047](adr/0047-codingame-api.md): test games freely within the limits, submissions of a release only on the owner's approval in the conversation.

- **Agent Skill:** install it in each session as llms.txt says (into `~/.claude/skills/codingame/`, outside the repository), then use skill `codingame`. It holds the endpoints, the game loop and the limits.
- **Token:** OAuth device flow; the owner opens the link the session gives him, checks the code and accepts. The token lives in `~/.config/codingame/token`, dies with the sandbox, and is never printed or committed. The owner lists and revokes tokens at <https://www.codingame.com/settings/tokens>.
- **Bot programming puzzles** are under `/puzzles/<pretty_id>` (A Code of Ice and Fire: `a-code-of-ice-and-fire`): rules (`statement`, the owner's `league`), `stub-generator`, `test-games`, `submissions`, `progress`, `battles`, `leaderboard`, `opponents`. Rust's language id is `Rust`.
- **Test games** return a text log with every frame, our bot's stderr, each agent's stdout and the scores. Opponents: `boss`, `default`, `me`, or an `agent_id` from `opponents` (the owner's league and the lower ones). With `"opponents": ["boss"]`, our bot was agent 0 in all 7 games of 2026-10-31. Battle (ranking game) logs have no stderr.
- **Limits** for A Code of Ice and Fire on 2026-10-31 (`GET /limits?puzzle=<pretty_id>`): 120 requests a minute; test games 10 a minute, 20 per 5 minutes, 30 per 15 minutes, 80 an hour, 150 per 3 hours, 200 per 6 hours, 600 a day; submits 5 per 5 minutes, 20 per 30 minutes, 40 an hour, 100 per 5 hours. Every response carries the tightest window of each in `limits`.
- **Names outside ASCII compile:** `ice-and-fire-v014`, minified with CJK names and carrying rustc's dead-code warnings ([ADR 0046](adr/0046-one-minifier.md), decisions 3 and 6), compiled and played 7 test games without a timeout and was accepted at submission on 2026-10-31.

## Compilation mode

In the past, CodinGame compiled Rust in debug mode in the IDE and in release mode only for arena submissions ([forum](https://forum.codingame.com/t/rust-release-mode-compilation/33552?page=2)). A later announcement planned release mode everywhere ([forum](https://forum.codingame.com/t/languages-update/1574/223)). On 2026-10-14 the owner's probe built under **Play my code** had `debug_assertions` off, so the IDE no longer compiles a debug build. Its optimization level is not known. Do not tune time management from IDE runs.

## Submission limits

- **Code size:** 100,000 characters, not bytes. A top Ultimate Tic-Tac-Toe player's [write-up of 2026-09-20](https://tomalard.github.io/posts/fighting-for-1-in-the-ultimate-tic-tac-toe-arena/) says so, and a character outside ASCII counts as one. The owner's probe confirmed it for Rust on 2026-10-14: a bot of 95,713 characters and 125,475 bytes compiled, played in the IDE and was accepted at submission ([research/utf16-packing.md](research/utf16-packing.md#checked-on-codingame)). Earlier, players reported a strict 100 kB limit ([forum](https://forum.codingame.com/t/ultimate-tic-tac-toe-puzzle-discussion/22616?page=2)). `scripts/cg-check.sh` enforces 100,000 UTF-16 units, so a character above U+FFFF counts as two ([ADR 0037](adr/0037-size-limit-in-characters.md)); how CodinGame counts those is not checked.
- **One file:** the bundler (Phase 1) merges a bot and its workspace crates into one file.
- **Files:** the same write-up reports that a bot may write a file and run it. It also says that sending a compiled program this way is banned in active contests. Our bots are Rust source that CodinGame compiles.

## Processor

The owner's probe of 2026-10-14 in the Ultimate Tic-Tac-Toe IDE ([research/utf16-packing.md](research/utf16-packing.md#checked-on-codingame)) found:

- **Compiled for basic x86-64:** `cfg!(target_feature = ...)` was false for `avx2`, `fma`, `bmi2`, `popcnt` and `sse4.2`, so rustc uses only the instructions every x86-64 processor has.
- **The processor has more:** `is_x86_feature_detected!` was true for `avx2`, `fma`, `bmi2` and `popcnt`, and false for `avx512f`.

A bot can therefore use AVX2 only through a function compiled for it (`#[target_feature(enable = "avx2")]`) and called after a check at run time. That call needs `unsafe`, which takes an ADR. Arena games may run on other machines than the IDE's.

## Timing

CodinGame measures a turn from when it starts sending the turn's input until it reads the bot's output. Extra time before the first input exists in practice but is not guaranteed ([CodinGame staff on the forum](https://forum.codingame.com/t/timeouts-how-do-they-work/24504/4)). Most multiplayer games give 1 s on the first turn and 50 ms afterwards, but older games differ, so each game's `RULES.md` records its own limits.

A bot must not think during the opponent's turn ("pondering"): it is strictly forbidden (the owner, 2026-10-15, in the conversation). A bot searches only between reading its turn's input and printing its answer.
