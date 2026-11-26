"""Turn-1 planner, version 4 (E080), for pictures only: no enemy.

The owner, 2026-11-26: do not lose much territory to the units' room, but
a little land at the end of the early game is worth giving up if more is
captured in the turns after; and the units should not plan independently:
it is a coordination problem.

Two modes of the same macro plans (version 3's options):
- independent (version 3): each unit keeps the sub-goal(s) of its
  training, walks to them, then grows freely toward the nearest free cell;
- coordinated: a training opens its sub-goal(s); every turn the open
  sub-goals not yet ours are matched to our units, the nearest pair first
  (each sub-goal to one unit, each unit to one sub-goal), so a sub-goal goes
  to whichever unit is nearest, however it was trained; a unit with no
  sub-goal grows into its own share (the free cells nearer to it than to our
  other units), and the units choose their targets in turn, each one's
  target reserved, so that two units do not chase the same cell. Steps:
  a new cell first, then nearer the other open sub-goals, then the most open
  cell.

Each plan is played H turns past T with free growth, to see what the
position captures in the turns after. Calibrations of the score:
- land: the most land at T;
- land + room: version 3 (room up to 3 a unit);
- later land: the most land at T + H;
- tolerance n: among the plans within n cells of the most land at T, the
  most land at T + H.
Then the Voronoi share at T, then the fewest cells along the void.
"""
import json, os, sys, itertools, time
from collections import deque
from multiprocessing import Pool

P = os.path.dirname(os.path.abspath(__file__))
S = os.path.dirname(P)
T = 9
H = 3
K = int(sys.argv[1]) if len(sys.argv) > 1 else 4
ROOM_CAP = 3
SRC = sys.argv[2] if len(sys.argv) > 2 else P + '/plans3.json'  # planner3.py's output, with the sub-goals
DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))
CALIBRATIONS = ['land', 'land + room', 'tolerance 1', 'tolerance 2', 'later land']


def bfs(rows, src, allow=None):
    d = {}
    q = deque()
    for c in src:
        d[c] = 0
        q.append(c)
    while q:
        x, y = q.popleft()
        for dx, dy in DIRS:
            a, b = x + dx, y + dy
            if 0 <= a < 12 and 0 <= b < 12 and rows[b][a] != '#' and (a, b) not in d and (allow is None or (a, b) in allow):
                d[(a, b)] = d[(x, y)] + 1
                q.append((a, b))
    return d


def owners(rows, heads, vset, blocked):
    """Each cell to the nearest head (multi-source BFS, ties to the first)."""
    own, dd = {}, {}
    q = deque()
    for i, h in enumerate(heads):
        if h not in dd:
            dd[h] = 0
            own[h] = i
            q.append(h)
    while q:
        x, y = q.popleft()
        for dx, dy in DIRS:
            c = (x + dx, y + dy)
            if c in vset and c not in dd and c not in blocked:
                dd[c] = dd[(x, y)] + 1
                own[c] = own[(x, y)]
                q.append(c)
    return own, dd


