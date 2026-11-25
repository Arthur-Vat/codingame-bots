"""Per front: the entrances on our side, a minimum vertex cut between the
enemy's land and the region's backfield, where only neutral cells we reach
no later than the enemy (red: at most one step later, the seat's own step)
can be cut."""
import json, os
from collections import deque
P=os.path.dirname(os.path.abspath(__file__))
G=json.load(open(P+'/goals.json'))
INF=10**9
ONLY=None
def nb(x,y,rows):
    for dx,dy in ((1,0),(-1,0),(0,1),(0,-1)):
        a,b=x+dx,y+dy
        if 0<=a<12 and 0<=b<12 and rows[b][a]!='#' and (ONLY is None or (a,b) in ONLY): yield a,b
def maxflow_cut(rows, sources, sinks, cuttable):
    # Node-split graph: in = 2*i, out = 2*i+1; edge in->out cap 1 if cuttable else INF.
    idx=lambda x,y: y*12+x
    cap={}
    adj={}
    def add(u,v,c):
        cap[(u,v)]=cap.get((u,v),0)+c; cap.setdefault((v,u),0)
        adj.setdefault(u,set()).add(v); adj.setdefault(v,set()).add(u)
    S_,T_=288,289
    for y in range(12):
        for x in range(12):
            if rows[y][x]=='#' or (ONLY is not None and (x,y) not in ONLY): continue
            i=idx(x,y)
            add(2*i,2*i+1,1 if (x,y) in cuttable else INF)
            for a,b in nb(x,y,rows):
                add(2*i+1,2*idx(a,b),INF)
    for (x,y) in sources: add(S_,2*idx(x,y),INF)
    for (x,y) in sinks: add(2*idx(x,y)+1,T_,INF)
    flow=0
    while True:
        par={S_:None}; q=deque([S_])
        while q and T_ not in par:
            u=q.popleft()
            for v in adj.get(u,()):
                if v not in par and cap[(u,v)]>0: par[v]=u; q.append(v)
        if T_ not in par: break
        f=INF; v=T_
        while par[v] is not None: f=min(f,cap[(par[v],v)]); v=par[v]
        v=T_
        while par[v] is not None: cap[(par[v],v)]-=f; cap[(v,par[v])]+=f; v=par[v]
        flow+=f
        if flow>=INF: return None
    # cut: cuttable cells whose in-node is reachable and out-node is not
    seen={S_}; q=deque([S_])
    while q:
        u=q.popleft()
        for v in adj.get(u,()):
            if v not in seen and cap[(u,v)]>0: seen.add(v); q.append(v)
    cut=[(x,y) for (x,y) in cuttable if 2*idx(x,y) in seen and 2*idx(x,y)+1 not in seen]
    return cut
def run():
    global ONLY
    for g in G:
        rows=g['boards'][0]['rows']
        cells=g['cells']; reg=g['region']
        ours_side={tuple(map(int,k.split(','))) for k,c in cells.items() if c['us'] is not None and c['them'] is not None and c['us']<=c['them']+1}
        get=lambda c,k: cells.get(f'{c[0]},{c[1]}',{}).get(k)
        for f in g['fronts']:
            R={tuple(map(int,k.split(','))) for k,l in reg.items() if l==f['lane']}
            back=[c for c in R if c in ours_side and get(c,'them') is not None and get(c,'them')-get(c,'us')>=3]
            front_side=[c for c in R if get(c,'us') is not None and get(c,'us')-get(c,'them')>=3]
            ONLY=R
            cut=maxflow_cut(rows,front_side,back,ours_side) if back and front_side else None
            ONLY=None
            f['entrances']=[list(c) for c in cut] if cut else []
            f['sealable']=bool(cut)
            f['backfield']=len(back)
            f['entrance_steps']=[get(c,'steps') for c in (cut or [])]
            print(g['game_id'],f['name'],'backfield',len(back),'entrances',cut and sorted(cut),'steps',f['entrance_steps'])
run()
json.dump(G,open(P+'/goals.json','w'),separators=(',',':'))
