# Optimal early plans: study run 44626073005

The files of the run read in [2026-11-19-optimal-early-plans.md](../2026-11-19-optimal-early-plans.md), copied from the branch `claude/study/44626073005`, which the study workflow wrote (`.github/workflows/study.yml`).

- Code: `claude/optimistic-rubin-my0tgi` at 4493de6 (`games/ice-and-fire/opening/examples/oracle.rs`).
- Arguments: `--suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 50 --turns 6,7,8 --limit 20000 --cap 20000`, 12 shards.
- Shards whole: 1, 2, 3, 6 and 11. The others (0, 4, 5, 7, 8, 9 and 10) were stopped by the 300-minute limit, and their rows so far are kept. That is 78 of the 94 starts, 234 rows.
- [`oracle.csv`](oracle.csv): a row a start and horizon (the columns are `HEADER` in the solver).
- [`oracle.plans.tsv`](oracle.plans.tsv): the best plan found for each start, horizon and goal (1,638), with whether it is proven. Its columns are `PLANS_HEADER` in the solver. `oracle --replay` plays all of them back with their stored values; `oracle --from` starts later searches from them.
- [`bench.md`](bench.md): `oracle --bench` on these plans, for the openings of `OPENINGS` (v029's, B, the rules alone, the assignment alone).