def plan_map(g):
    rows = g['boards'][0]['rows']
    O = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'O']
    X = [(x, y) for y in range(12) for x in range(12) if rows[y][x] == 'X']
    Xs = set(X)
    valid = [(x, y) for y in range(12) for x in range(12) if rows[y][x] != '#']
    vset = set(valid)
    opened = {c: sum(1 for dx, dy in DIRS if (c[0] + dx, c[1] + dy) in vset) for c in valid}
    dO, dX = bfs(rows, O), bfs(rows, X)
    subs = []
    for ai, a in enumerate(g['areas']):
        for kind in ('E', 'C', 'O'):
            subs.append((ai, kind, tuple(a['sub'][kind])))
    dist = {s[2]: bfs(rows, [s[2]]) for s in subs}
    singles, seen = [], set()
    for s in subs:
        if (s[0], s[2]) not in seen:
            seen.add((s[0], s[2]))
            singles.append([s])
    centres = [s for s in subs if s[1] == 'C']
    options = singles + [[a, b] for a, b in itertools.permutations(centres, 2)] + [None]

    def nxt_sub(ai, after):
        order = [s for s in subs if s[0] == ai]
        kinds = [s[1] for s in order]
        i = kinds.index(after) + 1 if after in kinds else 0
        return order[i] if i < len(order) else None

    def free_target(pos, L, occ, du, allowed):
        rest = [c for c in du if c not in L and c not in occ and c not in Xs and (allowed is None or c in allowed)]
        if not rest:
            return None
        return min(rest, key=lambda c: (du[c], abs(dO.get(c, 99) - dX.get(c, 99)), c))

    def step_to(pos, goal_ds, occ, L, extra=()):
        here = [d.get(pos, 99) for d in goal_ds]
        nb = [(pos[0] + dx, pos[1] + dy) for dx, dy in DIRS]
        nb = [c for c in nb if c in vset and c not in occ and c not in Xs]
        both = [c for c in nb if all(d.get(c, 99) < h for d, h in zip(goal_ds, here))]
        steps = both or [c for c in nb if goal_ds[0].get(c, 99) < here[0]]
        if not steps:
            return None, bool(both)
        near = lambda c: sum(d.get(c, 99) for d in extra)
        return min(steps, key=lambda c: (c in L, near(c), -opened[c], c)), bool(both)

    def simulate(seq, coord, log=None):
        L = set(O)
        units = []
        gold = 20
        n = 0
        opened_goals = []  # coordinated: the sub-goals the trainings opened
        snap = None
        for t in range(1, T + H + 1):
            gold += len(L) - len(units)
            if log is not None:
                log.append(dict(t=t, kind='turn', gold=gold, income=len(L) - len(units)))
            occ = {u['pos'] for u in units}
            if coord:
                # Open sub-goals: those opened and not ours; their area's
                # next one once reached.
                changed = True
                while changed:
                    changed = False
                    for i, s in enumerate(opened_goals):
                        if s is not None and s[2] in L:
                            opened_goals[i] = nxt_sub(s[0], s[1]) if s[1] != 'C' or True else None
                            changed = True
                open_now = [s for s in dict.fromkeys(opened_goals) if s is not None]
                # Matching, the nearest pair first.
                pairs = sorted(((dist[s[2]].get(u['pos'], 99), ui, si) for ui, u in enumerate(units) for si, s in enumerate(open_now)))
                gu, gs = {}, set()
                for d, ui, si in pairs:
                    if ui not in gu and si not in gs:
                        gu[ui] = si
                        gs.add(si)
                heads = [u['pos'] for u in units]
                own, _ = owners(rows, heads, vset, Xs | (L - set(heads)))
                reserved = set()
                for ui, u in enumerate(units):
                    pos = u['pos']
                    if ui in gu:
                        s = open_now[gu[ui]]
                        others = [dist[o[2]] for j, o in enumerate(open_now) if j != gu[ui] and j not in gs]
                        c, _ = step_to(pos, [dist[s[2]]], occ, L, others)
                        u['goals'] = [s]
                    else:
                        du = bfs(rows, [pos])
                        mine = {c for c, o in own.items() if o == ui}
                        goal = free_target(pos, L, occ | reserved, du, mine) or free_target(pos, L, occ | reserved, du, None)
                        u['goals'] = []
                        if goal is None:
                            continue
                        reserved.add(goal)
                        c, _ = step_to(pos, [bfs(rows, [goal])], occ, L)
                    if c is None:
                        continue
                    if log is not None:
                        log.append(dict(t=t, kind='move', frm=list(pos), to=list(c), goals=[[s[0], s[1]] for s in u['goals']], gold=gold))
                    occ.discard(pos)
                    occ.add(c)
                    u['pos'] = c
                    L.add(c)
            else:
                for u in units:
                    while u['goals'] and u['goals'][0][2] in L:
                        first, rest = u['goals'][0], u['goals'][1:]
                        nx = nxt_sub(first[0], first[1])
                        u['goals'] = rest if rest else ([nx] if nx else [])
                        if log is not None:
                            log.append(dict(t=t, kind='retarget', at=list(u['pos']), goals=[[s[0], s[1]] for s in u['goals']]))
                    pos = u['pos']
                    if u['goals']:
                        c, both = step_to(pos, [dist[s[2]] for s in u['goals']], occ, L)
                        if not both and len(u['goals']) > 1:
                            u['goals'] = u['goals'][:1]
                            if log is not None:
                                log.append(dict(t=t, kind='fork', at=list(pos), goals=[[s[0], s[1]] for s in u['goals']]))
                    else:
                        du = bfs(rows, [pos])
                        goal = free_target(pos, L, occ, du, None)
                        if goal is None:
                            continue
                        c, _ = step_to(pos, [bfs(rows, [goal])], occ, L)
                    if c is None:
                        continue
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
                        if c in vset and c not in L and c not in occ and c not in Xs:
                            front.add(c)
                if not front:
                    break
                if opt:
                    d0 = dist[opt[0][2]]
                    c = min(front, key=lambda c: (d0.get(c, 99), -opened[c], c))
                    goals = list(opt)
                    if coord:
                        opened_goals.extend(opt)
                else:
                    c = min(front, key=lambda c: (abs(dO.get(c, 99) - dX.get(c, 99)), -opened[c], c))
                    goals = []
                L.add(c)
                occ.add(c)
                units.append(dict(pos=c, goals=goals if not coord else []))
                gold -= 10
                n += 1
                if log is not None:
                    log.append(dict(t=t, kind='train', to=list(c), goals=[[s[0], s[1]] for s in goals], gold=gold))
            if t == T:
                snap = (frozenset(L), [u['pos'] for u in units])
        return snap, frozenset(L)

    def room_of(L, heads):
        own, dd = owners(rows, heads, vset, Xs | (L - set(heads)))
        room = [0] * len(heads)
        for c, o in own.items():
            if c not in L and dd[c] <= ROOM_CAP + 1:
                room[o] += 1
        return room

    def judge(snap, later):
        L, heads = snap
        room = room_of(L, heads)
        dl = bfs(rows, list(L))
        vor = sum(1 for c in dl if c not in dX or dl[c] < dX[c])
        void = sum(1 for c in L if opened[c] < 4)
        return dict(land=len(L), room=sum(min(r, ROOM_CAP) for r in room), later=len(later), vor=vor, void=void, rooms=room)

    def named(i):
        o = options[i]
        if o is None:
            return 'free'
        return ' and '.join(f"{g['areas'][s[0]]['name'].split()[0]} {s[1]}" for s in o)

    out = {}
    report = []
    for coord in (False, True):
        mode = 'coordinated' if coord else 'independent'
        t0 = time.time()
        res = []
        endings = set()
        for seq in itertools.product(range(len(options)), repeat=K):
            snap, later = simulate([options[i] for i in seq], coord)
            j = judge(snap, later)
            endings.add(snap[0] | frozenset(('u',) + h for h in snap[1]))
            res.append((seq, j))
        top = max(j['land'] for _, j in res)
        tie = lambda j: (j['vor'], -j['void'])
        keys = {
            'land': lambda j: (j['land'],) + tie(j),
            'land + room': lambda j: (j['land'] + j['room'],) + tie(j),
            'tolerance 1': lambda j: (j['land'] >= top - 1, j['later'], j['land']) + tie(j),
            'tolerance 2': lambda j: (j['land'] >= top - 2, j['later'], j['land']) + tie(j),
            'later land': lambda j: (j['later'], j['land']) + tie(j),
        }
        free_i = len(options) - 1
        picks = dict((name, max(res, key=lambda r: keys[name](r[1]))) for name in CALIBRATIONS)
        picks['free growth'] = next(r for r in res if all(i == free_i for i in r[0]))
        out[mode] = {}
        for name, (seq, j) in picks.items():
            log = []
            snap, later = simulate([options[i] for i in seq], coord, log)
            out[mode][name] = dict(seq=[named(i) for i in seq], cells=[list(c) for c in sorted(snap[0])], gained=[list(c) for c in sorted(later - snap[0])], heads=[list(h) for h in snap[1]], actions=log, **{k: v for k, v in j.items()})
            report.append(f"{g['game_id']} {mode:11} {name:12} land {j['land']:2} room {j['room']:2} T+{H} {j['later']:2} vor {j['vor']:3} void {j['void']:2} | {', '.join(named(i) for i in seq)}")
        out[mode]['endings'] = len(endings)
        out[mode]['plans'] = len(res)
        report.append(f"{g['game_id']} {mode} {len(res)} plans, {len(endings)} endings, {time.time() - t0:.0f}s")
    return g['game_id'], out, report


if __name__ == '__main__':
    G = json.load(open(SRC))
    with Pool(4) as pool:
        results = pool.map(plan_map, G)
    for g, (gid, out, report) in zip(G, results):
        g['plan4'] = out
        print('\n'.join(report), flush=True)
    json.dump(G, open(P + '/plans4.json', 'w'), separators=(',', ':'))
