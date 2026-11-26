"""E080 turn-1 planner as a bot (prototype, Python): plays against a real
opponent through the arena, until game turn 12.

Turns 1 to 3: the turn-1 plan (planner5's coordinated plan kept by
tolerance 1: each training opens its sub-goals, matched each turn to the
nearest unit). From turn 4, every turn, the plan is re-made against the
opponent's real position:
- an enemy model (env PLAN_MODEL: free, momentum, mirror, rush) guesses
  the enemy's land for the next turns, hence when it reaches each cell;
- each area's sub-goals move with it: E fixed; C the furthest cell from E
  to the static centre we still reach first; O the furthest cell beyond
  the centre toward the enemy's HQ we reach at least a turn first (deeper
  where the enemy does not come);
- options (area, sub-goal) are weighed by the land each moves to our side
  (Voronoi against the guessed enemy land) times a race factor (0 if the
  enemy arrives first, 0.6 on a tie, 1 with a turn to spare), and picked
  one at a time for the nearest unit or a training;
- roles: a unit the enemy can reach next turn fights (our turn and the
  enemy's best one-turn reply, all fighting units jointly); a unit with a
  sub-goal advances; every other unit is backline and fills its share of
  our territory (the free cells nearer our land than the guessed enemy).
Our HQ is always at (0, 0) inside: a blue bot turns the board around.
Logs one JSON line a turn to env PLAN_LOG.
"""
import itertools, json, os, sys
from collections import deque

DIRS = ((1, 0), (-1, 0), (0, 1), (0, -1))
MODEL = os.environ.get('PLAN_MODEL', 'free')
LOG = open(os.environ['PLAN_LOG'], 'w') if os.environ.get('PLAN_LOG') else None
AREAS = json.load(open(os.environ['PLAN_AREAS']))  # areas with static sub-goals (our HQ at (0, 0))
SEQ = json.loads(os.environ['PLAN_SEQ'])  # the turn-1 plan: per training, a list of [area, kind]
LAST_TURN = 12
BUDGET = float(os.environ.get('PLAN_BUDGET', '20'))  # seconds of climbing a turn
HOLD = 3  # a unit with no sub-goal this near the enemy's land holds the front
FACTOR = {-1: 0.0, 0: 0.6, 1: 1.0}


def rd():
    return input().strip()


def nbrs(c):
    for dx, dy in DIRS:
        a, b = c[0] + dx, c[1] + dy
        if 0 <= a < 12 and 0 <= b < 12:
            yield (a, b)


class State:
    pass


def read_state(lines, flip):
    rd = lambda: next(lines)
    s = State()
    s.gold, s.income, s.egold, s.eincome = (int(rd()) for _ in range(4))
    raw = [rd() for _ in range(12)]
    f = (lambda x, y: (11 - x, 11 - y)) if flip else (lambda x, y: (x, y))
    s.rows = [[None] * 12 for _ in range(12)]
    for y in range(12):
        for x in range(12):
            a, b = f(x, y)
            s.rows[b][a] = raw[y][x]
    s.valid = {(x, y) for y in range(12) for x in range(12) if s.rows[y][x] != '#'}
    s.ours = {(x, y) for (x, y) in s.valid if s.rows[y][x] in 'Oo'}
    s.theirs = {(x, y) for (x, y) in s.valid if s.rows[y][x] in 'Xx'}
    s.buildings = {}
    for _ in range(int(rd())):
        o, t, x, y = map(int, rd().split())
        s.buildings[f(x, y)] = (o, t)
    s.units = {}  # pos -> (owner, id, level)
    for _ in range(int(rd())):
        o, i, l, x, y = map(int, rd().split())
        s.units[f(x, y)] = (o, i, l)
    return s


def bfs(valid, src, allow=None):
    d = {c: 0 for c in src}
    q = deque(src)
    while q:
        c = q.popleft()
        for n in nbrs(c):
            if n in valid and n not in d and (allow is None or n in allow):
                d[n] = d[c] + 1
                q.append(n)
    return d


def active(s, side):
    land = s.ours if side == 0 else s.theirs
    hq = (0, 0) if side == 0 else (11, 11)
    return set(bfs(s.valid, [hq], land)) if hq in land else set()


def protected(s, c, side):
    """Whether `side`'s cell c is protected by a tower of `side`."""
    act = active(s, side)
    if s.buildings.get(c, (None, None)) == (side, 2):
        return True
    return c in act and any(n in act and s.buildings.get(n) == (side, 2) for n in nbrs(c))


def enterable(s, c, side, level=1):
    if c not in s.valid:
        return False
    mine = s.ours if side == 0 else s.theirs
    if c in mine:
        return c not in s.units and c not in s.buildings
    if level < 3 and protected(s, c, 1 - side):
        return False
    u = s.units.get(c)
    if u is not None and not (level == 3 or level > u[2]):
        return False
    return True


def moved_ids(cmds):
    return {c[1] for c in cmds if c[0] == 'MOVE'}


def moves_first(cmds):
    return [c for c in cmds if c[0] == 'MOVE'] + [c for c in cmds if c[0] != 'MOVE']


def clone(s):
    x = State()
    x.__dict__.update(s.__dict__)
    x.ours, x.theirs = set(s.ours), set(s.theirs)
    x.buildings, x.units = dict(s.buildings), dict(s.units)
    return x


def take(s, c, side):
    (s.ours if side == 0 else s.theirs).add(c)
    (s.theirs if side == 0 else s.ours).discard(c)
    if c in s.buildings and s.buildings[c][0] != side:
        del s.buildings[c]


