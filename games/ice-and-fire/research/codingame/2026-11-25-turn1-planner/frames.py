"""Builds actions.json: every action of the 8 games, in our view (our HQ
top-left), with the role and reason of each of ours from the bot's log."""
import json
AREAS = json.load(open('areas.json'))
NAMES = {'free': 'free growth', 'momentum': 'momentum', 'mirror': 'our own plan', 'rush': 'fastest rush'}
out = {}
for m in ['free', 'momentum', 'mirror', 'rush']:
    for seat in ['red', 'blue']:
        us = 0 if seat == 'red' else 1
        flip = us == 1
        F = lambda x, y: (11 - x, 11 - y) if flip else (x, y)
        acts = [json.loads(l) for l in open(f'act-{m}-{seat}.jsonl')]
        logs = {d['t']: d for d in map(json.loads, open(f'log-{m}-{seat}.jsonl'))}
        rec = json.load(open(f'rec-{m}-{seat}.json'))
        hq_lost = rec['winner'] is not None and rec['seats'][rec['winner']] != 'plan'
        frames = []
        prev_units = {}
        our_turn = 0
        k = 0
        for a in acts:
            b = a['board']
            rows = b['rows']
            if flip:
                rows = [r[::-1] for r in rows[::-1]]
                rows = [r.translate(str.maketrans('OoXx', 'XxOo')) for r in rows]
            units = [[*F(x, y), 0 if s == us else 1, i, l] for x, y, s, i, l in b['units']]
            towers = [[*F(x, y), 0 if s == us else 1] for x, y, s in b['towers']]
            who = 'us' if a['seat'] == us else 'v020'
            if a['cmd'] is None:
                if who == 'us':
                    our_turn += 1
                    k = 0
                prev_units = {u[3]: (u[0], u[1]) for u in units}
                if who == 'us':
                    frames.append(dict(t=a['turn'], lt=our_turn, who=who, start=True, rows=rows, units=units, towers=towers, gold=[b['gold'][us], b['gold'][1 - us]]))
                continue
            w = a['cmd'].split()
            hl, desc, why = None, '', ''
            log = logs.get(our_turn, {})
            if w[0] == 'MOVE':
                uid = int(w[1])
                to = list(F(int(w[2]), int(w[3])))
                fr = prev_units.get(uid)
                hl = dict(kind='move', frm=list(fr) if fr else None, to=to)
                desc = f"unit {uid} moves to ({to[0]}, {to[1]})"
                if who == 'us':
                    r = log.get('roles', {}).get(str(uid), {})
                    why = r.get('role', '')
                    if r.get('label'):
                        why += ' toward ' + r['label']
                    if r.get('replanned'):
                        why += ' (changed by the whole-turn search)'
            elif w[0] == 'TRAIN':
                to = list(F(int(w[2]), int(w[3])))
                hl = dict(kind='train', to=to)
                desc = f"trains a level-{w[1]} unit at ({to[0]}, {to[1]})"
                if who == 'us':
                    r = log.get('roles', {}).get(f'new {to[0]},{to[1]}', {})
                    why = r.get('role', '')
                    if r.get('label'):
                        why += ' toward ' + r['label']
                    added = [c for c in log.get('climb', {}).get('changes', []) if f'({to[0]}, {to[1]})' in c]
                    if added:
                        why = 'added by the whole-turn search: ' + added[0]
            elif w[0] == 'BUILD':
                to = list(F(int(w[2]), int(w[3])))
                hl = dict(kind='tower', to=to)
                desc = f"builds a tower at ({to[0]}, {to[1]})"
                if who == 'us':
                    added = [c for c in log.get('climb', {}).get('changes', []) if f'({to[0]}, {to[1]})' in c]
                    why = 'added by the whole-turn search' if added else 'guards the cells of the threat'
            prev_units = {u[3]: (u[0], u[1]) for u in units}
            k += 1
            frames.append(dict(t=a['turn'], lt=our_turn, who=who, desc=desc, why=why, ok=a['ok'], hl=hl, rows=rows, units=units, towers=towers, gold=[b['gold'][us], b['gold'][1 - us]]))
        turns = {}
        for t, d in logs.items():
            turns[t] = dict(picks=d.get('picks'), threat=d.get('threat'), climb=d.get('climb'), roles=d.get('roles', {}), subgoals=d.get('subgoals'))
        out[f'{m}|{seat}'] = dict(name=f"{NAMES[m]}, we play {seat}", frames=frames, turns=turns, hq_lost=hq_lost,
                                  plan=logs[1]['plan']['lands'])
json.dump(dict(areas=AREAS, games=out), open('actions.json', 'w'), separators=(',', ':'))
for k, g in out.items():
    f = g['frames'][-1]
    print(k, len(g['frames']), 'frames; ours', sum(r.count('O') + r.count('o') for r in f['rows']), 'v020', sum(r.count('X') + r.count('x') for r in f['rows']), 'HQ lost' if g['hq_lost'] else '')
