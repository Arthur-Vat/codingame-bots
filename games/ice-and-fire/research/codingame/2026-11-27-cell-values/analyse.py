import json, sys, collections
S = sys.argv[1]
d = json.load(open(S + '/cells.json'))
ranking = set()
for l in open(S + '/index.jsonl'):
    x = json.loads(l)
    if x.get('release') == 'ice-and-fire-v032' and x['kind'] != 'test':
        ranking.add(x['game_id'])

def auc(bins):
    # bins: list of (order_key, n, lost) with lower key = predicted more dangerous
    bins = sorted(bins)
    P = sum(b[2] for b in bins); N = sum(b[1]-b[2] for b in bins)
    if P == 0 or N == 0: return float('nan')
    # prob a random lost cell ranks more dangerous than a random kept cell (ties half)
    s = 0; neg_after = N
    for k, n, l in bins:
        neg = n - l
        neg_after -= neg
        s += l * (neg_after + neg / 2)
    return s / (P * N)

def key(feat, b):
    if b == 'none': return 10**6
    return int(b)

groups = collections.defaultdict(list)
for k, (n, l) in d['fights'].items():
    ph, who, feat, b = k.split('|')
    groups[(ph, who, feat)].append((key(feat, b), n, l, b))

print('## AUC: does the feature rank the cells lost this ply above those kept')
print('phase who cells lost% | cut direct steps')
for ph in ['t01-06','t07-12','t13-20','t21-40','t41+']:
    for who in ['them','us']:
        row = []
        tot = None
        for feat in ['cut','direct','steps']:
            g = groups[(ph, who, feat)]
            row.append(auc([(a,n,l) for a,n,l,_ in g]))
            tot = (sum(x[1] for x in g), sum(x[2] for x in g))
        print(f'{ph} {who:4} {tot[0]:7} {100*tot[1]/tot[0]:5.2f} | ' + ' '.join(f'{v:.3f}' for v in row))

print('\n## loss rate by margin (price with cuts - attacker gold), all phases from t07, attacker = them / us')
for who in ['them','us']:
    agg = collections.defaultdict(lambda: [0,0])
    for (ph, w, feat), g in groups.items():
        if w != who or feat != 'cut' or ph == 't01-06': continue
        for a, n, l, b in g:
            if b == 'none': kk = 'none'
            else:
                v = int(b)
                kk = '<=-100' if v <= -100 else ('-90..-50' if v <= -50 else ('-40..-10' if v < 0 else ('0..30' if v < 40 else ('40..90' if v < 100 else '>=100'))))
            agg[kk][0] += n; agg[kk][1] += l
    for kk in ['<=-100','-90..-50','-40..-10','0..30','40..90','>=100','none']:
        n, l = agg[kk]
        print(f'  {who:4} {kk:9} cells {n:8}  lost {100*l/max(n,1):6.2f}%')

print('\n## how D lost its cells: captured vs severed (cut off)')
for k, (cap, sev) in sorted(d['lost_kind'].items()):
    print(f'  {k:12} captured {cap:6} severed {sev:6} ({100*sev/max(cap+sev,1):.0f}% severed)')

print('\n## early game: P(red holds the neutral cell after turn 12) by contest margin m = blue steps - red steps')
early = collections.defaultdict(dict)
for k, v in d['early'].items():
    t, m = k.split('|'); early[t][int(m)] = v
print('m   ' + ' '.join(f'{t:>16}' for t in sorted(early)))
for m in range(-4, 5):
    cells = []
    for t in sorted(early):
        v = early[t].get(m, [0,0,0]); n = sum(v)
        cells.append(f'{100*v[0]/max(n,1):4.0f}/{100*v[1]/max(n,1):3.0f} n={n:5}')
    print(f'{m:+d}  ' + ' '.join(f'{c:>16}' for c in cells))

games = [g for g in d['games'] if g['game'] not in ranking]
print(f'\n## {len(games)} test games (we are red)')
ends = collections.Counter(g['end'] for g in games)
print('  ends:', dict(ends))
lens = sorted(g['turns'] for g in games)
q = lambda p: lens[int(p*(len(lens)-1))]
print(f'  game turns: median {q(.5)}, quartiles {q(.25)}-{q(.75)}, 90% {q(.9)}; reach turn 100: {sum(1 for g in games if g["turns"]>=100)}')
won = [g for g in games if g['winner'] == 0]
print(f'  we win {len(won)/len(games):.3f}')
for name, f in [('active cells after t12', lambda g: g['active12'][0]-g['active12'][1]),
                ('contested-at-t4 cells held after t12', lambda g: g['contested12'][0]-g['contested12'][1]),
                ('income after t12', lambda g: g['income12'][0]-g['income12'][1])]:
    pos = [g for g in games if f(g) > 0]; neg = [g for g in games if f(g) < 0]; eq = [g for g in games if f(g) == 0]
    wr = lambda s: sum(1 for g in s if g['winner']==0)/max(len(s),1)
    # AUC of the difference vs winning
    pairs = [(f(g), g['winner']==0) for g in games if g['winner'] is not None]
    P=[v for v,w in pairs if w]; N=[v for v,w in pairs if not w]
    import bisect
    Ns=sorted(N); a=0
    for v in P:
        lo=bisect.bisect_left(Ns,v); hi=bisect.bisect_right(Ns,v); a+=lo+(hi-lo)/2
    print(f'  {name:38} ahead: {len(pos):4} games win {wr(pos):.2f} | level {len(eq):4} win {wr(eq):.2f} | behind {len(neg):4} win {wr(neg):.2f} | AUC {a/(len(P)*len(N)):.3f}')
by = collections.defaultdict(list)
for g in games: by[g['opponent']].append(g)
print('  by opponent (games, our win, median turns, contested held after t12 ours/theirs mean):')
for o, gs in sorted(by.items(), key=lambda x: -len(x[1])):
    ls = sorted(g['turns'] for g in gs)
    print(f'    {o:14} {len(gs):4} {sum(g["winner"]==0 for g in gs)/len(gs):.2f} {ls[len(ls)//2]:3}  {sum(g["contested12"][0] for g in gs)/len(gs):.1f}/{sum(g["contested12"][1] for g in gs)/len(gs):.1f}')
