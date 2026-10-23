# Metrics report: ice-and-fire-v005 against ice-and-fire-v004

The first metrics report of a large batch (owner, 2026-10-23; [ADR 0044](../../../../docs/adr/0044-metrics-reports.md), skill `review-games`, "Metrics reports").

- **Bots:** [`ice-and-fire-v005`](../../releases/ice-and-fire-v005.rs) (the current release) against its parent [`ice-and-fire-v004`](../../releases/ice-and-fire-v004.rs), both compiled with Rust 1.90.0.
- **Games:** 500 seat-swapped pairs (1,000 games), seed 234, at CodinGame's time limits, two games at a time: v005 won 567, v004 433 (+46.8 Elo [+26.2, +67.8]); 4 timeouts of v005 and 1 of v004, no crash or invalid answer.
- **Tool:** `ice-and-fire-review` at commit cc891c8: 50 metrics a side at every ply; the report by slice (length thirds, seatings, each bot's wins, whether the leader at turn 10 kept its lead), from the start and on each game's last turn.
- **Report:** [report.md](report.md), computed by code. **Interpretation:** [summary.md](summary.md), by the `review-summarizer` agent (Haiku), which read the report alone, saved unchanged by the main session.
- **Charts:** a private page with every metric's curves by slice, from the start or before the end, and a heatmap of when each metric tells the winner: https://claude.ai/artifact/SaCRkqyMF8Ng5KfLG8EQSQ (the owner shares it from its Share menu).

The records and the games' files are not kept: the seed rebuilds them, and the tool rebuilds the report.

```sh
target/release/ice-and-fire-arena match --bot v005=BIN/v005 --bot v004=BIN/v004 --pairs 500 --seed 234 --jobs 2 --records RECORDS
cargo run --release -p ice-and-fire-review -- games --out GAMES --text TEXT RECORDS/*.json
cargo run --release -p ice-and-fire-review -- report GAMES --curves curves.json > report.md
```