def central_path(valid, start, goal):
    d = bfs(valid, [goal])
    if start not in d:
        return [start]
    out = [start]
    while d[out[-1]] > 0:
        out.append(min((n for n in nbrs(out[-1]) if d.get(n, 99) == d[out[-1]] - 1), key=lambda c: (abs(c[0] - c[1]), c)))
    return out


# --- Enemy models: the enemy's land after each of its next H turns. ---

def guess(s, H, prev_units, plan_cells):
    E = set(s.theirs)
    out = [frozenset(E)]
    blocked = set(s.ours)
    dO = bfs(s.valid, [(0, 0)])
    dX = bfs(s.valid, [(11, 11)])
    if MODEL == 'rush':
        g, n, u = s.egold, len(active(s, 1)), sum(1 for v in s.units.values() if v[0] == 1)
        r = 0
        for _ in range(H):
            g += n - u
            if u:
                r += 1
                n += u
            k = g // 10
            r += k
            n += k
            u += k
            g -= 10 * k
            d = bfs(s.valid, list(s.theirs), s.valid - blocked)
            out.append(frozenset(c for c, v in d.items() if v <= r))
        return out
    units = [(c, v[1]) for c, v in s.units.items() if v[0] == 1]
    g = s.egold
    goals = []
    if MODEL == 'mirror':
        goals = [(11 - c[0], 11 - c[1]) for c in plan_cells]
    for _ in range(H):
        g += len(E) - len(units)
        occ = {c for c, _ in units}
        reserved = set()
        new_units = []
        open_goals = [c for c in goals if c not in E]
        taken_goals = set()
        for pos, uid in sorted(units, key=lambda t: dO.get(t[0], 99)):
            target = None
            if MODEL == 'momentum' and uid in prev_units:
                p = prev_units[uid]
                step = (pos[0] + (pos[0] - p[0]), pos[1] + (pos[1] - p[1]))
                if step in s.valid and step not in E and step not in blocked and step not in occ and step != pos:
                    target = step
            if target is None and open_goals:
                cand = [c for c in open_goals if c not in taken_goals]
                if cand:
                    dg = {c: bfs(s.valid, [c]).get(pos, 99) for c in cand}
                    best = min(cand, key=lambda c: dg[c])
                    taken_goals.add(best)
                    dgb = bfs(s.valid, [best])
                    steps = [n for n in nbrs(pos) if n in s.valid and n not in blocked and n not in occ and dgb.get(n, 99) < dgb.get(pos, 99)]
                    if steps:
                        target = min(steps, key=lambda n: (n in E, n))
            if target is None:
                du = bfs(s.valid, [pos], s.valid - blocked)
                free = [c for c in du if c not in E and c not in occ and c not in reserved]
                if free:
                    goal = min(free, key=lambda c: (du[c], abs(dO.get(c, 99) - dX.get(c, 99)), c))
                    reserved.add(goal)
                    dg = bfs(s.valid, [goal], s.valid - blocked)
                    steps = [n for n in nbrs(pos) if n in dg and n not in occ and dg[n] < dg.get(pos, 99)]
                    if steps:
                        target = min(steps, key=lambda n: (n in E, n))
            if target is not None:
                occ.discard(pos)
                occ.add(target)
                E.add(target)
                new_units.append((target, uid))
            else:
                new_units.append((pos, uid))
        units = new_units
        while g >= 10:
            front = {n for c in E for n in nbrs(c) if n in s.valid and n not in E and n not in blocked and n not in occ}
            if not front:
                break
            if MODEL == 'momentum':
                c = max(front, key=lambda c: (dX.get(c, 0), c))
            elif MODEL == 'mirror' and [x for x in open_goals if x not in E]:
                tgt = [x for x in open_goals if x not in E][0]
                dt = bfs(s.valid, [tgt])
                c = min(front, key=lambda c: (dt.get(c, 99), c))
            else:
                c = min(front, key=lambda c: (abs(dO.get(c, 99) - dX.get(c, 99)), c))
            E.add(c)
            occ.add(c)
            units.append((c, -1))
            g -= 10
        out.append(frozenset(E))
    return out


def arrival(guesses):
    a = {}
    for k, E in enumerate(guesses):
        for c in E:
            a.setdefault(c, k)
    return a


# --- The turn. ---

