# 0060. Probe submissions: an experiment's bot judged on CodinGame

- Status: accepted (owner, 2026-11-19)
- Date: 2026-11-19
- Supersedes: [0047](0047-codingame-api.md), decisions 4 and 5, for probes (a session submits only a frozen release; a rank is recorded in the release's entry)
- Scope: framework

## Context

ADR 0047 lets a session submit only a frozen release, and a release exists only when its SPRT against the previous release accepts it. The SPRT is self-play: it judges a bot against our own last release. The curriculum of capabilities for A Code of Ice and Fire ([research/codingame/2026-11-18-zasmu-strategy.md](../../games/ice-and-fire/research/codingame/2026-11-18-zasmu-strategy.md)) asks for judges outside self-play, because a change aimed at the top players' play can be invisible against our own bot.

Experiment E069 showed such a case on 2026-11-19. Its bot keeps a strike on the enemy HQ affordable next turn in all the capability tests drawn from the top player's positions (against 51% for the release and 71% for that player), and screened level with the release in self-play (−6.5 Elo, interval −16.8 to +3.7, over 800 pairs). The release itself defends against such a strike more than the top player does, so self-play may hide what the change is worth. The owner chose to submit the experiment's bot to CodinGame and read its ranking games, and to record that as a rule for such probes (form, 2026-11-19).

## Decision

1. **A session may submit an experiment's bot that is not a release, a probe, on the owner's approval in the conversation of that probe.** The approval follows [ADR 0023](0023-chat-approved-merges.md). A probe is for a change whose value self-play cannot show, and its approval covers the resubmission of decision 4.
2. **The file submitted is frozen in the repository before the submission.** It is the minified bundle of the experiment's commit, at `games/<game>/probes/<game>-eNNN-<variant>.rs`, committed and pushed on the experiment's branch and submitted unchanged from that commit. It is merged with the experiment's journal entry whatever the experiment's outcome, stays when a dropped experiment's bot change is removed, and never changes afterwards, like a release file. Its journal entry names the commit and the file's SHA-256.
3. **The probe's submission line in `games/<game>/codingame-submissions.txt` carries the probe's name**, so the collection attributes its ranking games to it and they are read like a release's: the capability tests and the style measures with `--release <name>`, and its rank through the API.
4. **After its ranking, the better ranked of the probe and the release it replaced stays submitted.** If the probe ranks lower, the session submits that release's file again.
5. **A probe is never a release.** Its rank and its games are recorded in its experiment's journal entry; a new release still comes only from an accepted SPRT, and the owner decides what follows from a probe's result.

## Consequences

- A change aimed at the top players' play can be judged against them, in ranking games, when self-play cannot judge it.
- A probe takes the owner's place on the leaderboard for a ranking, and a rank varies from one submission to the next: read a probe's rank beside its games, not alone.
- Each probe uses two submissions when it ranks lower (itself and the release again), from the per-user limits that ADR 0047 shares with the owner's own use.
- `scripts/check-releases.sh` holds the probe files frozen as it does the releases.
- The collection's daily test games still play the newest release ([ADR 0059](0059-collection-follows-the-newest-release.md)); only the ranking games are the probe's.
