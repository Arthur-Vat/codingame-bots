---
name: rules-check
description: Builds a game's rules-check bot, which plays on CodinGame and compares, every turn, what CodinGame sends with what our engine predicts, and guides the owner in running it. Use at stage 5 of the new-game lifecycle, and whenever a game's engine changes how it reads the rules.
---

# Check our engine against CodinGame

The example is Ultimate Tic-Tac-Toe's [rules-check bot](../../../games/uttt/bots/rules-check/) and its README section "Checking the rules on CodinGame" ([games/uttt/README.md](../../../games/uttt/README.md)).

## The bot

`games/<game>/bots/rules-check/`, bundled like any bot:

- It plays legal moves at random (seeded, the seed printed to stderr).
- Every turn, it replays the moves seen so far in our engine and compares what the engine predicts with what CodinGame sends: the legal actions, and whatever else the input reveals (positions, scores, resources).
- When its own move ends the game in the engine's view, it prints the expected result, so the owner can compare with CodinGame's end screen.
- One line per turn to stderr ending with running totals (`N turns checked, M with differences`); each difference on a line starting with `rules-check: DIFFERENCE`, with both sides.
- With `CG_RULES_CHECK_STRICT` set, the first difference exits with an error, so CI can play it against the referee and prove it agrees with our own rules.

The input must be read in full every turn, and stdout flushed after every move (`CLAUDE.md`).

## On CodinGame

Ask the owner, in one message:

1. which file to paste (the bundle in `target/cg/`, or attached to the pull request) and in which league (where the full rules apply);
2. to click **Play my code** a few times, against different opponents;
3. to paste back the last stderr line of each game, every `rules-check: DIFFERENCE` line, and the result on the end screen.

## After

- No difference: record it in `games/<game>/RULES.md` ("Checked on CodinGame", with the date, the league and the number of turns), as in Ultimate Tic-Tac-Toe's.
- A difference: find which side is right from the statement and the source; fix the referee and engine with a test that reproduces the case, then ask for another run.
