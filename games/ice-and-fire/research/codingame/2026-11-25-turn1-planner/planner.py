"""Prototype of the turn-1 planner (E080), for pictures only.

Red at game turn 1 on each of the six maps. A plan is an ordered list of
target cells (each front's entrances on our side and its deep cell) and a
mode for the trainings ('sequential': the first unfinished target gets
them all; 'spread': the target with the least progress). Each plan is
simulated for our turns 1..T with simple rules; the enemy is model (1): it
takes a cell at the earliest turn a pure rush with all its gold could reach
it (its reach after t turns, from its starting land).
"""
import json, os, sys, itertools, time
from collections import deque

P = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(P)
T = int(sys.argv[1]) if len(sys.argv) > 1 else 9
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


def rush_reach(turns, cells0, red):
    """Reach (steps from the starting land) of a pure rush after each turn."""
    r, n, u, g = 0, cells0, 0, 20
    out = []
    for t in range(1, turns + 1):
        g += n - u
        # Every unit takes a new cell; the tip unit advances the front.
        if u > 0:
            r += 1
            n += u
        k = g // 10
        r += k
        n += k
        u += k
        g -= 10 * k
        out.append(r)
    return out


def simulate(rows, ours0, enemy_at, targets, mode, free_bias):
    """Our land after T turns playing `targets` in order."""
    L = set(ours0)
    units = []  # [pos, target index or None, moved_this_turn]
    g = 20
    done = [False] * len(targets)
    progress = [0] * len(targets)
    occupied = set()
    for t in range(1, T + 1):
        g += len(L) - len(units)
        blocked = {c for c, e in enemy_at.items() if e < t}  # the enemy's by its turn t-1
        def ok(c):
            return c not in blocked and enemy_at.get(c, 99) >= t
        # Moves.
        for un in units:
            pos, ti = un[0], un[1]
            if ti is not None and done[ti]:
                ti = un[1] = next((i for i in range(len(targets)) if not done[i]), None)
            if ti is not None:
                d = bfs(rows, [targets[ti]], frozenset(blocked))
            else:
                free = [c for c in d_all if c not in L and ok(c)]
                if not free:
                    continue
                d = bfs(rows, free, frozenset(blocked))
            here = d.get(tuple(pos), 99)
            best = None
            for dx, dy in DIRS:
                c = (pos[0] + dx, pos[1] + dy)
                if c in d and d[c] < here and c not in occupied and ok(c) or (c in d and d[c] < here and c in L and c not in occupied):
                    if best is None or (c not in L, -d[c]) > (best not in L, -d[best]):
                        best = c
            if best is not None:
                occupied.discard(tuple(pos))
                un[0] = best
                occupied.add(best)
                L.add(best)
        for i, tc in enumerate(targets):
            if tc in L:
                done[i] = True
        # Trainings, chained at the tip.
        while g >= 10:
            open_ = [i for i in range(len(targets)) if not done[i]]
            if open_:
                ti = open_[0] if mode == 'sequential' else min(open_, key=lambda i: (progress[i], i))
                d = bfs(rows, [targets[ti]], frozenset(blocked))
                cand = [c for c in nbr(L, rows) if c in d and ok(c) and c not in occupied]
                if not cand:
                    done[ti] = True
                    continue
                c = min(cand, key=lambda c: (d[c], c))
            else:
                ti = None
                cand = [c for c in nbr(L, rows) if ok(c) and c not in occupied]
                if not cand:
                    break
                c = min(cand, key=lambda c: (free_bias.get(c, 99), c))
            L.add(c)
            occupied.add(c)
            units.append([c, ti])
            g -= 10
            if ti is not None:
                progress[ti] += 1
                if c == targets[ti]:
                    done[ti] = True
    return L, done


def nbr(L, rows):
    out = set()
    for (x, y) in L:
        for dx, dy in DIRS:
            a, b = x + dx, y + dy
            if 0 <= a < 12 and 0 <= b < 12 and rows[b][a] != '#' and (a, b) not in L:
                out.add((a, b))
    return out


def score(rows, L, enemy0, reach_T):
    # The enemy's land at T: what its rush reaches without crossing ours.
    d = bfs(rows, enemy0, frozenset(L))
    B = {c for c, v in d.items() if v <= reach_T}
    dl, db = bfs(rows, list(L)), bfs(rows, list(B))
    vor = sum(1 for c in dl if c not in db or dl[c] < db[c])
    return len(L), vor, len(B)


out = []
for g in G:
    rows = g['boards'][0]['rows']
    O = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'O']
    X = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'X']
    d_all = bfs(rows, O)
    dX = bfs(rows, X)
    reach = rush_reach(T, len(X), red=False)
    enemy_at = {}
    for c, dist in dX.items():
        e = next((t for t, r in enumerate(reach, 1) if r >= dist), 99)
        enemy_at[c] = e
    for c in X:
        enemy_at[c] = 0
    # Free growth toward the middle line: by |d(us) - d(them)|.
    free_bias = {c: abs(d_all.get(c, 99) - dX.get(c, 99)) for c in d_all}
    targets = []
    for f in g['fronts']:
        for c in f.get('entrances', []):
            targets.append((tuple(c), f['lane'], 'entrance'))
        if f.get('deep'):
            targets.append((tuple(f['deep']['cell']), f['lane'], 'deep'))
    t0 = time.time()
    plans = []
    cells = [t[0] for t in targets]
    for k in range(0, 5):
        for order in itertools.permutations(range(len(targets)), k):
            for mode in (('sequential', 'spread') if k > 1 else ('sequential',)):
                L, done = simulate(rows, O, enemy_at, [cells[i] for i in order], mode, free_bias)
                n, vor, nb = score(rows, L, X, reach[-1])
                plans.append((vor + n, n, vor, nb, order, mode, sorted(L), done))
    plans.sort(key=lambda p: (-p[0], -p[1]))
    best = plans[0]
    free = next(p for p in plans if not p[4])
    print(g['game_id'], g['kind'], f'{len(plans)} plans in {time.time() - t0:.1f}s', 'best', best[:4], [targets[i][1:] + (targets[i][0],) for i in best[4]], best[5], '| free growth', free[:4])
    g['plan'] = dict(score=best[0], cells=best[1], voronoi=best[2], enemy=best[3], order=[dict(cell=list(targets[i][0]), lane=targets[i][1], kind=targets[i][2], won=best[7][j]) for j, i in enumerate(best[4])], mode=best[5], land=[list(c) for c in best[6]], plans=len(plans))
    g['free'] = dict(score=free[0], cells=free[1], voronoi=free[2], enemy=free[3], land=[list(c) for c in free[6]])
    g['top'] = [dict(score=p[0], cells=p[1], voronoi=p[2], order=[dict(cell=list(targets[i][0]), lane=targets[i][1], kind=targets[i][2]) for i in p[4]], mode=p[5]) for p in plans[:5]]
    g['reach'] = reach
json.dump(G, open(P + '/plans.json', 'w'), separators=(',', ':'))
