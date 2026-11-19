# The search's surprises, release by release

A check run on each release's CodinGame games (asked by the owner on 2026-11-14, after [2026-11-14-overconfidence.md](2026-11-14-overconfidence.md), whose sections 3 and 3d define the two measures): does a new release leave fewer positions in which the opponent's reply costs a lot?

Each row is the section "The release's check" of `ice-and-fire-review codingame-surprise` on the release's collected games (the branch `claude/codingame-games`; the steps are in the skill `deliver-release`), judged by the plan bot's search at 600 tries:

- a **large miss** is one of our plies whose real reply left 50 or more below the worst reply the search modeled (shares of the *normal* plies, those whose values hold no threat on an HQ);
- a **foreseen large removal** is a normal ply whose reply removed 10 or more of our cells while the search expected about that much (a surprise above −20): the price of a weak position built earlier;
- an **HQ threat not foreseen** is a ply after which the opponent can pay a chain to our HQ, or took it, when the search expected neither (share of all plies).

The opponents differ from one release to the next (a release's ranking games, then the collection's test games against the top players), and so does the share of games lost: read a row beside its *Lost* column.

| Release | Games | Lost | Plies | Normal plies | Large misses | Foreseen large removals | HQ threats not foreseen | Data | Judge |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| ice-and-fire-v023 | 306 | 51% | 4053 | 3514 | 63 (1.8%) | 102 (2.9%) | 19 (0.5%) | 0992ae7 | v026 (main at 6a22ae3) |
| ice-and-fire-v024 | 131 | 60% | 1877 | 1605 | 29 (1.8%) | 51 (3.2%) | 17 (0.9%) | 0992ae7 | v026 (main at 6a22ae3) |
| ice-and-fire-v025 | 157 | 47% | 2015 | 1760 | 41 (2.3%) | 64 (3.6%) | 13 (0.6%) | 0992ae7 | v026 (main at 6a22ae3) |
| ice-and-fire-v026 | 220 | 51% | 2992 | 2636 | 50 (1.9%) | 62 (2.4%) | 7 (0.2%) | 0992ae7 | v026 (main at 6a22ae3) |
| ice-and-fire-v028 | 158 | 38% | 2184 | 1982 | 48 (2.4%) | 66 (3.3%) | 16 (0.7%) | e3b20fa | v028 (its tag) |

*Games* are those with a judged ply (a game that ends before the middle game has none). *Data* is the commit of the data branch read; *Judge* the release whose search judged the plies: these four rows by main's build at 6a22ae3, whose search is v026's (the bot's mode `surprise` came after the tag `ice-and-fire-v026`, and v023 to v025 were judged by v026's search in the note too); from v027 on, each release by the build of its own tag.

v028's row reads its 158 ranking games only (1 left out as unusable): the collection had played no test games with it yet. It lost fewer of them (38%) than the earlier releases lost of theirs, with about as many large misses and foreseen large removals as v025, and more HQ threats not foreseen than v026 (0.7% against 0.2%, 16 plies).
