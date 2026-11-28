#!/usr/bin/env python3
"""Fits the plan bot's evaluation weights jointly on game outcomes (E116).

Standard library and numpy only. Inputs are made by the plan bot's analysis
mode `terms` on a bank of positions:

  ice-and-fire-bot-plan terms --positions BANK > FILE.terms

  --cg-terms FILE     terms of our CodinGame plies (bank from
                      `ice-and-fire-review codingame-judge --bank`)
  --cg-index FILE     the data's index.jsonl; the label of a position is our
                      result in the game of its name: ranking[our_seat] == 0
  --leaderboard FILE  a leaderboard snapshot (data/ice-and-fire/leaderboard/*.json);
                      with --top N, `codingame-strong` keeps only games of
                      our releases v030 and later against the top N players
  --arena-terms FILE  terms of every ply of arena self-play games (bank from
                      `ice-and-fire-review bank --all-plies`)
  --arena-records DIR the arena's records; a position's name is a record's
                      file name without .json, its label the winner's seat

Model: P(the side to act wins) = sigmoid(k * sum_i w_i * term_i), the w_i
fractions of today's weights (1.0 = 100). Positions of game turn 7 and later,
not finished and without the `-WIN / 2` threat. Games (not positions) are
split 80/20 into train and held-out with a fixed seed.

Step 1 fits k alone with all w = 1. Step 2 fits all w with k fixed from step 1
and a pull towards 1.0, mean log-loss + lam * sum (w_i - 1)^2, lam chosen on
the held-out games. Prints a Markdown report.
"""

import argparse
import glob
import json
import os
import sys

import numpy as np

NAMES = ["gold", "income", "cells", "units", "towers", "pressure", "standing",
         "debt", "shape", "strong", "idle", "threatened", "mines", "wall",
         "goal", "voronoi", "flank"]
FROM_TURN = 7
SEED = 116
LAMBDAS = [0.0, 1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1]


def load(terms_path, labels):
    """Rows (game, X, y) of the positions that count; labels: name -> 0/1."""
    games, xs, ys = [], [], []
    left = {"early": 0, "finished": 0, "threat": 0, "unlabelled": 0, "kept": 0}
    with open(terms_path) as f:
        for line in f:
            d = json.loads(line)
            if d["turn"] < FROM_TURN:
                left["early"] += 1
                continue
            if d["finished"]:
                left["finished"] += 1
                continue
            if d["threat"]:
                left["threat"] += 1
                continue
            win = labels.get(d["name"])
            if win is None:
                left["unlabelled"] += 1
                continue
            # The label is a seat's result; the side to act is d["seat"].
            games.append(d["name"])
            xs.append([d["terms"][n] for n in NAMES])
            ys.append(win[d["seat"]])
            left["kept"] += 1
    return np.array(games), np.array(xs, dtype=float), np.array(ys, dtype=float), left


def cg_labels(index_path, strong=None):
    """Our results by game id; with `strong` (a set of nicknames), only the
    games of our releases v030 and later against one of them."""
    out = {}
    with open(index_path) as f:
        for line in f:
            d = json.loads(line)
            seat = d["our_seat"]
            rank = d["ranking"]
            if rank[0] == rank[1]:
                continue
            if strong is not None:
                rel = d.get("release") or ""
                if not (rel.startswith("ice-and-fire-v") and int(rel[14:]) >= 30):
                    continue
                if not any(p["name"] in strong for p in d["players"] if not p.get("me")):
                    continue
            res = [0.0, 0.0]
            res[seat] = 1.0 if rank[seat] == 0 else 0.0
            res[1 - seat] = 1.0 - res[seat]
            out[str(d["game_id"])] = res
    return out


def arena_labels(records_dir):
    out = {}
    for path in glob.glob(os.path.join(records_dir, "*.json")):
        with open(path) as f:
            d = json.load(f)
        w = d.get("winner")
        if w is None:
            continue
        res = [0.0, 0.0]
        res[w] = 1.0
        res[1 - w] = 0.0
        out[os.path.basename(path)[:-5]] = res
    return out


def split(games, rng_seed=SEED):
    """A boolean mask of the training positions: 80% of the games."""
    names = sorted(set(games))
    rng = np.random.default_rng(rng_seed)
    rng.shuffle(names)
    train = set(names[: int(0.8 * len(names))])
    return np.array([g in train for g in games])


def sigmoid(z):
    return 1.0 / (1.0 + np.exp(-np.clip(z, -35, 35)))


def logloss(z, y):
    # -log sigmoid(z) for wins, -log sigmoid(-z) for losses, stable.
    s = np.where(y > 0.5, z, -z)
    return float(np.mean(np.logaddexp(0.0, -s)))


def fit_k(x, y):
    """The scale k of sigmoid(k * sum(x)) alone, by Newton's method."""
    s = x.sum(axis=1)
    k = 0.01
    for _ in range(100):
        p = sigmoid(k * s)
        g = np.mean((p - y) * s)
        h = np.mean(p * (1 - p) * s * s) + 1e-12
        step = g / h
        k -= step
        if abs(step) < 1e-12:
            break
    return k


