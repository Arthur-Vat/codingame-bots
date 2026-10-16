# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023). Older detail is in this branch's history (before 2026-10-16 15:00 UTC the file kept every step).

## Waiting for the owner

None.

## Open questions

- **Next work, answered in the conversation, 2026-10-16 (form): "Tune PUCT (Recommended)".** Experiment **E032**: finer tuning of PUCT's `VALUE_TEMPERATURE` (0.1) and `PRIOR_WEIGHT` (0.25) in `games/uttt/bots/mcts/src/main.rs`, which E030 screened coarsely at 20 ms (six settings). Plan: screen settings around them at full time against `uttt-v017` (Claude's session, Rust 1.90.0, 4-ply openings, seed 1, 3 games at a time, 100 pairs each), then a candidate and its SPRT for the best one, else the entry dropped (skill run-experiment). Screened 16 settings at full time (100 pairs each, seed 1): temperatures 0.03 to 0.07 with weights 0.5 to 1.0 gave +33 to +60 Elo; confirmed on seed 2 (150 pairs): (0.05, 0.5) +47.8 [+23.3, +72.7], (0.03, 0.75) +40.7. **Candidate `uttt-v018` (0.05, 0.5) in #152** (opened 2026-10-16 about 16:30 UTC), SPRT running; then copy the result into the entry and ask the owner to approve the merge (or drop it), skill run-experiment.
- Still proposed, not decided (2026-10-16), each one experiment for the owner's go-ahead: trainer item 2, fits on several Train runs' data at once (E024's data, run 38799715918, expires about 2026-10-22; run 38875273472's about 2026-10-23; but E028 showed v016's data hurt, -81); D3 accumulator checkpoints at nodes with many visits (the walk's updates take about a fifth of the time); trainer item 4, a hyperparameter sweep of the fit in Train (sample of 10M positions, parallel jobs; the SPRT stays the judge). Also: a second, larger network only for nodes with many visits (later); D4 symmetry in the first layer; a learned policy (does not fit today).
- Owner asked (2026-10-16) about AWS spot (about 22,000 vCPU-hours, $220 to $550 for 100M games) and home labs (Mac mini M4 / M4 Pro, NUC 15 Pro+): estimates given, advised against for now; a self-hosted runner on a public repository needs an ADR.
- Phase 7's second game: later (owner, 2026-10-13); the owner names it (skill `new-game`).

## Follow-ups

- Self-play's progress lines (owner, 2026-10-15, "for later"): each of a job's 4 processes prints `N games, P positions, T s` unnamed. Make each say its core and share, no time-left estimate (answered 2026-10-15: "About 2h left is too much, but ok for the rest"), e.g. `core 2: 10,800 of 25,000 games (43%), 528,079 positions, 5,649 s`. Can ride the next trainer pull request.
- Width plan (agreed 2026-10-15): entropy coder (rANS) in cg-pack/cg-packer, 5- or 6-bit weights and width 1,024 only if a width-512 network wins; it has not (E019, E025). A minifier would need an ADR first (it changes what a release file is).
- Retrospective proposals in games/uttt/RETROSPECTIVE.md wait for the owner's choice; the first two only when a game needs them.
- Studio: the front end re-serialises records on export/import and the review page rounds 64-bit seeds (JavaScript numbers); the WebAssembly files are built twice in CI's front-end job; `actions/deploy-pages@v4` targets the deprecated Node 20 (move when a newer major exists); the e2e arena tests hard-code port 4173; the chart's per-bot dashed lines wait for releases that report their own evaluation; "Back to the game" shares its name with the top-left link (rename only if asked). No Firefox testing (owner, 2026-10-13).
- Scheduled workflows stop after 60 days without commits in a public repository.
- ADR 0023 still says the SPRT is not a required check; the owner keeps it as is (2026-10-09). ADR 0010's context still says sessions cannot download Rust toolchains (they can since 2026-10-16): supersede only if a rule changes.
- Never ask the owner for CodinGame ranks (owner, 2026-10-13).

## Notes

- Releases: `uttt-v017` is current (E030 PUCT, +58.5 Elo over v016), delivered to the owner as a file. Lineage since the NNUE: v011 (E017), v012 (E018 jacekmax), v013 (E020 minimax 0.5), v014 (E021 exploration 0.2), v015 (E022 walk updates), v016 (E024 side-game data), v017 (E030 PUCT).
- Lessons: move agreement with the search's move tracked strength, the fit's error did not (E019, E023, E028); data from v016's self-play hurt (E028, -81) while the fit's target barely matters (E031); width 512 did not win (E019, E025), 128 lost (E027). Screen at full time: 20 ms screens time out in a third of games on Claude's session. Measure speed and screen with Rust 1.90.0 (`rustup toolchain install 1.90.0`; a newer compiler runs the same code at another speed, E026).
- Constraints from the owner: SSE2 only (CodinGame compiles for basic x86-64), no `unsafe` without an ADR, no AVX2 (2026-10-14), no pondering and no time budget change (2026-10-15). Every packing tool is Rust (2026-10-14). Train runs need the owner's go-ahead and his choice of size.
- Trainer facts: `fit-nnue` uses Adam 0.001 falling to a twentieth (cosine), batches of 1,024, random symmetry, threads kept through the fit, `-C target-cpu=native` in Train; Train's nnue self-play plays as the newest release (PUCT flags in train.yml), stops at `minutes`, side games via `side_games`, openings drawn at temperature 2, copies dropped by the fit. The bot has a value network only (NNUE, width 256). Train run reports live on `claude/train/<run>` until copied into a pull request (skill training-run).
- CI scoped by area and game (ADR 0029); the ruleset requires `CI passed`, `SPRT passed` and `Pull request title and labels`. Job logs are readable from sessions (ADR 0028, skill `github-api`). The exact network entry the owner allowed stays out of the repository, this file included (the branch is public). In cloud sessions, `sudo dockerd` starts Docker, so actionlint runs locally.
- The studio is on GitHub Pages (https://arthur-vat.github.io/codingame-bots/) and runs locally too; owner's wording rule (2026-10-12): no text that brings no information; a release is a file, the bot built from it plays. Phase 6 is done.
