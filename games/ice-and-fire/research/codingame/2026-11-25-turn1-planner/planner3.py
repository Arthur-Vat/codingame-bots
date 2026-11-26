"""Turn-1 planner, version 3 (E080), for pictures only: no enemy.

The owner, 2026-11-26: per area three sub-goals, the entrance (E), the
front centre (C) and an outpost (O) between the centre and the opponent;
the most advanced units' advance is part of the macro decisions (a unit can
serve two goals until the fork); among close plans, avoid building along
void cells; maximize the gold earned in the long run: units should not end
inside our land (coordinated snakes that must not collide).

- Macro: for each of the first K trainings, one option among: a single
  sub-goal (area, E/C/O), an ordered pair of front centres (the unit steps
  nearer both while it can, then commits to the first), or free growth.
  On reaching its sub-goal a unit goes on to its area's next one (E, C, O),
  then grows freely.
- Micro: a unit steps to a neighbour nearer its sub-goal(s), a new cell
  first, then the cell with the most valid neighbours (not along the void),
  then the lowest cell. A training goes on the free cell next to our land
  nearest its sub-goal, the most open cell on a tie.
- Objective: the most land plus room at game turn T+1 (each unit's room:
  the free cells nearer to it than to any other unit of ours, up to 3), then
  the Voronoi share against the enemy's starting land, then the fewest of
  our cells along the void.
"""
import json, os, sys, itertools, time
from collections import deque

P = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(P)
T = 9
K = int(sys.argv[1]) if len(sys.argv) > 1 else 4
ROOM_CAP = 3
G = json.load(open(P + '/plans2.json'))  # written by planner2.py
DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))


def bfs(rows, src):
    d = {}
    q = deque()
    for c in src:
        d[c] = 0
        q.append(c)
    while q:
        x, y = q.popleft()
        for dx, dy in DIRS:
            a, b = x + dx, y + dy
            if 0 <= a < 12 and 0 <= b < 12 and rows[b][a] != '#' and (a, b) not in d:
                d[(a, b)] = d[(x, y)] + 1
                q.append((a, b))
    return d


def central_path(rows, start, goal):
    d = bfs(rows, [goal])
    if start not in d:
        return []
    out = [start]
    while d[out[-1]] > 0:
        x, y = out[-1]
        out.append(min(((x + dx, y + dy) for dx, dy in DIRS if (x + dx, y + dy) in d and d[(x + dx, y + dy)] == d[out[-1]] - 1), key=lambda c: (abs(c[0] - c[1]), c)))
    return out