def fit_w(x, y, k, lam, iters=100):
    """All weights, k fixed: Newton's method on the mean log-loss plus
    lam * sum (w - 1)^2."""
    n, m = x.shape
    w = np.ones(m)
    xs = k * x
    for _ in range(iters):
        p = sigmoid(xs @ w)
        g = xs.T @ (p - y) / n + 2 * lam * (w - 1)
        h = (xs * (p * (1 - p))[:, None]).T @ xs / n + 2 * lam * np.eye(m)
        h += 1e-9 * np.eye(m)
        step = np.linalg.solve(h, g)
        w -= step
        if np.max(np.abs(step)) < 1e-10:
            break
    return w


def evaluate(name, x, y, train):
    xt, yt, xh, yh = x[train], y[train], x[~train], y[~train]
    k = fit_k(xt, yt)
    base = logloss(k * xh.sum(axis=1), yh)
    best = None
    rows = []
    for lam in LAMBDAS:
        w = fit_w(xt, yt, k, lam)
        loss = logloss(k * (xh @ w), yh)
        rows.append((lam, loss))
        if best is None or loss < best[1]:
            best = (lam, loss, w)
    lam, loss, w = best
    return {"name": name, "k": k, "base": base, "lam": lam, "loss": loss,
            "w": w, "grid": rows, "train": int(train.sum()),
            "held": int((~train).sum())}


def cross(r, x, y, train):
    """Held-out log-loss of the weights of fit `r` on another source: k is
    refitted on that source's training games, so only the weights differ."""
    xt, yt, xh, yh = x[train], y[train], x[~train], y[~train]
    kt = fit_k((xt @ r["w"])[:, None], yt)
    k0 = fit_k(xt, yt)
    return logloss(k0 * xh.sum(axis=1), yh), logloss(kt * (xh @ r["w"]), yh)


def pct(w):
    return [int(min(300, max(25, round(v * 100)))) for v in w]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cg-terms")
    ap.add_argument("--cg-index")
    ap.add_argument("--leaderboard")
    ap.add_argument("--top", type=int, default=10)
    ap.add_argument("--arena-terms")
    ap.add_argument("--arena-records")
    args = ap.parse_args()

    sources = {}
    if args.arena_terms:
        sources["arena"] = load(args.arena_terms, arena_labels(args.arena_records))
    if args.cg_terms:
        sources["codingame"] = load(args.cg_terms, cg_labels(args.cg_index))
        if args.leaderboard:
            with open(args.leaderboard) as f:
                rows = json.load(f)["rows"]
            strong = {r["nickname"] for r in rows if r["rank"] <= args.top and not r.get("me")}
            sources["codingame-strong"] = load(args.cg_terms, cg_labels(args.cg_index, strong))
            print(f"strong opponents (top {args.top}, newest snapshot): {sorted(strong)}\n")

    data = {}
    for name, (games, x, y, left) in sources.items():
        data[name] = (games, x, y, split(games))
        print(f"## {name}\n")
        print(f"{len(set(games))} games, {left['kept']} positions kept; left out: "
              f"{left['early']} before turn {FROM_TURN}, {left['finished']} finished, "
              f"{left['threat']} with the -WIN/2 threat, {left['unlabelled']} unlabelled "
              f"(drawn, unknown or filtered games)\n")
        print(f"mean outcome of the side to act: {y.mean():.3f}; "
              f"terms never nonzero: "
              f"{[n for i, n in enumerate(NAMES) if not np.any(x[:, i])]}\n")
    if "codingame" in data and "arena" in data:
        # Names never collide: CodinGame ids are numbers.
        parts = [data["codingame"], data["arena"]]
        data["both"] = tuple(np.concatenate([d[i] for d in parts]) for i in range(4))

    results = {name: evaluate(name, d[1], d[2], d[3]) for name, d in data.items()}
    for name, d in data.items():
        results[name]["games"] = len(set(d[0]))
    print("## Held-out log-loss, each fit on its own source\n")
    print("| fit on | games | train / held-out positions | k | today's | fitted | lambda |")
    print("|---|---|---|---|---|---|---|")
    for r in results.values():
        print(f"| {r['name']} | {r['games']} | {r['train']} / {r['held']} | {r['k']:.5f} | "
              f"{r['base']:.5f} | {r['loss']:.5f} | {r['lam']:g} |")
    print()
    print("## Each fit on each source's held-out games (k refitted on the source's train)\n")
    print("| fit | held-out source | today's | the fit's weights |")
    print("|---|---|---|---|")
    for fit, r in results.items():
        for src, d in data.items():
            if src == "both":
                continue
            base, got = cross(r, d[1], d[2], d[3])
            print(f"| {fit} | {src} | {base:.5f} | {got:.5f} |")
    print()
    print("## Multipliers (percent, rounded, clamped to 25 to 300; unclamped in brackets)\n")
    cols = list(results)
    print("| term | " + " | ".join(cols) + " |")
    print("|---|" + "---|" * len(cols))
    for i, n in enumerate(NAMES):
        cells = [f"{pct(results[c]['w'])[i]} ({results[c]['w'][i] * 100:.0f})" for c in cols]
        print(f"| {n} | " + " | ".join(cells) + " |")
    print()
    print("## Lambda grid (held-out log-loss)\n")
    print("| lambda | " + " | ".join(cols) + " |")
    print("|---|" + "---|" * len(cols))
    for j, lam in enumerate(LAMBDAS):
        print(f"| {lam:g} | " + " | ".join(f"{results[c]['grid'][j][1]:.5f}" for c in cols) + " |")
    print()
    for c in cols:
        print(f"WEIGHTS {c}: {pct(results[c]['w'])}")


if __name__ == "__main__":
    sys.exit(main())
