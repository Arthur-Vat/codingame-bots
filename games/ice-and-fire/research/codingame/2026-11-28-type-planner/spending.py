"""Counts zasmu's purchases by turn band and gold band, checks them on held-out
games and writes the table the bot packs (stage 6 of the type planner,
2026-11-28-type-planner.md, E109).

    ice-and-fire-bot-plan spending --positions zasmu.bank > buys.jsonl
    python3 -I spending.py buys.jsonl --json spending.json
    ice-and-fire-bot-plan spending-table --json spending.json --out games/ice-and-fire/bots/plan/src/types/spending/table.rs

Standard library only. `buys.jsonl` is the output of the plan bot's `spending`
mode: a line a position, with the turn band, the gold band of the side to act
and the kind of each of its purchases. Only positions from game turn 5 count.
A row (turn band, gold band) with fewer than MIN_CASES replies takes the
counts of its turn band, all gold bands together.
"""
import argparse
import json
import math
import random
import sys

BUYS = ['t1-capture', 't1-front', 't1-near', 't1-far',
        't2-capture', 't2-front', 't2-near', 't2-far',
        't3-capture', 't3-front', 't3-near', 't3-far',
        'tower-joint', 'tower-next-joint', 'tower-front', 'tower-near', 'tower-far',
        'mine']
BUY = {k: i for i, k in enumerate(BUYS)}
BANDS = ['5-6', '7-8', '9-12', '13-20', '21+']
GOLDS = ['kill', '<40', '40-79', '80+']
MIN_CASES = 20
FROM_TURN = 5


def read(path):
    """The replies from FROM_TURN: (game, turn, row, purchase kind indices)."""
    out = []
    with open(path) as f:
        for line in f:
            p = json.loads(line)
            if p['turn'] < FROM_TURN:
                continue
            row = BANDS.index(p['band']) * 4 + GOLDS.index(p['gold'])
            out.append((p['game'], p['turn'], row, [BUY[b] for b in p['buys']]))
    return out


def count(replies):
    """counts[row] = [replies, replies that buy nothing, purchases of each kind]."""
    counts = [[0, 0] + [0] * len(BUYS) for _ in range(20)]
    for _, _, row, buys in replies:
        c = counts[row]
        c[0] += 1
        c[1] += not buys
        for b in buys:
            c[2 + b] += 1
    return counts


def pooled(counts, row):
    """The row's counts, or its turn band's when it has too few replies."""
    if counts[row][0] >= MIN_CASES:
        return counts[row]
    band = row // 4
    total = [0] * len(counts[0])
    for r in range(band * 4, band * 4 + 4):
        total = [a + b for a, b in zip(total, counts[r])]
    return total


def shares(c):
    """[share of replies that buy nothing] + [share of each kind among the
    purchases], as fractions."""
    n = sum(c[2:])
    nothing = c[1] / c[0] if c[0] else 0.0
    return [nothing] + [x / n if n else 0.0 for x in c[2:]]


def ranked(row):
    return sorted(range(len(BUYS)), key=lambda k: (-row[1 + k], k))


def held_out(replies):
    games = sorted({r[0] for r in replies})
    random.Random(1).shuffle(games)
    test = set(games[:len(games) // 5])
    train = [r for r in replies if r[0] not in test]
    held = [r for r in replies if r[0] in test]
    print(f'{len(games)} games, {len(games) - len(test)} to count and {len(test)} held out; '
          f'{len(train)} replies to count, {len(held)} held out (from game turn {FROM_TURN})')
    counts = count(train)
    tab = [shares(pooled(counts, row)) for row in range(20)]
    hits = [0] * 4
    purchases = 0
    loss = 0.0
    for _, _, row, buys in held:
        order = ranked(tab[row])
        for b in buys:
            purchases += 1
            for k in range(4):
                hits[k] += b in order[:k + 1]
            loss -= math.log(max(tab[row][1 + b], 1e-3))
    print(f'\n{purchases} purchases held out')
    print('top-1   top-2   top-3   top-4   log-loss (shares floored at 0.001)')
    print('  '.join(f'{100 * h / purchases:5.1f}%' for h in hits) + f'   {loss / purchases:.4f}')
    print('\nreplies that buy nothing, by gold band (held out; counted):')
    print('band    ' + '  '.join(f'{g:>13}' for g in GOLDS))
    for band, name in enumerate(BANDS):
        cells = []
        for g in range(4):
            row = band * 4 + g
            mine = [r for r in held if r[2] == row]
            real = sum(not r[3] for r in mine) / len(mine) if mine else float('nan')
            cells.append(f'{100 * real:4.0f}% ({len(mine):4d})/{100 * tab[row][0]:3.0f}%')
        print(f'{name:>6}  ' + '  '.join(cells))
    print('\nall games, by gold band: ' + ', '.join(
        f'{g} {100 * sum(not r[3] for r in replies if r[2] % 4 == i) / max(1, sum(r[2] % 4 == i for r in replies)):.0f}%'
        for i, g in enumerate(GOLDS)))
    return counts


def table(counts):
    """The 20 rows: nothing share, then the 18 kinds' shares."""
    return {'rows': [[round(x, 6) for x in shares(pooled(counts, row))] for row in range(20)]}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('buys', help='the output of the bot\'s `spending` mode')
    ap.add_argument('--json', help='write the table of all games here')
    args = ap.parse_args()
    replies = read(args.buys)
    held_out(replies)
    counts = count(replies)
    thin = [row for row in range(20) if counts[row][0] < MIN_CASES]
    print(f'\nrows with fewer than {MIN_CASES} replies on all games (pooled by turn band): {thin}')
    if args.json:
        with open(args.json, 'w') as f:
            json.dump(table(counts), f, separators=(',', ':'))
            f.write('\n')


if __name__ == '__main__':
    sys.exit(main())