class Bot:
    def __init__(self):
        rd_n = int(rd())
        self.mine_spots_raw = [tuple(map(int, rd().split())) for _ in range(rd_n)]
        self.t = 0
        self.flip = None
        self.trained = 0
        self.opened = []  # the turn-1 plan's open sub-goals: [area, kind]
        self.prev_enemy = {}
        self.plan_cells = [tuple(a['sub'][k]) for a in AREAS for k in ('E', 'C', 'O')]

    def static_sub(self, a, kind):
        return tuple(AREAS[a]['sub'][kind])

    def run(self):
        while True:
            lines = [rd() for _ in range(16)]
            nb = int(rd())
            lines += [str(nb)] + [rd() for _ in range(nb)]
            nu = int(rd())
            lines += [str(nu)] + [rd() for _ in range(nu)]
            if self.flip is None:
                self.flip = lines[4][0] not in 'Oo'
                self.mine_spots = {(11 - x, 11 - y) if self.flip else (x, y) for x, y in self.mine_spots_raw}
            s = read_state(iter(lines), self.flip)
            self.t += 1
            cmds, log = self.turn(s)
            out = []
            for c in cmds:
                if c[0] == 'MOVE':
                    x, y = self.out_xy(c[2])
                    out.append(f'MOVE {c[1]} {x} {y}')
                elif c[0] == 'BUILD':
                    x, y = self.out_xy(c[1])
                    out.append(f'BUILD TOWER {x} {y}')
                elif c[0] == 'TRAIN2':
                    x, y = self.out_xy(c[1])
                    out.append(f'TRAIN 2 {x} {y}')
                else:
                    x, y = self.out_xy(c[1])
                    out.append(f'TRAIN 1 {x} {y}')
            print(';'.join(out) if out else 'WAIT', flush=True)
            if LOG:
                LOG.write(json.dumps(log) + '\n')
                LOG.flush()

    def out_xy(self, c):
        return (11 - c[0], 11 - c[1]) if self.flip else c

    def turn(self, s):
        t = self.t
        log = dict(t=t, seat=1 if self.flip else 0, model=MODEL, gold=s.gold, egold=s.egold,
                   rows=[''.join(r) for r in s.rows],
                   units=[[c[0], c[1], v[0], v[1], v[2]] for c, v in s.units.items()],
                   buildings=[[c[0], c[1], v[0], v[1]] for c, v in s.buildings.items()])
        if t == 1:
            log['plan'] = self.predict_plan(s)
        s0 = clone(s)
        H = 4
        prev = {v[1]: c for c, v in self.prev_enemy.items()}
        guesses = guess(s, H, prev, self.plan_cells) if t >= 4 else [frozenset(s.theirs)]
        self.prev_enemy = {c: v for c, v in s.units.items() if v[0] == 1}
        arr = arrival(guesses)
        EH = guesses[-1]
        if t >= 4:
            # Beyond the guess, the enemy goes on a step a turn.
            dEH = bfs(s.valid, list(EH), s.valid - s.ours)
            for c, v in dEH.items():
                arr.setdefault(c, len(guesses) - 1 + v)
        log['guess'] = sorted(map(list, EH - s.theirs))
        my_units = {c: v for c, v in s.units.items() if v[0] == 0}
        e_act = active(s, 1)
        e_reach = {n for c in e_act for n in nbrs(c) if n in s.valid} | {c for c, v in s.units.items() if v[0] == 1}
        fighting = [c for c in my_units if any(n in s.theirs or n in e_reach for n in nbrs(c) if n in s.valid)]
        free_units = [c for c in my_units if c not in fighting]
        roles = {}
        cmds = []
        # 1. Fights: our turn and the enemy's best one-turn reply.
        if fighting:
            self.fight(s, fighting, cmds, roles)
        # 2. Towers against the enemy's guessed reply, weighed with the
        # options the same gold would pay for.
        if t >= 4:
            self.towers(s, t, free_units, arr, EH, cmds, log)
        # 3. Advancing: options picked for units and trainings.
        n_train = s.gold // 10
        picks = self.options(s, t, free_units, n_train, arr, EH, log)
        dL = None
        for agent, cell, label, value in picks:
            if agent[0] == 'unit':
                pos = agent[1]
                roles[my_units[pos][1]] = dict(role='advance', goal=list(cell), label=label)
                self.step(s, pos, cell, cmds)
        # 3. Backline: each its share of our territory.
        terr = self.territory(s, EH)
        log['territory'] = sorted(map(list, terr))
        rest = [c for c in free_units if not any(p[0] == ('unit', c) for p in picks)]
        rest = [c for c in rest if c in s.units]  # still there
        # The front line: the free cells about as near our land as the
        # enemy's (within a step); a unit near it holds and pushes it.
        dLo, dTh = bfs(s.valid, list(s.ours)), bfs(s.valid, list(s.theirs))
        line = [c for c in s.valid if c not in s.ours and c not in s.theirs and abs(dLo.get(c, 99) - dTh.get(c, 99)) <= 1]
        dEn = bfs(s.valid, line) if line else bfs(s.valid, list(s.theirs))
        hold = [c for c in rest if dEn.get(c, 99) <= HOLD]
        back = [c for c in rest if c not in hold]
        self.hold(s, hold, dEn, cmds, roles)
        self.backline(s, back, terr, cmds, roles)
        # 4. Trainings: the options' first, then for our territory.
        for agent, cell, label, value in picks:
            if agent[0] == 'train' and s.gold >= 10:
                c = self.train_toward(s, cell)
                if c:
                    cmds.append(('TRAIN', c))
                    roles['new %d,%d' % c] = dict(role='advance', goal=list(cell), label=label)
        while s.gold >= 10:
            cover = sum(1 for v in s.units.values() if v[0] == 0)
            if len(terr - s.ours) <= 2 * cover and t >= 4:
                break
            front = [c for c in self.trainable(s) if (c in terr or t < 4) and c not in s.ours]
            if not front:
                break
            dO, dX = bfs(s.valid, [(0, 0)]), bfs(s.valid, [(11, 11)])
            c = min(front, key=lambda c: (abs(dO.get(c, 99) - dX.get(c, 99)), c))
            if not self.do_train(s, c):
                break
            cmds.append(('TRAIN', c))
            roles['new %d,%d' % c] = dict(role='backline')
        if t >= 4:
            cmds = self.climb(s0, cmds, log, roles)
            s = self.apply(s0, cmds)[0]
        log['roles'] = roles
        log['cmds'] = [[c[0]] + ([c[1], list(c[2])] if c[0] == 'MOVE' else [list(c[1])]) for c in cmds]
        log['after_gold'] = s.gold
        log['after_buildings'] = [[c[0], c[1], v[0], v[1]] for c, v in s.buildings.items()]
        log['fighting'] = [list(c) for c in fighting]
        log['after'] = [''.join('#' if (x, y) not in s.valid else 'O' if (x, y) in s.ours else 'X' if (x, y) in s.theirs else '.' for x in range(12)) for y in range(12)]
        log['after_units'] = [[c[0], c[1], v[0], v[1], v[2]] for c, v in s.units.items()]
        return cmds, log

    # The whole turn against the enemy's reply.
    def apply(self, s0, cmds):
        """Our commands played on a copy of `s0`, each checked by the rules;
        the position and the commands the rules accepted."""
        x = clone(s0)
        ok = []
        for c in cmds:
            if c[0] == 'MOVE':
                pos = next((p for p, v in x.units.items() if v[0] == 0 and v[1] == c[1]), None)
                if pos is None or c[2] not in set(nbrs(pos)) or c[1] in moved_ids(ok):
                    continue
                n = len(x.units)
                if self.move(x, pos, c[2], []):
                    ok.append(c)
            elif c[0] in ('TRAIN', 'TRAIN2'):
                level, cost = (1, 10) if c[0] == 'TRAIN' else (2, 20)
                cell = c[1]
                if x.gold < cost or cell not in self.trainable_any(x) or not enterable(x, cell, 0, level):
                    continue
                x.units.pop(cell, None)
                take(x, cell, 0)
                x.units[cell] = (0, -1, level)
                x.gold -= cost
                ok.append(c)
            elif c[0] == 'BUILD':
                cell = c[1]
                if x.gold < 15 or cell not in active(x, 0) or cell in x.units or cell in x.buildings or cell in self.mine_spots:
                    continue
                x.buildings[cell] = (0, 2)
                x.gold -= 15
                ok.append(c)
        return x, ok

    def trainable_any(self, s):
        act = active(s, 0)
        return set(act) | {n for c in act for n in nbrs(c) if n in s.valid}

    def evaluate(self, s0, cmds):
        x, ok = self.apply(s0, cmds)
        lost, deadly, _ = self.reply(x)
        y = self.reply_state
        ours, theirs = set(active(y, 0)), set(active(y, 1))
        dL, dE = bfs(y.valid, list(ours)), bfs(y.valid, list(theirs))
        vor = sum(1 for c in y.valid if dL.get(c, 99) < dE.get(c, 99)) - sum(1 for c in y.valid if dE.get(c, 99) < dL.get(c, 99))
        # The enemy's next turn from there, our turn between not counted.
        y2 = clone(y)
        y2.egold, y2.eincome = max(0, self.reply_next_gold), 0
        lost2, deadly2, _ = self.reply(y2)
        return len(ours) - len(theirs) + 0.25 * vor - 1000 * deadly - 0.5 * lost2 - 300 * deadly2 + 0.05 * x.gold, ok

    def climb(self, s0, cmds, log, roles):
        """From the turn the roles made, one change at a time while one
        raises the score of the position after the enemy's reply: a unit's
        step (or none), a purchase removed, or one added (a level-1 or
        level-2 training, a tower)."""
        import time
        t0 = time.time()
        best, cmds = self.evaluate(s0, cmds)
        start = best
        evals = 1
        changes = []
        my_units = [(p, v) for p, v in s0.units.items() if v[0] == 0]
        while time.time() - t0 < BUDGET:
            improved = False
            trials = []
            for p, v in my_units:
                rest = [c for c in cmds if not (c[0] == 'MOVE' and c[1] == v[1])]
                trials.append((rest, f'unit {v[1]} stays'))
                for n in nbrs(p):
                    if n in s0.valid:
                        trials.append((moves_first(rest + [('MOVE', v[1], n)]), f'unit {v[1]} to {n}'))
            for i, c in enumerate(cmds):
                if c[0] != 'MOVE':
                    trials.append((cmds[:i] + cmds[i + 1:], f'no {c[0].lower()} at {c[1]}'))
            x, _ = self.apply(s0, cmds)
            near = {n for e in x.theirs for n in nbrs(e)} | {n for e in x.theirs for m in nbrs(e) for n in nbrs(m)}
            for cell in self.trainable_any(x):
                if cell not in near:
                    continue
                if x.gold >= 10 and enterable(x, cell, 0, 1):
                    trials.append((cmds + [('TRAIN', cell)], f'train at {cell}'))
                elif x.gold >= 20 and enterable(x, cell, 0, 2) and cell in x.units:
                    trials.append((cmds + [('TRAIN2', cell)], f'level 2 at {cell}'))
            if x.gold >= 15:
                for cell in active(x, 0):
                    if cell in near and cell not in x.units and cell not in x.buildings and cell not in self.mine_spots:
                        trials.append((cmds + [('BUILD', cell)], f'tower at {cell}'))
            top = None
            for trial, name in trials:
                if time.time() - t0 >= BUDGET:
                    break
                sc, ok = self.evaluate(s0, trial)
                evals += 1
                if sc > best + 1e-9 and (top is None or sc > top[0]):
                    top = (sc, ok, name)
            if top is None:
                # No single change helps: pairs of changes, a purchase with
                # a step or another purchase.
                adds = [(tr, nm) for tr, nm in trials if len(tr) > len(cmds)]
                for (a, na), (b, nb) in itertools.combinations(trials, 2):
                    if time.time() - t0 >= BUDGET:
                        break
                    if not ((a, na) in adds or (b, nb) in adds):
                        continue
                    extra = [c for c in b if c not in cmds]
                    gone = [c for c in cmds if c not in b]
                    trial = moves_first([c for c in a if c not in gone] + extra)
                    sc, ok = self.evaluate(s0, trial)
                    evals += 1
                    if sc > best + 1e-9 and (top is None or sc > top[0]):
                        top = (sc, ok, na + ' and ' + nb)
            if top is None:
                break
            best, cmds = top[0], top[1]
            changes.append(top[2])
            improved = True
        log['climb'] = dict(start=round(start, 2), end=round(best, 2), evals=evals, seconds=round(time.time() - t0, 1), changes=changes)
        # Roles of what the climb changed.
        for name in changes:
            if name.startswith('unit '):
                uid = int(name.split()[1])
                r = roles.get(uid, {})
                r = dict(r, replanned=name)
                roles[uid] = r
        for c in cmds:
            if c[0] in ('TRAIN', 'TRAIN2'):
                roles.setdefault('new %d,%d' % c[1], dict(role='fight', label=None))
        for k in [k for k in roles if isinstance(k, str) and k.startswith('new ')]:
            xy = tuple(map(int, k[4:].split(',')))
            if not any(c[0] in ('TRAIN', 'TRAIN2') and c[1] == xy for c in cmds):
                del roles[k]
        return cmds

    def towers(self, s, t, free_units, arr, EH, cmds, log):
        base_lost, base_deadly, entered = self.reply(s)
        log['threat'] = dict(lost=base_lost, deadly=base_deadly, entered=[list(c) for c in entered])
        act = active(s, 0)
        # Candidate spots: our free active cells on or next to the cells
        # the reply enters (a tower protects its cell and its neighbours).
        spots_c = {c for e in entered for c in [e] + list(nbrs(e)) if c in act and c not in s.units and c not in s.buildings and c not in self.mine_spots}
        if base_deadly:
            spots_c |= {c for c in nbrs((0, 0)) if c in act and c not in s.units and c not in s.buildings and c not in self.mine_spots}
        spots = []
        for c in spots_c:
            x = clone(s)
            x.buildings[c] = (0, 2)
            lost, deadly, _ = self.reply(x)
            if lost < base_lost or deadly < base_deadly:
                spots.append((base_lost - lost + 1000 * (base_deadly - deadly), c))
        spots = [c for _, c in sorted(spots, reverse=True)[:4]]
        combos = [()] + [(c,) for c in spots] + list(itertools.combinations(spots, 2))
        best = None
        for combo in combos:
            if 15 * len(combo) > s.gold:
                continue
            x = clone(s)
            for c in combo:
                x.buildings[c] = (0, 2)
            x.gold -= 15 * len(combo)
            lost, deadly, _ = self.reply(x)
            saved = (self.trained, list(self.opened))
            picks = self.options(x, t, free_units, x.gold // 10, arr, EH, {})
            self.trained, self.opened = saved
            value = sum(p[3]['gain'] * FACTOR[max(-1, min(1, p[3]['margin']))] for p in picks)
            score = value - 2 * lost - 1000 * deadly
            row = dict(towers=[list(c) for c in combo], value=value, lost=lost, deadly=deadly, score=score)
            log.setdefault('tower_choices', []).append(row)
            if best is None or score > best[0]:
                best = (score, combo)
        for c in best[1]:
            s.buildings[c] = (0, 2)
            s.gold -= 15
            cmds.append(('BUILD', c))
        log['towers'] = [list(c) for c in best[1]]

    def reply(self, s):
        """The enemy's guessed reply for one turn, as our search's replies:
        its units take the cells of ours whose loss costs us most, then its
        gold pays for the chain of trainings that does most damage (the
        cells it takes and the cells it cuts off from our HQ), through our
        cells (10 gold, 20 on a level-1 unit) and the neutral cells next to
        our land, then for single trainings with the gold left. Returns the
        cells we lose, whether the enemy can then pay for a chain to our HQ,
        and the cells the reply enters."""
        x = clone(s)
        gold = s.egold + s.eincome
        entered = []
        before0 = active(x, 0)

        def damage_of(cells):
            removed = set(cells)
            cur = set(bfs(x.valid, [(0, 0)], x.ours)) if (0, 0) in x.ours else set()
            after = set(bfs(x.valid, [(0, 0)], x.ours - removed)) if (0, 0) not in removed else set()
            return sum(1 for c in cells if c in x.ours) + len(cur - after - removed)

        for pos, v in list(x.units.items()):
            if v[0] != 1:
                continue
            cand = [n for n in nbrs(pos) if n in x.ours and enterable(x, n, 1, v[2])]
            if not cand:
                # No capture: a step into the neutral cell nearest our land,
                # to cut at the next turn.
                dOurs = bfs(x.valid, list(x.ours))
                steps = [n for n in nbrs(pos) if n in x.valid and n not in x.ours and n not in x.theirs and n not in x.units and dOurs.get(n, 99) < dOurs.get(pos, 99)]
                if steps:
                    n = min(steps, key=lambda n: (dOurs.get(n, 99), n))
                    del x.units[pos]
                    take(x, n, 1)
                    x.units[n] = v
                continue
            c = max(cand, key=lambda n: (damage_of([n]), n))
            if c == (0, 0):
                self.reply_state = x
                return 100, 1, entered + [c]
            entered.append(c)
            del x.units[pos]
            x.units.pop(c, None)
            take(x, c, 1)
            x.units[c] = v
        # The best chain, by depth-first search.
        near = {n for c in x.ours for n in nbrs(c) if n in x.valid} | set(x.ours)

        def cost(c):
            if c in x.theirs:
                return None
            for level, g in ((1, 10), (2, 20)):
                if enterable(x, c, 1, level):
                    return g
            return None

        act = active(x, 1)
        starts = {n for a in act for n in nbrs(a) if n in near and n not in x.theirs}
        best = [0, []]
        seen = 0

        def dfs(path, used, budget):
            nonlocal seen
            seen += 1
            if seen > 1500:
                return
            if path:
                d = damage_of(path)
                if (0, 0) in path:
                    d = 1000
                if d > best[0]:
                    best[0], best[1] = d, list(path)
            frontier = starts if not path else {n for n in nbrs(path[-1]) if n in near and n not in x.theirs}
            for n in sorted(frontier):
                if n in used:
                    continue
                g = cost(n)
                if g is None or g > budget:
                    continue
                path.append(n)
                used.add(n)
                dfs(path, used, budget - g)
                path.pop()
                used.discard(n)

        dfs([], set(), gold)
        for c in best[1]:
            if c == (0, 0):
                self.reply_state = x
                return 100, 1, entered + best[1]
            gold -= cost(c) or 10
            entered.append(c)
            x.units.pop(c, None)
            take(x, c, 1)
            x.units[c] = (1, -1, 1)
        while gold >= 10:
            act = active(x, 1)
            cand = [n for n in {n for a in act for n in nbrs(a) if n in x.ours} if cost(n) is not None and cost(n) <= gold]
            if not cand:
                break
            c = max(cand, key=lambda n: (damage_of([n]) / cost(n), n))
            if c == (0, 0):
                self.reply_state = x
                return 100, 1, entered + [c]
            gold -= cost(c)
            entered.append(c)
            x.units.pop(c, None)
            take(x, c, 1)
            x.units[c] = (1, -1, 1)
        after = active(x, 0)
        lost = len(before0 - after)
        nxt = gold + len(active(x, 1)) - sum(1 for v in x.units.values() if v[0] == 1)
        deadly = 1 if self.chain_cost(x) <= nxt else 0
        self.reply_state = x
        self.reply_next_gold = nxt
        return lost, deadly, entered

    def chain_cost(self, s):
        """The gold of the enemy's cheapest chain of trainings from its
        active land to our HQ (10, 20 or 30 a cell by the level it needs)."""
        import heapq
        act = active(s, 1)
        start = {n for c in act for n in nbrs(c) if n in s.valid and n not in s.theirs}
        def need(c):
            if c in s.theirs:
                return 0
            if c in s.ours and (protected(s, c, 0) or s.buildings.get(c, (None, None))[1] == 2):
                return 30
            u = s.units.get(c)
            if u is not None and u[0] == 0:
                return 30 if u[2] >= 2 else 20
            return 10
        best = {}
        h = [(need(c), c) for c in start]
        heapq.heapify(h)
        while h:
            d, c = heapq.heappop(h)
            if c in best:
                continue
            best[c] = d
            if c == (0, 0):
                return d
            for n in nbrs(c):
                if n in s.valid and n not in best:
                    heapq.heappush(h, (d + need(n), n))
        return 10 ** 6

    # Options and their weights.
    def options(self, s, t, units, n_train, arr, EH, log):
        agents = [('unit', c) for c in units] + [('train', i) for i in range(n_train)]
        trainable = self.trainable(s)
        dist_cache = {}

        def dist_from(c):
            if c not in dist_cache:
                dist_cache[c] = bfs(s.valid, [c])
            return dist_cache[c]

        def d_agent(agent, cell):
            dc = dist_from(cell)
            if agent[0] == 'unit':
                return dc.get(agent[1], 99)
            return 1 + min((dc.get(c, 99) for c in trainable), default=99)

        subs = []
        if t < 4:
            # The turn-1 plan: the open sub-goals, static.
            for a, k in self.opened:
                c = self.static_sub(a, k)
                if c not in s.ours:
                    subs.append((c, f"{AREAS[a]['name'].split()[0]} {k}"))
            # Trainings open the plan's next sub-goals.
            nxt = SEQ[self.trained:self.trained + n_train]
            for o in nxt:
                for a, k in o:
                    c = self.static_sub(a, k)
                    if c not in s.ours and all(c != x for x, _ in subs):
                        subs.append((c, f"{AREAS[a]['name'].split()[0]} {k}"))
                self.opened += [tuple(x) for x in o]
            self.trained += len(nxt)
            # The nearest pair first.
            pairs = sorted((d_agent(ag, c), i, j) for i, ag in enumerate(agents) for j, (c, _) in enumerate(subs))
            used_a, used_s, picks = set(), set(), []
            for d, i, j in pairs:
                if i not in used_a and j not in used_s:
                    used_a.add(i)
                    used_s.add(j)
                    picks.append((agents[i], subs[j][0], subs[j][1], None))
            # Open sub-goals reached go on to the area's next.
            order = ['E', 'C', 'O']
            self.opened = [(a, order[order.index(k) + 1]) if self.static_sub(a, k) in s.ours and k != 'O' else (a, k) for a, k in self.opened]
            self.opened = [(a, k) for a, k in self.opened if self.static_sub(a, k) not in s.ours]
            log['subgoals'] = [[a, k, list(self.static_sub(a, k))] for a in range(len(AREAS)) for k in order]
            log['picks'] = [[p[0][0], list(p[1]), p[2]] for p in picks]
            return picks
        # From turn 4: sub-goals move with the enemy.
        def margin(cell, agent):
            return arr.get(cell, 99) - d_agent(agent, cell)

        def best_margin(cell):
            return max((margin(cell, ag) for ag in agents), default=-99)

        cur = []
        for ai, a in enumerate(AREAS):
            E0, C0 = tuple(a['sub']['E']), tuple(a['sub']['C'])
            axis = central_path(s.valid, E0, C0)
            beyond = central_path(s.valid, C0, (11, 11))[1:7]
            C = None
            for c in axis:
                if c in s.theirs:
                    break
                if best_margin(c) >= 0 or c in s.ours:
                    C = c
            O = None
            if C == C0:
                for c in beyond:
                    if c in s.theirs:
                        break
                    if best_margin(c) >= 1 or c in s.ours:
                        O = c
                    else:
                        break
            for k, c in (('E', E0), ('C', C), ('O', O)):
                if c is not None:
                    cur.append((ai, k, c))
        log['subgoals'] = [[a, k, list(c)] for a, k, c in cur]
        # A cell is decided for us when one of the agents already committed
        # reaches it (its steps to its option and on from it) no later than
        # the enemy's guessed arrival; an option is worth the cells it adds.
        base = {c: 0 for c in s.ours}

        def gain(cell, d0):
            dc = dist_from(cell)
            return sum(1 for x, j in arr.items() if j > 0 and base.get(x, 99) > j and d0 + dc.get(x, 99) <= j)

        picks, used = [], set()
        cand = [(a, k, c) for a, k, c in cur if c not in s.ours]
        while cand:
            best = None
            for a, k, c in cand:
                for i, ag in enumerate(agents):
                    if i in used:
                        continue
                    g = gain(c, d_agent(ag, c))
                    m = margin(c, ag)
                    f = FACTOR[max(-1, min(1, m))] if m < 99 else 0
                    v = g * f
                    key = (v, -d_agent(ag, c))
                    if best is None or key > best[0]:
                        best = (key, i, a, k, c, g, m)
            if best is None or best[0][0] <= 0:
                break
            _, i, a, k, c, g, m = best
            used.add(i)
            picks.append((agents[i], c, f"{AREAS[a]['name'].split()[0]} {k}", dict(gain=g, margin=m)))
            dc = dist_from(c)
            d0 = d_agent(agents[i], c)
            for x, v in dc.items():
                if d0 + v < base.get(x, 99):
                    base[x] = d0 + v
            cand = [x for x in cand if x[2] != c]
        log['picks'] = [[p[0][0], list(p[1]), p[2], p[3]] for p in picks]
        return picks

    def territory(self, s, EH):
        dL, dE = bfs(s.valid, list(s.ours)), bfs(s.valid, list(EH))
        return {c for c in s.valid if c not in s.ours and c not in EH and dL.get(c, 99) < dE.get(c, 99)}

    def trainable(self, s):
        act = active(s, 0)
        cells = set(act) | {n for c in act for n in nbrs(c)}
        return [c for c in cells if enterable(s, c, 0)]

    def do_train(self, s, c):
        if s.gold < 10 or c not in self.trainable(s):
            return False
        if c in s.units:
            del s.units[c]
        take(s, c, 0)
        s.units[c] = (0, -1, 1)
        s.gold -= 10
        return True

    def train_toward(self, s, goal):
        dg = bfs(s.valid, [goal])
        cand = [c for c in self.trainable(s) if c not in s.ours or c == goal]
        cand = cand or self.trainable(s)
        if not cand:
            return None
        c = min(cand, key=lambda c: (dg.get(c, 99), -sum(1 for m in nbrs(c) if m in s.valid), abs(c[0] - c[1]), c))
        return c if self.do_train(s, c) else None

    def move(self, s, pos, to, cmds):
        u = s.units.get(pos)
        if u is None or u[0] != 0 or u[1] < 0 or not enterable(s, to, 0, u[2]):
            return False
        del s.units[pos]
        if to in s.units:
            del s.units[to]
        take(s, to, 0)
        s.units[to] = u
        cmds.append(('MOVE', u[1], to))
        return True

    def opened_cells(self, c):
        return sum(1 for n in nbrs(c))

    def step(self, s, pos, goal, cmds):
        dg = bfs(s.valid, [goal])
        here = dg.get(pos, 99)
        steps = [n for n in nbrs(pos) if n in dg and dg[n] < here and enterable(s, n, 0)]
        if not steps:
            return
        n = min(steps, key=lambda n: (n in s.ours, -sum(1 for m in nbrs(n) if m in s.valid), n))
        self.move(s, pos, n, cmds)

    def hold(self, s, hold, dEn, cmds, roles):
        """Units near the front line stay there: each takes the free cell
        within 2 steps of it nearest the front line, else stays to block."""
        reserved = set()
        for pos in hold:
            u = s.units.get(pos)
            if u is None:
                continue
            du = bfs(s.valid, [pos])
            cand = [c for c in du if du[c] <= 2 and c not in s.ours and c not in s.theirs and dEn.get(c, 99) <= dEn.get(pos, 99) and c not in reserved]
            roles[u[1]] = dict(role='hold')
            if not cand:
                continue
            goal = min(cand, key=lambda c: (dEn.get(c, 99), du[c], c))
            reserved.add(goal)
            self.step(s, pos, goal, cmds)

    def backline(self, s, back, terr, cmds, roles):
        if not back:
            return
        own, dd = {}, {}
        q = deque()
        for i, h in enumerate(back):
            own[h], dd[h] = i, 0
            q.append(h)
        while q:
            c = q.popleft()
            for n in nbrs(c):
                if n in s.valid and n not in dd and (n in terr or n in s.ours):
                    dd[n] = dd[c] + 1
                    own[n] = own[c]
                    q.append(n)
        reserved = set()
        for i, pos in enumerate(back):
            u = s.units.get(pos)
            if u is None:
                continue
            share = [c for c, o in own.items() if o == i and c in terr and c not in reserved]
            pool = share or [c for c in terr if c not in reserved]
            roles[u[1]] = dict(role='backline', share=len(share))
            if not pool:
                continue
            du = bfs(s.valid, [pos])
            goal = min(pool, key=lambda c: (du.get(c, 99), c))
            reserved.add(goal)
            self.step(s, pos, goal, cmds)

    def fight(self, s, fighting, cmds, roles):
        fighting = sorted(fighting, key=lambda c: min((abs(c[0] - e[0]) + abs(c[1] - e[1]) for e in s.theirs), default=99))[:4]
        options = []
        for pos in fighting:
            u = s.units[pos]
            opts = [None] + [n for n in nbrs(pos) if enterable(s, n, 0, u[2])]
            options.append(opts)
        best = None
        for combo in itertools.product(*options):
            if len({c for c in combo if c}) < sum(1 for c in combo if c):
                continue
            sc = self.fight_score(s, fighting, combo)
            if best is None or sc > best[0]:
                best = (sc, combo)
        for pos, to in zip(fighting, best[1]):
            u = s.units.get(pos)
            if u is None:
                continue
            if to is not None:
                self.move(s, pos, to, cmds)
            roles[u[1]] = dict(role='fight', to=list(to) if to else None, score=best[0][0])

    def fight_score(self, s, fighting, combo):
        import copy
        x = State()
        x.valid, x.rows = s.valid, s.rows
        x.ours, x.theirs = set(s.ours), set(s.theirs)
        x.buildings, x.units = dict(s.buildings), dict(s.units)
        gained = 0
        for pos, to in zip(fighting, combo):
            if to is None or not enterable(x, to, 0, x.units[pos][2]):
                continue
            u = x.units.pop(pos)
            gained += 2 if to in x.theirs else 1 if to not in x.ours else 0
            x.units.pop(to, None)
            take(x, to, 0)
            x.units[to] = u
        # The enemy's reply: each unit takes one of our cells it may enter,
        # then its gold trains on our cells next to its active land.
        lost = set()
        for c, v in x.units.items():
            if v[0] != 1:
                continue
            for n in nbrs(c):
                if n in x.ours and n not in lost and enterable(x, n, 1, v[2]):
                    lost.add(n)
                    break
        act = active(x, 1)
        k = (s.egold + s.eincome) // 10
        for c in sorted({n for a in act for n in nbrs(a) if n in x.ours and n not in lost and enterable(x, n, 1)}):
            if k <= 0:
                break
            lost.add(c)
            k -= 1
        block = sum(1 for to in combo if to is not None)
        return (gained - len(lost), -block)

    # The turn-1 plan played alone, without the enemy, to game turn 12.
    def predict_plan(self, s):
        import copy
        x = State()
        x.valid, x.rows = s.valid, s.rows
        x.ours, x.theirs = set(s.ours), set()
        x.buildings = {c: v for c, v in s.buildings.items() if v[0] == 0}
        x.units = {}
        saved = (self.trained, list(self.opened), self.t)
        nid = 10_000
        lands = []
        gold = s.gold
        for t in range(1, LAST_TURN + 1):
            self.t = t
            x.gold, x.egold, x.eincome = gold, 0, 0
            for c, v in list(x.units.items()):
                if v[1] < 0:
                    x.units[c] = (0, nid, 1)
                    nid += 1
            free_units = [c for c, v in x.units.items() if v[0] == 0]
            picks = self.options(x, t if t < 4 else 1, free_units, x.gold // 10, {}, frozenset(), {}) if t < 4 else self.options_plan_late(x, free_units)
            cmds = []
            for agent, cell, label, _ in picks:
                if agent[0] == 'unit':
                    self.step(x, agent[1], cell, cmds)
            moved = {c[2] for c in cmds if c[0] == 'MOVE'}
            back = [c for c in x.units if c not in moved and x.units[c][0] == 0 and c not in [p[0][1] for p in picks if p[0][0] == 'unit']]
            self.backline(x, back, {c for c in x.valid if c not in x.ours}, cmds, {})
            for agent, cell, label, _ in picks:
                if agent[0] == 'train':
                    self.train_toward(x, cell)
            while x.gold >= 10:
                dO, dX = bfs(x.valid, [(0, 0)]), bfs(x.valid, [(11, 11)])
                front = [c for c in self.trainable(x) if c not in x.ours]
                if not front:
                    break
                c = min(front, key=lambda c: (abs(dO.get(c, 99) - dX.get(c, 99)), c))
                self.do_train(x, c)
            lands.append(sorted(map(list, x.ours)))
            gold = x.gold + len(x.ours) - len(x.units)
        self.trained, self.opened, self.t = saved
        return dict(lands=lands, units=[list(c) for c in x.units])

    def options_plan_late(self, x, units):
        # After turn 3 in the plan alone: the open sub-goals still matched.
        subs = []
        for a, k in self.opened:
            c = self.static_sub(a, k)
            if c not in x.ours:
                subs.append((c, f"{AREAS[a]['name'].split()[0]} {k}"))
        order = ['E', 'C', 'O']
        self.opened = [(a, order[order.index(k) + 1]) if self.static_sub(a, k) in x.ours and k != 'O' else (a, k) for a, k in self.opened]
        self.opened = [(a, k) for a, k in self.opened if self.static_sub(a, k) not in x.ours]
        agents = [('unit', c) for c in units]
        pairs = sorted((bfs(x.valid, [c]).get(ag[1], 99), i, j) for i, ag in enumerate(agents) for j, (c, _) in enumerate(subs))
        used_a, used_s, picks = set(), set(), []
        for d, i, j in pairs:
            if i not in used_a and j not in used_s:
                used_a.add(i)
                used_s.add(j)
                picks.append((agents[i], subs[j][0], subs[j][1], None))
        return picks


if __name__ == '__main__':
    Bot().run()
