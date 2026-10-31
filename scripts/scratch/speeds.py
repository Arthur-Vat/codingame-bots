# Reads the arena's output with --show-bot-stderr; prints the timing bot's
# workload speeds (units a millisecond): turn 1 (500 ms alone) and the holds.
import re, statistics, sys
games = {}
for line in sys.stdin:
    if not line.startswith("timing v1 "):
        continue
    head, _, turns = line.rpartition("| ")
    entries = turns.split()
    if not entries or not entries[0].startswith("1:"):
        continue
    key = (head, entries[0].rsplit("/", 2)[0])  # seat, map and turn 1's timings
    if len(entries) > len(games.get(key, [])):
        games[key] = entries
first, holds = [], []
for entries in games.values():
    for e in entries:
        n, _, rest = e.partition(":")
        speed = rest.split("/")[-1]
        if speed == "-":
            continue
        (first if n == "1" else holds).append(float(speed))
def q(xs, p):
    xs = sorted(xs); return xs[min(len(xs) - 1, int(p * len(xs)))]
label = sys.argv[1] if len(sys.argv) > 1 else "speed"
for name, xs in (("first turn", first), ("holds", holds)):
    if xs:
        print(f"{label} {name}: n {len(xs)}, median {statistics.median(xs):.1f}, "
              f"min {min(xs):.1f}, 10% {q(xs, .1):.1f}, 90% {q(xs, .9):.1f}, max {max(xs):.1f}")
