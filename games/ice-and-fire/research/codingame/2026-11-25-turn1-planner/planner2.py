"""Turn-1 planner, version 2 (E080), for pictures only: no enemy.

Two levels of decision (the owner, 2026-11-26):
- macro: which area each training serves; every sequence of areas for the
  first K trainings is tried (an area per training, or none: free growth);
- micro: a unit walks the shortest path to its area's current target
  (the area's entrance, then its front centre, then the area's nearest cell
  not ours), taking a new cell when it can; ties by cell order (to be
  arbitrated later).

Areas: the fronts of the map and the region each decides (as before). An
area's entrance: where our shortest path from the HQ to the front centre
enters the area for good (the front centre itself when the whole path is
inside the area); the front centre: the front's contested cell
nearest its centroid, for the middle the most central (x - y nearest 0).
Objective: the most cells at game turn T+1, then the Voronoi share against
the enemy's starting land.
"""
import json, os, sys, itertools, time
from collections import deque

P = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(P)
T = 9
K = int(sys.argv[1]) if len(sys.argv) > 1 else 7
G = json.load(open(S + '/page84/goals.json'))
DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))


def bfs(rows, src, block=frozenset()):
    d = {}
    q = deque()
    for c in src:
        d[c] = 0
        q.append(c)
    while q:
        x, y = q.popleft()
        for dx, dy in DIRS:
            a, b = x + dx, y + dy
            if 0 <= a < 12 and 0 <= b < 12 and rows[b][a] != '#' and (a, b) not in d and (a, b) not in block:
                d[(a, b)] = d[(x, y)] + 1
                q.append((a, b))
    return d


def path(rows, start, goal, block=frozenset()):
    """A shortest path from `start` (a set) to `goal`, first step first."""
    d = bfs(rows, [goal], block)
    s = min((c for c in start if c in d), key=lambda c: (d[c], c), default=None)
    if s is None:
        return []
    out = [s]
    while d[out[-1]] > 0:
        x, y = out[-1]
        # Among equally short paths, the most central (|x - y| smallest).
        nxt = min(((x + dx, y + dy) for dx, dy in DIRS if (x + dx, y + dy) in d and d[(x + dx, y + dy)] == d[out[-1]] - 1), key=lambda c: (abs(c[0] - c[1]), c))
        out.append(nxt)
    return out


def nbr(L, rows):
    out = set()
    for (x, y) in L:
        for dx, dy in DIRS:
            a, b = x + dx, y + dy
            if 0 <= a < 12 and 0 <= b < 12 and rows[b][a] != '#' and (a, b) not in L:
                out.add((a, b))
    return out


def simulate(rows, O, areas, seq, log=None):
    """Our land after T turns; `seq[i]` is the area of the i-th training
    (None: free growth; beyond the sequence, free growth)."""
    L = set(O)
    units = []  # [pos, area]
    g = 20
    n_train = 0
    for t in range(1, T + 1):
        g += len(L) - len(units)
        if log is not None:
            log.append(dict(t=t, kind='turn', gold=g, income=len(L) - len(units)))
        occupied = {tuple(u[0]) for u in units}
        # Micro: each unit one step toward its area's current target.
        for un in units:
            pos, a = tuple(un[0]), un[1]
            goal = target(rows, L, areas, a)
            if goal is None:
                # Free: the nearest cell not ours, toward the middle line on
                # a tie.
                du = bfs(rows, [pos])
                rest = [c for c in du if c not in L and c not in occupied]
                if not rest:
                    continue
                goal = min(rest, key=lambda c: (du[c], free_key(c), c))
            d = bfs(rows, [goal])
            here = d.get(pos, 99)
            steps = [(pos[0] + dx, pos[1] + dy) for dx, dy in DIRS]
            steps = [c for c in steps if c in d and d[c] < here and c not in occupied]
            if not steps:
                continue
            nxt = min(steps, key=lambda c: (c in L, c))
            if log is not None:
                log.append(dict(t=t, kind='move', frm=list(pos), to=list(nxt), target=a, gold=g))
            occupied.discard(pos)
            occupied.add(nxt)
            un[0] = nxt
            L.add(nxt)
        # Macro: each training to the area of the sequence, at the tip.
        while g >= 10:
            a = seq[n_train] if n_train < len(seq) else None
            goal = target(rows, L, areas, a)
            cand = [c for c in nbr(L, rows) if c not in occupied]
            if not cand:
                break
            if goal is not None:
                d = bfs(rows, [goal])
                c = min(cand, key=lambda c: (d.get(c, 99), c))
            else:
                c = min(cand, key=lambda c: (free_key(c), c))
            L.add(c)
            occupied.add(c)
            units.append([c, a])
            g -= 10
            n_train += 1
            if log is not None:
                log.append(dict(t=t, kind='train', to=list(c), target=a, gold=g))
    return L


