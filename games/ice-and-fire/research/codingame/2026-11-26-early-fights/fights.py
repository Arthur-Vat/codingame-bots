"""v032's fights before game turn 12 in its CodinGame games (E083).

Input: the JSON lines of `arena/examples/plies.rs` (the board at the start
of every ply up to game turn 13, red's view) and the data directory's
index (our seat and the result). For each of our plies, what the enemy's
next ply took from us: cells captured, cells cut off from our HQ, the
cells we had just taken, our units; and every large loss (3 or more cells
in one enemy ply) with each side's gold and what the enemy trained.

Usage: python3 fights.py PLIES.jsonl INDEX.jsonl RECORDS_DIR
"""
import collections
import json
import statistics as st
import sys

plies_path, index_path, records = sys.argv[1:4]
idx = {str(json.loads(l)['game_id']): json.loads(l) for l in open(index_path)}
plies = collections.defaultdict(list)
for line in open(plies_path):
    d = json.loads(line)
    if d['cmd'] is None:
        plies[d['game']].append(d)


def own(board, side, active=False):
    chars = ('O' if active else 'Oo') if side == 0 else ('X' if active else 'Xx')
    return {(x, y) for y, r in enumerate(board['rows']) for x, ch in enumerate(r) if ch in chars}


rows, big = [], []
for g, P in plies.items():
    d = idx[g]
    us, them = d['our_seat'], 1 - d['our_seat']
    won = d['ranking'][0] == us
    P.sort(key=lambda p: p['ply'])
    first_seen = {}
    for p in P:
        for c in own(p['board'], us):
            first_seen.setdefault(c, p['turn'])
    for i in range(len(P) - 2):
        a, b, c = P[i], P[i + 1], P[i + 2]
        if a['seat'] != us or b['seat'] != them or c['seat'] != us or a['turn'] > 12:
            continue
        oa, ob = own(a['board'], us), own(b['board'], us)
        captured = ob & own(c['board'], them)
        cut = (own(b['board'], us, True) - own(c['board'], us, True)) - captured
        took = ob - oa
        units_b = {u[3] for u in b['board']['units'] if u[2] == us}
        units_c = {u[3] for u in c['board']['units'] if u[2] == us}
        rows.append(dict(game=g, won=won, turn=a['turn'], took=len(took),
                         captured=len(captured), cut=len(cut),
                         retaken=len(took & own(c['board'], them)),
                         lost_units=len(units_b - units_c)))
        lost = captured | cut
        if len(lost) >= 3:
            turns = json.load(open(f'{records}/{g}.json'))['turns']
            answer = ';'.join(turns[b['ply']][0]['lines']) if b['ply'] < len(turns) else ''
            trains = [x for x in answer.split(';') if x.strip().startswith('TRAIN')]
            big.append(dict(won=won, turn=b['turn'], lost=len(lost), cut=len(cut),
                            trains=len(trains), egold=b['board']['gold'][them],
                            gold_before=a['board']['gold'][us], gold_after=b['board']['gold'][us],
                            age=st.median(b['turn'] - first_seen.get(x, b['turn']) for x in lost)))

for label, want in (('won', True), ('lost', False)):
    R = [r for r in rows if r['won'] == want]
    n = len({r['game'] for r in R})
    per = lambda k: sum(r[k] for r in R) / n
    print(f"{label}: {n} games; per game to turn 12: captured {per('captured'):.1f}, cut off {per('cut'):.1f}, "
          f"taken back at once {per('retaken'):.1f}, units lost {per('lost_units'):.2f}")
m = lambda k: st.median(x[k] for x in big)
print(f"large losses: {len(big)}; median cells {m('lost')}, cut off {m('cut')}, enemy trainings {m('trains')}, "
      f"enemy gold {m('egold')}, our gold before our ply {m('gold_before')} and after it {m('gold_after')}, "
      f"age of the lost cells {m('age')} turns; our gold after our ply at least 15: "
      f"{sum(1 for x in big if x['gold_after'] >= 15) / len(big):.0%}")
