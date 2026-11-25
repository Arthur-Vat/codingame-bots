import json, os
from collections import deque
P=os.path.dirname(os.path.abspath(__file__))
G=json.load(open(P+'/goals.json'))
def bfs(rows, src, block=()):
    d=[[None]*12 for _ in range(12)]; q=deque()
    for (x,y) in src: d[y][x]=0; q.append((x,y))
    while q:
        x,y=q.popleft()
        for dx,dy in ((1,0),(-1,0),(0,1),(0,-1)):
            a,b=x+dx,y+dy
            if 0<=a<12 and 0<=b<12 and rows[b][a]!='#' and d[b][a] is None and (a,b) not in block:
                d[b][a]=d[y][x]+1; q.append((a,b))
    return d
def nearer(rows,da,db):
    n=0
    for y in range(12):
        for x in range(12):
            if rows[y][x]!='#' and da[y][x] is not None and (db[y][x] is None or da[y][x]<db[y][x]): n+=1
    return n
for g in G:
    rows=g['boards'][0]['rows']
    O=[(x,y) for y in range(12) for x in range(12) if rows[y][x]=='O']
    X=[(x,y) for y in range(12) for x in range(12) if rows[y][x]=='X']
    dO,dX=bfs(rows,O),bfs(rows,X)
    base=nearer(rows,dO,dX)
    # Each neutral cell valued by holding the shortest chain of cells from
    # our land to it: the share of the map nearer our land plus the chain
    # than the enemy's (the chain blocking it), less the share as it is.
    par={}
    q=deque(O); seen=set(O)
    while q:
        x,y=q.popleft()
        for dx,dy in ((1,0),(-1,0),(0,1),(0,-1)):
            a,b=x+dx,y+dy
            if 0<=a<12 and 0<=b<12 and rows[b][a]!='#' and (a,b) not in seen and rows[b][a]!='X':
                seen.add((a,b)); par[(a,b)]=(x,y); q.append((a,b))
    val={}
    for y in range(12):
        for x in range(12):
            if rows[y][x]!='.' or (x,y) not in par: continue
            path=[]; c=(x,y)
            while c in par and c not in O:
                path.append(c); c=par[c]
            da=bfs(rows,O+path); db=bfs(rows,X,block=set(path))
            gain=nearer(rows,da,db)-base
            open_=sum(1 for dx,dy in ((1,0),(-1,0),(0,1),(0,-1)) if 0<=x+dx<12 and 0<=y+dy<12 and rows[y+dy][x+dx]!='#')
            val[(x,y)]=dict(v=gain, steps=len(path), per=round(gain/max(1,len(path)),1), open=open_, us=dO[y][x], them=dX[y][x])
    g['cells']={f'{x},{y}':c for (x,y),c in val.items()}
    reg=g['region']
    for f in g['fronts']:
        line={tuple(c) for c in f['cells']}
        mine_=[(k,c) for k,c in val.items() if reg.get(f'{k[0]},{k[1]}')==f['lane']]
        ent=[(k,c) for k,c in mine_ if k in line]
        deep=[(k,c) for k,c in mine_ if k not in line and c['them'] is not None and c['us'] is not None and -8<=c['them']-c['us']<=-4]
        best=lambda L: max(L,key=lambda kc:(kc[1]['v'],-kc[1]['steps'])) if L else None
        
        e,d=best(ent),best(deep)
        f['entrance']=dict(cell=list(e[0]),**e[1]) if e else None
        f['deep']=dict(cell=list(d[0]),**d[1]) if d else None
        print(g['game_id'],f['name'],'base',base,'entrance',f['entrance'] and (f['entrance']['cell'],f['entrance']['v'],f['entrance']['steps']),'deep',f['deep'] and (f['deep']['cell'],f['deep']['v'],f['deep']['steps']))
json.dump(G,open(P+'/goals.json','w'),separators=(',',':'))