def target(rows, L, areas, a):
    """The current target of area `a`: its entrance, then its front
    centre, then its nearest cell not ours; None for free growth."""
    if a is None:
        return None
    ar = areas[a]
    for c in (ar['entrance'], ar['centre']):
        if c not in L:
            return c
    rest = [c for c in ar['cells'] if c not in L]
    if not rest:
        return None
    d = bfs(rows, list(L))
    return min(rest, key=lambda c: (d.get(c, 99), c))


out = []
for g in G:
    rows = g['boards'][0]['rows']
    O = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'O']
    X = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'X']
    dO, dX = bfs(rows, O), bfs(rows, X)
    def free_key(c, dO=dO, dX=dX):
        return abs(dO.get(c, 99) - dX.get(c, 99))
    areas = []
    for f in g['fronts']:
        line = [tuple(c) for c in f['cells']]
        cells = [tuple(map(int, k.split(','))) for k, l in g['region'].items() if l == f['lane']]
        if f['lane'] == 1:
            centre = min(line, key=lambda c: (abs(c[0] - c[1]), dO.get(c, 99), c))
        else:
            mx = sum(c[0] for c in line) / len(line)
            my = sum(c[1] for c in line) / len(line)
            centre = min(line, key=lambda c: ((c[0] - mx) ** 2 + (c[1] - my) ** 2, dO.get(c, 99), c))
        p = path(rows, set(O), centre)
        region = set(cells)
        # Where the path enters the area for good: the cell after the last
        # one outside it; the front centre when the whole path is inside.
        outside = [i for i, c in enumerate(p) if c not in region]
        entrance = p[outside[-1] + 1] if outside and outside[-1] + 1 < len(p) else centre
        areas.append(dict(lane=f['lane'], name=f['name'], centre=centre, entrance=entrance, cells=cells))
    t0 = time.time()
    plans = []
    choices = list(range(len(areas))) + [None]
    for seq in itertools.product(choices, repeat=K):
        L = simulate(rows, O, areas, list(seq))
        dl, dx = bfs(rows, list(L)), bfs(rows, X)
        vor = sum(1 for c in dl if c not in dx or dl[c] < dx[c])
        reached = [a for a, ar in enumerate(areas) if ar['centre'] in L]
        plans.append((len(L), vor, seq, sorted(L), reached))
    plans.sort(key=lambda p: (-p[0], -p[1], p[2].count(None)))
    best = plans[0]
    free = next(p for p in plans if all(s is None for s in p[2]))
    log, flog = [], []
    simulate(rows, O, areas, list(best[2]), log)
    simulate(rows, O, areas, list(free[2]), flog)
    names = {None: 'free'}
    names.update({i: a['name'] for i, a in enumerate(areas)})
    print(g['game_id'], g['kind'], f'{len(plans)} plans {time.time() - t0:.0f}s', 'best', best[:2], [names[s] for s in best[2]], 'centres reached', [areas[a]['name'] for a in best[4]], '| free', free[:2])
    g['areas'] = [dict(lane=a['lane'], name=a['name'], centre=list(a['centre']), entrance=list(a['entrance'])) for a in areas]
    g['plan'] = dict(cells=best[0], voronoi=best[1], seq=[None if s is None else areas[s]['lane'] for s in best[2]], land=[list(c) for c in best[3]], reached=[areas[a]['lane'] for a in best[4]], actions=[dict(a, target=None if a.get('target') is None else areas[a['target']]['lane']) for a in log], plans=len(plans))
    g['free'] = dict(cells=free[0], voronoi=free[1], land=[list(c) for c in free[3]], actions=[dict(a, target=None) for a in flog])
    g['top'] = [dict(cells=p[0], voronoi=p[1], seq=[None if s is None else areas[s]['lane'] for s in p[2]]) for p in plans[:5]]
    out.append(g)
json.dump(out, open(P + '/plans2.json', 'w'), separators=(',', ':'))
