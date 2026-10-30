---
id: E030
date: 2026-10-30
pull_request: "#230"
parent: ice-and-fire-v014
hypothesis: A plan whose score against the memory is already below the best score of a plan whose replies are known cannot become the best, since its own replies can only lower it; skipping its reply search (about a third of a turn goes to reply searches) leaves more time for climbs and beats v014.
change: None released. Local screening only: the climbs return their plan's exact score against the memory; a plan below the best known (a plan's score with its own replies) gets no reply search (skip1), or only the cheap replies, its kill chain, cuts and bronze's greedy turn without the opponent's own climb (skip2).
release: none
sprt: not run
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-30 ("Indeed skip it!"), step 5 of the time
roadmap (phase 3: use time better).

## Why

In v011's searches the opponent's replies took 35 to 40% of a turn (its own
climb most of it). A plan scored below the best plan whose replies are known
seemed unable to become the one played. Not exactly: the memory grows during
the turn and can lower the best plan too, so a skipped plan could rarely
end up best; E027's final check would still search its replies then.

## Screening

Against `ice-and-fire-v014` (#229's candidate) at CodinGame's limits,
400 seat-swapped pairs each, seed 30030, two games at a time on a 4-core
machine:

| Variant | Elo against v014 | Timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | --- |
| skip1: no reply search | −0.9 [−21.0, +19.2] | 2 (v014 6) | 24.9, 40.4, 41.8 ms (v014 36.3, 39.6, 42.5) |
| skip2: cheap replies only | **−29.6 [−49.5, −9.9]** | 2 (v014 4) | 30.7, 39.7, 42.4 ms (v014 36.7, 39.6, 41.5) |

## What was learned

- The replies of a plan that cannot win are not wasted: they also teach the
  memory that later climbs plan against. Without them the search learns
  nothing new sooner, runs out of first actions, and its turns end earlier
  (median 25 ms against 36): the time saved goes unused.
- Keeping only the cheap replies is worse still: the memory then fills with
  weaker replies, and the plans are judged against them.
- With E029's forced climbs, a turn now ends at the soft limit in most
  cases; what fills the time matters more than freeing it.

The variants' code is in commit 734a45e of this pull request (`SKIP` in
`search.rs`, 1 and 2), removed by the next commit, with #228's review
tidy-ups (they change no answer: the speed bank's 200 answers stayed v014's
with `SKIP` 0).
