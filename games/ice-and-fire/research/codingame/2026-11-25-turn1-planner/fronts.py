import json, os, csv
from collections import deque
P=os.path.dirname(os.path.abspath(__file__)); S=os.path.dirname(P)
def bfs(rows, src):
    d=[[None]*12 for _ in range(12)]; q=deque()
    for (x,y) in src: d[y][x]=0; q.append((x,y))
    while q:
        x,y=q.popleft()
        for dx,dy in ((1,0),(-1,0),(0,1),(0,-1)):
            a,b=x+dx,y+dy
            if 0<=a<12 and 0<=b<12 and rows[b][a]!='#' and d[b][a] is None:
                d[b][a]=d[y][x]+1; q.append((a,b))
    return d
def lane(x,y):
    k=x-y
    return 0 if k<=-4 else 2 if k>=4 else 1
NAMES=['bottom-left flank','middle','top-right flank']
def nearer(rows,a,b):
    da,db=bfs(rows,a),bfs(rows,b); n=0
    for y in range(12):
        for x in range(12):
            if rows[y][x]!='#' and da[y][x] is not None and (db[y][x] is None or da[y][x]<db[y][x]): n+=1
    return n
out=[]
fr10={}
for r in csv.DictReader(open(S+'/fronts.csv')):
    if r['turn']=='10': fr10[(r['game_id'],r['seat'],r['lane'])]=int(r['nearer'])
for line in open(P+'/raw.jsonl'):
    g=json.loads(line); b1=g['boards'][0]; rows=b1['rows']
    A=bfs(rows,[(0,0)]); B=bfs(rows,[(11,11)])
    ring=[[None if rows[y][x]=='#' or A[y][x] is None or B[y][x] is None else abs(A[y][x]-B[y][x])//2 for x in range(12)] for y in range(12)]
    parts=[[],[],[]]
    for y in range(12):
        for x in range(12):
            if ring[y][x] is not None and ring[y][x]<=1: parts[lane(x,y)].append((x,y))
    kept=[l for l in range(3) if len(parts[l])>=4]
    ds={l:bfs(rows,parts[l]) for l in kept}
    region={}
    for y in range(12):
        for x in range(12):
            if rows[y][x]=='#': continue
            best=min(kept,key=lambda l:((ds[l][y][x] if ds[l][y][x] is not None else 99),0 if l==lane(x,y) else 1 if l==1 else 2))
            region[(x,y)]=best
    ours=[(x,y) for y in range(12) for x in range(12) if rows[y][x]=='O']
    theirs=[(x,y) for y in range(12) for x in range(12) if rows[y][x]=='X']
    dO,dX=bfs(rows,ours),bfs(rows,theirs)
    fronts=[]
    for l in kept:
        F=parts[l]
        value=nearer(rows,ours+F,theirs)-nearer(rows,ours,theirs+F)
        us=min(dO[y][x] for x,y in F if dO[y][x] is not None); them=min(dX[y][x] for x,y in F if dX[y][x] is not None)
        margin=them-us
        # Blue starts a step nearer to every front (two cells to red's one):
        # the race is read against that step.
        rel=margin+1
        status='race' if abs(rel)<=1 else 'ours by the map' if rel>=2 else 'lost to the map'
        key=str(g['game_id'])
        t10=None
        k10={0:'bottom_left',1:'middle',2:'top_right'}[l]
        if (key,'0',k10) in fr10: t10=fr10[(key,'0',k10)]-fr10[(key,'1',k10)]
        fronts.append(dict(lane=l,name=NAMES[l],cells=F,region=sum(1 for v in region.values() if v==l),value=value,us=us,them=them,margin=margin,rel=rel,status=status,prio=value/max(1,us),t10=t10))
    goals=sorted([f for f in fronts if f['status']=='race'],key=lambda f:(-f['rel'],-f['prio']))
    for i,f in enumerate(goals): f['rank']=i+1
    kind={(0,1,2):'three fronts',(0,2):'two flanks only',(1,):'middle only'}.get(tuple(kept),'other')
    out.append(dict(game_id=g['game_id'],blue=g['blue'],winner=g['winner'],kind=kind,boards=g['boards'],region={f'{x},{y}':v for (x,y),v in region.items()},fronts=fronts))
    print(g['game_id'],kind,[(f['name'],f['value'],f['us'],f['them'],f['rel'],f['status'],f.get('rank'),f['t10']) for f in fronts])
order={'three fronts':0,'two flanks only':1,'middle only':2}
out.sort(key=lambda g:(order.get(g['kind'],3),g['game_id']))
json.dump(out,open(P+'/goals.json','w'),separators=(',',':'))
