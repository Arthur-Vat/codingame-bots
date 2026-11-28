"""Counts zasmu's kinds of action by unit type, at five levels of detail, checks
them on held-out games and writes the table the bot packs (stage 1 of the
type planner, 2026-11-28-type-planner.md).

    ice-and-fire-review codingame-judge --data DATA --me Tarleya --players zasmu --bank zasmu.bank
    ice-and-fire-bot-plan types --positions zasmu.bank > units.jsonl
    python3 -I tables.py units.jsonl --json table.json
    ice-and-fire-bot-plan types-table --json table.json --out games/ice-and-fire/bots/plan/src/types/table.rs
    ice-and-fire-bot-plan types --positions zasmu.bank > units-rust.jsonl
    python3 -I tables.py units.jsonl --check units-rust.jsonl

Standard library only. `units.jsonl` is the output of the plan bot's `types`
mode: a line a position, [type, level, turn band, gold band, cap, en2, own2,
tw2, d, dj, onj, free1, adj, kind, top3] a unit, raw numbers. Only positions
from game turn 5 count. The clamps are applied here, as `Key::fields` does in
the bot.
"""
import argparse
import json
import math
import random
import sys

KINDS = ['capture', 'forward-n', 'forward-o', 'side-n', 'side-o', 'back-n', 'back-o', 'stay']
KIND = {k: i for i, k in enumerate(KINDS)}
BANDS = ['5-6', '7-8', '9-12', '13-20', '21+']
# How many leading fields a key has at each level L0 to L4.
LENGTHS = [2, 3, 4, 8, 13]
MIN_CASES = 20
FROM_TURN = 5


def fields(u):
    """The clamped fields of a unit: type, level (1 or 2), turn band, gold
    band, cap, en2, own2, tw2, d, dj, onj, free1, adj."""
    typ, level, band, gold, cap, en2, own2, tw2, d, dj, onj, free1, adj = u[:13]
    return (typ, min(level, 2), band, gold, min(cap, 1), min(en2, 2), min(own2, 2),
            min(tw2, 1), min(d, 4), min(dj, 3), onj, min(free1, 2), min(adj, 2))


def read(path):
    """The units of positions from FROM_TURN: (game, turn, fields, kind index,
    the bot's top3 kinds)."""
    out = []
    with open(path) as f:
        for line in f:
            p = json.loads(line)
            if p['turn'] < FROM_TURN:
                continue
            for u in p['units']:
                out.append((p['game'], p['turn'], fields(u), KIND[u[13]], u[14]))
    return out


def count(units):
    """counts[level][key] = the number of each kind."""
    counts = [{} for _ in LENGTHS]
    for _, _, f, kind, _ in units:
        for level, n in enumerate(LENGTHS):
            row = counts[level].setdefault(f[:n], [0] * len(KINDS))
            row[kind] += 1
    return counts


def probs(row):
    total = sum(row)
    return [(c + 0.5) / (total + len(KINDS) / 2) for c in row]


def predict(counts, f, level):
    """The probabilities of the finest level up to `level` whose key has at
    least MIN_CASES cases, else the coarsest known."""
    for lv in range(level, -1, -1):
        row = counts[lv].get(f[:LENGTHS[lv]])
        if row is not None and sum(row) >= MIN_CASES:
            return probs(row)
    return probs(counts[0].get(f[:LENGTHS[0]], [0] * len(KINDS)))


def ranked(p):
    return sorted(range(len(p)), key=lambda k: (-p[k], k))


def held_out(units):
    games = sorted({u[0] for u in units})
    random.Random(1).shuffle(games)
    test = set(games[:len(games) // 5])
    train = [u for u in units if u[0] not in test]
    held = [u for u in units if u[0] in test]
    print(f'{len(games)} games, {len(games) - len(test)} to count and {len(test)} held out; '
          f'{len(train)} units to count, {len(held)} held out (from game turn {FROM_TURN})')
    counts = count(train)
    print('\nlevel  top-1   top-2   top-3   top-4   log-loss')
    by_band = {}
    for level in range(len(LENGTHS)):
        hits = [0] * 4
        loss = 0.0
        band = {b: [0, 0] for b in BANDS}
        for _, _, f, kind, _ in held:
            p = predict(counts, f, level)
            order = ranked(p)
            for k in range(4):
                hits[k] += kind in order[:k + 1]
            loss -= math.log(p[kind])
            band[f[2]][0] += order[0] == kind
            band[f[2]][1] += 1
        n = len(held)
        print(f'L{level}    ' + '  '.join(f'{100 * h / n:5.1f}%' for h in hits)
              + f'   {loss / n:.4f}')
        by_band[level] = band
    print('\ntop-1 by turn band')
    print('level  ' + '  '.join(f'{b:>6}' for b in BANDS))
    for level in range(len(LENGTHS)):
        print(f'L{level}     ' + '  '.join(
            f'{100 * by_band[level][b][0] / max(by_band[level][b][1], 1):5.1f}%' for b in BANDS))
    print('units by turn band: ' + ', '.join(f'{b} {by_band[0][b][1]}' for b in BANDS))


def table(counts):
    """Every key with at least MIN_CASES cases at each level, with its
    probabilities."""
    levels = []
    for level in range(len(LENGTHS)):
        rows = []
        for key, row in sorted(counts[level].items()):
            if sum(row) >= MIN_CASES:
                rows.append([list(key), [round(p, 6) for p in probs(row)]])
        levels.append(rows)
    return {'levels': levels}


def check(counts, path):
    """The bot's top3 against the table's, as sets."""
    tab = [{tuple(key): p for key, p in rows} for rows in table(counts)['levels']]
    agree = total = 0
    for _, _, f, _, top in read(path):
        total += 1
        for level in range(len(LENGTHS) - 1, -1, -1):
            p = tab[level].get(f[:LENGTHS[level]])
            if p is not None:
                break
        else:
            p = None
        want = {KINDS[k] for k in ranked(p)[:3]} if p else set()
        agree += want == set(top)
    print(f'the bot\'s top3 agrees with the table\'s on {agree} of {total} units '
          f'({100 * agree / max(total, 1):.2f}%)')
    return agree / max(total, 1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('units', help='the output of the bot\'s `types` mode')
    ap.add_argument('--json', help='write the table of all games here')
    ap.add_argument('--check', help='the bot\'s `types` output with the embedded table, to compare')
    args = ap.parse_args()
    units = read(args.units)
    if args.check:
        check(count(units), args.check)
        return
    held_out(units)
    counts = count(units)
    tab = table(counts)
    print('\nkeys with at least %d cases, on all games: ' % MIN_CASES
          + ', '.join(f'L{i} {len(rows)}' for i, rows in enumerate(tab['levels'])))
    if args.json:
        with open(args.json, 'w') as f:
            json.dump(tab, f, separators=(',', ':'))
            f.write('\n')


if __name__ == '__main__':
    sys.exit(main())
