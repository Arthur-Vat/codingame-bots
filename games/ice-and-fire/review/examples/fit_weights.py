"""Fits the plan bot's evaluation weights to self-play outcomes (E017).

Usage: python3 fit.py features.csv
"""
import csv
import sys

import numpy as np

rows = list(csv.DictReader(open(sys.argv[1])))
g = lambda k: np.array([float(r[k]) for r in rows])

# The terms, each as the evaluation uses them, from the side to act.
terms = {
    "gold": g("own_gold") - g("opp_gold"),
    "income": g("own_income") - g("opp_income"),
    "cells": g("own_cells") - g("opp_cells"),
    "units": g("own_units") - g("opp_units"),
    "towers": g("own_towers") - g("opp_towers"),
    "mines": g("own_mines") - g("opp_mines"),
    "own_pressure": g("own_pressure"),
    "opp_pressure": g("opp_pressure"),
    "own_debt": g("own_debt"),
    "opp_debt": g("opp_debt"),
    "joints": g("own_joints") - g("opp_joints"),
    "cut": g("own_cut") - g("opp_cut"),
    "threat": g("threat"),
}
# The evaluation today, in gold (threat as WIN / 2 is left out: it decides).
current = {
    "gold": 1, "income": 3, "cells": 4, "units": 1, "towers": 10, "mines": 0,
    "own_pressure": 1, "opp_pressure": -1, "own_debt": -2, "opp_debt": 2,
    "joints": -2, "cut": -2, "threat": 0,
}
import os
drop = set(filter(None, os.environ.get("DROP", "").split(",")))
names = [n for n in terms if n not in drop]
X = np.column_stack([terms[n] for n in names])
blue = g("blue")
y = g("won")
game = g("game").astype(int)
turn = g("game_turn")

# Hold out a fifth of the games.
test = game % 5 == 0
train = ~test


def fit(X, y, l2=1e-3, iters=50):
    """Logistic regression by Newton's method, with a small L2 penalty."""
    w = np.zeros(X.shape[1])
    for _ in range(iters):
        p = 1 / (1 + np.exp(-(X @ w)))
        grad = X.T @ (p - y) / len(y) + l2 * w
        hess = (X.T * (p * (1 - p))) @ X / len(y) + l2 * np.eye(len(w))
        step = np.linalg.solve(hess, grad)
        w -= step
        if np.abs(step).max() < 1e-10:
            break
    return w


def logloss(z, y):
    p = np.clip(1 / (1 + np.exp(-z)), 1e-12, 1 - 1e-12)
    return float(-np.mean(y * np.log(p) + (1 - y) * np.log(1 - p)))


def accuracy(z, y):
    return float(np.mean((z > 0) == (y > 0.5)))


# Standardize for the fit, then express the weights in the features' units.
scale = X[train].std(axis=0)
scale[scale == 0] = 1
Xs = X / scale
design = lambda Xm, b: np.column_stack([Xm, b, np.ones(len(b))])
w = fit(design(Xs[train], blue[train]), y[train])
w_terms = w[: len(names)] / scale
z_fit = design(Xs, blue) @ w

# The current evaluation, scaled by the best factor (with the same seat and
# intercept terms), for a fair comparison.
e_cur = X @ np.array([current[n] for n in names], dtype=float)
w_cur = fit(np.column_stack([e_cur / e_cur[train].std(), blue, np.ones(len(y))])[train], y[train])
z_cur = np.column_stack([e_cur / e_cur[train].std(), blue, np.ones(len(y))]) @ w_cur

print(f"positions {len(y)} (train {train.sum()}, test {test.sum()}), games {len(set(game))}")
zt = design(Xs, blue) @ w
print(f"train log loss: fitted {logloss(zt[train], y[train]):.4f}, current {logloss(z_cur[train], y[train]):.4f}")
print(f"held-out log loss: fitted {logloss(z_fit[test], y[test]):.4f}, current {logloss(z_cur[test], y[test]):.4f}")
print(f"held-out accuracy: fitted {accuracy(z_fit[test], y[test]):.3f}, current {accuracy(z_cur[test], y[test]):.3f}")
gold = w_terms[names.index("gold")]
print()
print(f"{'term':14} {'fitted (gold=1)':>16} {'current':>8}")
for n, v in zip(names, w_terms):
    print(f"{n:14} {v / gold:16.2f} {current[n]:8}")
print(f"seat (blue)     logit {w[len(names)]:.3f}")

# By phase: fitted on each third of the middle game's turns.
for lo, hi in [(0, 12), (12, 18), (18, 200)]:
    m = train & (turn >= lo) & (turn < hi)
    if m.sum() < 500:
        continue
    wp = fit(design(Xs[m], blue[m]), y[m])[: len(names)] / scale
    gp = wp[names.index("gold")]
    print(f"turns {lo}-{hi} ({m.sum()} positions): " + ", ".join(f"{n} {v / gp:.1f}" for n, v in zip(names, wp)))
