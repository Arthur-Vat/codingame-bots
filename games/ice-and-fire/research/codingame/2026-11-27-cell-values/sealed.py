import json, sys, bisect, math, collections
S = sys.argv[1]
d = json.load(open(S + '/cells2.json'))
ranking = set()
for l in open(S + '/index.jsonl'):
    x = json.loads(l)
    if x.get('release') == 'ice-and-fire-v032' and x['kind'] != 'test':
        ranking.add(x['game_id'])
games = [g for g in d['games'] if g['game'] not in ranking and g['winner'] is not None]
def auc(pairs):
    P = [v for v, w in pairs if w]; N = sorted(v for v, w in pairs if not w)
    a = 0
    for v in P:
        lo = bisect.bisect_left(N, v); hi = bisect.bisect_right(N, v); a += lo + (hi - lo) / 2
    return a / (len(P) * len(N))
def logit(rows):
    # rows: (x1, x2, y); fit y ~ b0 + b1 x1 + b2 x2 by Newton steps
    b = [0.0, 0.0, 0.0]
    for _ in range(25):
        g = [0.0]*3; H = [[0.0]*3 for _ in range(3)]
        for x1, x2, y in rows:
            x = (1.0, x1, x2); z = sum(bi*xi for bi, xi in zip(b, x)); p = 1/(1+math.exp(-z))
            for i in range(3):
                g[i] += (y - p) * x[i]
                for j in range(3): H[i][j] += p*(1-p)*x[i]*x[j]
        # solve H d = g
        import copy
        A = [H[i][:] + [g[i]] for i in range(3)]
        for i in range(3):
            piv = A[i][i]
            for j in range(i, 4): A[i][j] /= piv
            for k in range(3):
                if k != i:
                    f = A[k][i]
                    for j in range(i, 4): A[k][j] -= f * A[i][j]
        b = [b[i] + A[i][3] for i in range(3)]
    # standard errors
    H = [[0.0]*3 for _ in range(3)]
    for x1, x2, y in rows:
        x = (1.0, x1, x2); z = sum(bi*xi for bi, xi in zip(b, x)); p = 1/(1+math.exp(-z))
        for i in range(3):
            for j in range(3): H[i][j] += p*(1-p)*x[i]*x[j]
    return b, H
print(f'{len(games)} games; we are red\n')
print('turn | mean sealed ours/theirs | winners/losers sealed (ours when we win, theirs when they win) | AUC active diff | AUC sealed diff | AUC active+sealed | logistic: per sealed cell vs per active cell')
for t in (8, 10, 12):
    rows = []
    for g in games:
        s = next((x for x in g['seals'] if x['turn'] == t), None)
        if s is None: continue
        rows.append((s['active'][0]-s['active'][1], s['sealed'][0]-s['sealed'][1], g['winner']==0, s))
    so = sum(r[3]['sealed'][0] for r in rows)/len(rows); st = sum(r[3]['sealed'][1] for r in rows)/len(rows)
    win_s = [r[3]['sealed'][0] if r[2] else r[3]['sealed'][1] for r in rows]
    lose_s = [r[3]['sealed'][1] if r[2] else r[3]['sealed'][0] for r in rows]
    a1 = auc([(r[0], r[2]) for r in rows]); a2 = auc([(r[1], r[2]) for r in rows]); a3 = auc([(r[0]+r[1], r[2]) for r in rows])
    b, H = logit([(r[0], r[1], 1.0 if r[2] else 0.0) for r in rows])
    print(f'{t:4} | {so:.1f}/{st:.1f} ({len(rows)} games) | {sum(win_s)/len(win_s):.1f}/{sum(lose_s)/len(lose_s):.1f} | {a1:.3f} | {a2:.3f} | {a3:.3f} | {b[2]:.3f} vs {b[1]:.3f} (ratio {b[2]/b[1]:.2f})')
print('\nAt game turn 10, games level in active cells (|diff| <= 2): our score by sealed difference')
rows = []
for g in games:
    s = next((x for x in g['seals'] if x['turn'] == 10), None)
    if s and abs(s['active'][0]-s['active'][1]) <= 2: rows.append((s['sealed'][0]-s['sealed'][1], g['winner']==0))
for name, f in (('sealed ahead by 3+', lambda v: v >= 3), ('within 2', lambda v: abs(v) <= 2), ('behind by 3+', lambda v: v <= -3)):
    s = [w for v, w in rows if f(v)]
    print(f'  {name:18} {len(s):4} games, our score {sum(s)/max(len(s),1):.2f}')
print('\nBy opponent at game turn 10: sealed ours/theirs, active ours/theirs')
by = collections.defaultdict(list)
for g in games:
    s = next((x for x in g['seals'] if x['turn'] == 10), None)
    if s: by[g['opponent']].append(s)
for o, ss in sorted(by.items(), key=lambda x: -len(x[1])):
    n = len(ss)
    print(f'  {o:14} {n:4}  sealed {sum(s["sealed"][0] for s in ss)/n:4.1f}/{sum(s["sealed"][1] for s in ss)/n:4.1f}  active {sum(s["active"][0] for s in ss)/n:4.1f}/{sum(s["active"][1] for s in ss)/n:4.1f}')