for g in G:
    rows = g['boards'][0]['rows']
    O = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'O']
    X = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'X']
    valid = [(x, y) for y in range(12) for x in range(12) if rows[y][x] != '#']
    vset = set(valid)
    opened = {c: sum(1 for dx, dy in DIRS if (c[0] + dx, c[1] + dy) in vset) for c in valid}
    dO, dX = bfs(rows, O), bfs(rows, X)
    # Sub-goals per area.
    subs = []  # (area index, kind, cell)
    for ai, a in enumerate(g['areas']):
        E, C = tuple(a['entrance']), tuple(a['centre'])
        p = central_path(rows, C, (11, 11))
        Ocell = p[2] if len(p) > 2 else C
        for kind, cell in (('E', E), ('C', C), ('O', Ocell)):
            subs.append((ai, kind, cell))
        a['sub'] = {'E': list(E), 'C': list(C), 'O': list(Ocell)}
    cells = sorted({s[2] for s in subs})
    dist = {c: bfs(rows, [c]) for c in cells}
    # Options: single sub-goals (deduplicated cells), ordered pairs of
    # centres, free.
    singles = []
    seen = set()
    for s in subs:
        if (s[0], s[2]) not in seen:
            seen.add((s[0], s[2]))
            singles.append([s])
    centres = [s for s in subs if s[1] == 'C']
    pairs = [[a, b] for a, b in itertools.permutations(centres, 2)]
    options = singles + pairs + [None]

    def nxt_sub(ai, after):
        order = [s for s in subs if s[0] == ai]
        kinds = [s[1] for s in order]
        i = kinds.index(after) + 1 if after in kinds else 0
        return order[i] if i < len(order) else None

    def simulate(seq, log=None):
        L = set(O)
        units = []  # dict(pos, goals: list of subs)
        gold = 20
        n = 0
        for t in range(1, T + 1):
            gold += len(L) - len(units)
            if log is not None:
                log.append(dict(t=t, kind='turn', gold=gold, income=len(L) - len(units)))
            occ = {u['pos'] for u in units}
            for u in units:
                # Arrival: on to the area's next sub-goal.
                # A sub-goal already ours (reached by this unit or another)
                # is done.
                while u['goals'] and u['goals'][0][2] in L:
                    first = u['goals'][0]
                    rest = u['goals'][1:]
                    nx = nxt_sub(first[0], first[1])
                    u['goals'] = ([nx] if nx else []) if not rest else rest
                    if log is not None:
                        log.append(dict(t=t, kind='retarget', at=list(u['pos']), goals=[[s[0], s[1]] for s in u['goals']]))
                pos = u['pos']
                if u['goals']:
                    ds = [dist[s[2]] for s in u['goals']]
                    here = [d.get(pos, 99) for d in ds]
                    nb = [(pos[0] + dx, pos[1] + dy) for dx, dy in DIRS]
                    nb = [c for c in nb if c in vset and c not in occ]
                    both = [c for c in nb if all(d.get(c, 99) < h for d, h in zip(ds, here))]
                    if both:
                        steps = both
                    else:
                        if len(u['goals']) > 1:
                            u['goals'] = u['goals'][:1]  # the fork: commit to the first
                            if log is not None:
                                log.append(dict(t=t, kind='fork', at=list(pos), goals=[[s[0], s[1]] for s in u['goals']]))
                        steps = [c for c in nb if ds[0].get(c, 99) < here[0]]
                else:
                    du = bfs(rows, [pos])
                    rest = [c for c in du if c not in L and c not in occ]
                    if not rest:
                        continue
                    goal = min(rest, key=lambda c: (du[c], abs(dO.get(c, 99) - dX.get(c, 99)), c))
                    dg = bfs(rows, [goal])
                    steps = [c for c in ((pos[0] + dx, pos[1] + dy) for dx, dy in DIRS) if c in vset and c not in occ and dg.get(c, 99) < dg.get(pos, 99)]
                if not steps:
                    continue
                c = min(steps, key=lambda c: (c in L, -opened[c], c))
                if log is not None:
                    log.append(dict(t=t, kind='move', frm=list(pos), to=list(c), goals=[[s[0], s[1]] for s in u['goals']], gold=gold))
                occ.discard(pos)
                occ.add(c)
                u['pos'] = c
                L.add(c)
            while gold >= 10:
                opt = seq[n] if n < len(seq) else None
                front = set()
                for (x, y) in L:
                    for dx, dy in DIRS:
                        c = (x + dx, y + dy)
                        if c in vset and c not in L and c not in occ:
                            front.add(c)
                if not front:
                    break
                if opt:
                    d0 = dist[opt[0][2]]
                    c = min(front, key=lambda c: (d0.get(c, 99), -opened[c], c))
                    goals = list(opt)
                else:
                    c = min(front, key=lambda c: (abs(dO.get(c, 99) - dX.get(c, 99)), -opened[c], c))
                    goals = []
                L.add(c)
                occ.add(c)
                units.append(dict(pos=c, goals=goals))
                gold -= 10
                n += 1
                if log is not None:
                    log.append(dict(t=t, kind='train', to=list(c), goals=[[s[0], s[1]] for s in goals], gold=gold))
        return L, units, gold

    def judge(L, units):
        heads = [u['pos'] for u in units]
        free = [c for c in valid if c not in L and c not in set(X)]
        # Each free cell to the nearest head (over valid cells).
        owner = {}
        dd = {}
        q = deque()
        for i, h in enumerate(heads):
            dd[h] = 0
            owner[h] = i
            q.append(h)
        while q:
            x, y = q.popleft()
            for dx, dy in DIRS:
                c = (x + dx, y + dy)
                if c in vset and c not in dd and c not in set(X):
                    dd[c] = dd[(x, y)] + 1
                    owner[c] = owner[(x, y)]
                    q.append(c)
        room = [0] * len(heads)
        for c in free:
            if c in owner and dd[c] <= ROOM_CAP + 1:
                room[owner[c]] += 1
        room_total = sum(min(r, ROOM_CAP) for r in room)
        dl = bfs(rows, list(L))
        vor = sum(1 for c in dl if c not in dX or dl[c] < dX[c])
        void = sum(1 for c in L if opened[c] < 4)
        return len(L) + room_total, vor, -void, len(L), room

    t0 = time.time()
    results = []
    endings = set()
    for seq in itertools.product(range(len(options)), repeat=K):
        L, units, gold = simulate([options[i] for i in seq])
        key = judge(L, units)
        endings.add(frozenset(L) | frozenset(('u',) + u['pos'] for u in units))
        results.append((key[:3], seq, key))
    results.sort(key=lambda r: tuple(-x for x in r[0]))
    best = results[0]
    free_i = len(options) - 1
    freeres = next(r for r in results if all(i == free_i for i in r[1]))
    def named(i):
        o = options[i]
        if o is None:
            return 'free'
        return ' then '.join(f"{g['areas'][s[0]]['name'].split()[0]} {s[1]}" for s in o)
    print(g['game_id'], g['kind'], f"{len(results)} plans, {len(endings)} distinct endings, {time.time() - t0:.0f}s; best", best[2][:3], 'land', best[2][3], 'room', best[2][4], [named(i) for i in best[1]], '| free', freeres[2][:3])
    out = {}
    for key, res in (('plan', best), ('free', freeres)):
        log = []
        L, units, gold = simulate([options[i] for i in res[1]], log)
        out[key] = dict(score=res[2][0], voronoi=res[2][1], void=-res[2][2], land=[list(c) for c in sorted(L)], cells=res[2][3], room=res[2][4], heads=[list(u['pos']) for u in units], seq=[named(i) for i in res[1]], actions=log)
    g['plan3'] = out['plan']
    g['free3'] = out['free']
    g['endings'] = len(endings)
    g['plans3'] = len(results)
json.dump(G, open(P + '/plans3.json', 'w'), separators=(',', ':'))
